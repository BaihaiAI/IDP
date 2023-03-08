// Copyright 2022 BaihaiAI, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::routing::on;
use axum::routing::post;
use axum::routing::MethodFilter;
use axum::Json;
use axum::Router;
use cache_io::CacheService;
use cache_io::CloneState;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;
use tokio::process::Command;
use tracing::info;

use crate::api_model::environment::*;
use crate::api_model::TeamIdProjectIdQueryString;
use crate::api_model::TeamIdQueryString;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler;

pub fn environment_apis_route(ctx: AppContext) -> Router {
    Router::new()
        .route("/list", get(conda_env_list))
        .route("/clone", post(clone))
        .route("/clone/state", get(clone_state))
        .route("/current", get(current_env))
        .route("/switch", on(MethodFilter::PUT, switch_environment))
        .with_state(ctx)
}

/// return process_id of `conda create`
pub async fn clone(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    State(app_context): State<AppContext>,
    Json(payload): Json<EnvClone>,
) -> Result<Rsp<String>, ErrorTrace> {
    info!("access conda env clone api.");
    let team_id = get_cookie_value_by_team_id(cookies);
    if payload.origin_name.is_empty() || payload.target_name.is_empty() {
        return Err(ErrorTrace::new("origin_name or target_name is error"));
    }

    handler::environment::clone_(
        team_id,
        payload.origin_name,
        payload.target_name,
        &app_context.redis_cache,
    )
    .await
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CloneStateReq {
    clone_state_key: String,
}
pub async fn clone_state(
    Query(clone_state_req): Query<CloneStateReq>,
    State(app_context): State<AppContext>,
) -> Result<Rsp<Option<String>>, IdpGlobalError> {
    //get clone state form redis
    Ok(Rsp::success(
        app_context
            .redis_cache
            .get_clone_state(&clone_state_req.clone_state_key)
            .await?,
    ))
}
pub async fn conda_env_list(
    Query(TeamIdQueryString { team_id }): Query<TeamIdQueryString>,
) -> Result<Rsp<Vec<String>>, IdpGlobalError> {
    let mut env_list = Vec::new();
    let conda_root = business::path_tool::conda_root(team_id);
    for file in std::fs::read_dir(format!("{conda_root}/envs"))?.flatten() {
        let filename = file.file_name();
        let filename = filename.to_str().unwrap();
        // skip hidden folder/file
        if filename.starts_with('.') {
            continue;
        }
        if !file.path().join("bin").join("python").exists() {
            tracing::error!("env no bin/python: /store/{team_id}/miniconda3/envs/{filename}");
            continue;
        }
        env_list.push(filename.to_string());
    }
    Ok(Rsp::success(env_list))
}

#[allow(clippy::unused_async)]
pub async fn current_env(
    Query(TeamIdProjectIdQueryString {
        team_id,
        project_id,
    }): Query<TeamIdProjectIdQueryString>,
) -> Rsp<String> {
    Rsp::success(business::path_tool::project_conda_env(team_id, project_id))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchEnvReq {
    pub team_id: u64,
    pub project_id: u64,
    pub environment_name: String,
    pub compute_type: String,
}
pub async fn switch_environment(
    Query(SwitchEnvReq {
        team_id,
        project_id,
        environment_name,
        compute_type,
    }): Query<SwitchEnvReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let target_env = environment_name;
    info!("switch_environment to {target_env}");

    // close all kernels in this project.
    handler::kernel::shutdown_by_project_id_and_kernel_idpnb_starts_with_path(project_id, "")
        .await?;

    // get current env to compare
    let origin_env = business::path_tool::project_conda_env(team_id, project_id);
    if origin_env == target_env {
        return Ok(Rsp::success(()));
    }
    let python_path = business::path_tool::get_conda_env_python_path(team_id, target_env.clone());
    if !std::path::Path::new(&python_path).exists() {
        return Err(
            ErrorTrace::new(&format!("{target_env} is broken")).code(ErrorTrace::CODE_WARNING)
        );
    }

    tracing::info!("switch_environment: {origin_env} -> {target_env}");
    std::fs::write(
        crate::business_::path_tool::project_conda_env_file_path(team_id, project_id),
        target_env,
    )?;

    // if need switch between cpu and gpu env ,modify region
    if let Err(err) = send_change_compute_type_to_resource_(project_id, &compute_type).await {
        tracing::warn!("{err}");
    };
    Ok(Rsp::success(()))
}
#[inline]
async fn send_change_compute_type_to_resource_(
    project_id: u64,
    compute_type: &str,
) -> Result<(), err::ErrorTrace> {
    let client = reqwest::Client::new();
    let _res = client
        .post("http://idp-resource-svc:10005/api/v1/project/update")
        .json(&serde_json::json!({
            "id": project_id,
            "computeType": compute_type
        }))
        .send()
        .await?;
    Ok(())
}

pub async fn clone_(
    team_id: u64,
    origin_name: String,
    target_name: String,
    redis_cache: &CacheService,
) -> Result<Rsp<String>, ErrorTrace> {
    let conda_root = business::path_tool::conda_root(team_id);

    let conda_path = format!("{}/bin/conda", conda_root);
    // clean conda cache first to prevent CondaVerificationError

    let timestamp = chrono::Local::now().timestamp();
    let clone_state_key = format!("conda_clone_{team_id}_{origin_name}_{target_name}_{timestamp}");
    let clone_state_key_ = clone_state_key.clone();
    let svc = redis_cache.clone();
    tokio::spawn(async move {
        if let Err(err) = clone_state_monitor(
            &svc,
            clone_state_key.clone(),
            conda_path,
            origin_name,
            target_name,
        )
        .await
        {
            tracing::error!("conda clone {err:#?}");
            if let Err(err) = svc
                .set_clone_state(&clone_state_key, CloneState::Failed)
                .await
            {
                tracing::error!("{err}");
            }
        }
    });
    Ok(Rsp::success(clone_state_key_))
}

async fn clone_state_monitor(
    cache_service: &CacheService,
    clone_state_key: String,
    conda_path: String,
    origin_name: String,
    target_name: String,
) -> Result<(), ErrorTrace> {
    // firstly set clone state as cloning.
    cache_service
        .set_clone_state(&clone_state_key, CloneState::Cloning)
        .await?;

    let mut cmd = Command::new(&conda_path);
    cmd.arg("clean").arg("--all").arg("-y");
    info!("cmd = {cmd:?}");
    let output = cmd.spawn()?.wait().await?;
    if !output.success() {
        return Err(ErrorTrace::new("conda clean fail"));
    }

    let mut cmd = Command::new(&conda_path);
    cmd.arg("create")
        .args(["-y", "-n"])
        .arg(target_name)
        .arg("--clone")
        .arg(origin_name);
    info!("cmd = {cmd:?}");
    let mut child = cmd.spawn()?;

    let exit_status = child.wait().await?;
    if exit_status.success() {
        cache_service
            .set_clone_state(&clone_state_key, CloneState::Success)
            .await?;
        Ok(())
    } else {
        Err(ErrorTrace::new("conda clone exit code non zero"))
    }
}
