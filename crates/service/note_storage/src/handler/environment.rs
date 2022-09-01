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

use axum::extract::Extension;
use axum::extract::Query;
use axum::Json;
use cache_io::CacheService;
use cache_io::CloneState;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;
use tokio::process::Child;
use tokio::process::Command;
use tower_cookies::Cookies;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::api_model::environment::*;
use crate::api_model::TeamIdProjectIdQueryString;
use crate::api_model::TeamIdQueryString;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler;

/// return process_id of `conda create`
pub async fn clone(
    cookies: Cookies,
    Json(payload): Json<EnvClone>,
    Extension(app_context): Extension<AppContext>,
) -> Result<Rsp<String>, IdpGlobalError> {
    info!("access conda env clone api.");
    let team_id = get_cookie_value_by_team_id(cookies);
    if payload.origin_name.is_empty() || payload.target_name.is_empty() {
        return Err(IdpGlobalError::NoteError(
            "InvalidRequestParamError".to_string(),
        ));
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
    Extension(mut app_context): Extension<AppContext>,
) -> Result<Rsp<Option<String>>, IdpGlobalError> {
    info!("access query clone state api.");

    clone_state_(
        clone_state_req.clone_state_key,
        &mut app_context.redis_cache,
    )
    .await
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
    handler::kernel::shutdown_by_dir_path(project_id, "".to_string()).await?;

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

#[instrument(skip(redis_cache))]
pub async fn clone_(
    team_id: u64,
    origin_name: String,
    target_name: String,
    redis_cache: &CacheService,
) -> Result<Rsp<String>, IdpGlobalError> {
    use std::process::Stdio;
    let conda_root = business::path_tool::conda_root(team_id);

    let conda_path = format!("{}/bin/conda", conda_root);
    let mut cmd = Command::new(conda_path);
    cmd.arg("create")
        .args(["-y", "-n"])
        .arg(target_name)
        .arg("--clone")
        .arg(origin_name)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    info!("conda clone command {cmd:#?}");
    let child = cmd.spawn()?;
    let timestamp = chrono::Local::now().timestamp();
    let clone_state_key = format!("{}_{}", timestamp, child.id().unwrap_or(923));
    tokio::spawn(clone_state_monitor(
        child,
        redis_cache.clone(),
        clone_state_key.clone(),
    ));
    Ok(Rsp::success(clone_state_key))
}
async fn clone_state_monitor(
    mut child: Child,
    cache_service: CacheService,
    clone_state_key: String,
) {
    info!("fork child process finished,pid:{:#?}", child.id());

    // firstly set clone state as cloning.
    if let Err(err) = cache_service
        .set_clone_state(&clone_state_key, CloneState::Cloning)
        .await
    {
        error!("{err}");
    }

    match child.wait().await {
        Ok(status) => {
            if status.success() {
                info!("clone success");
                //set clone state as success.
                if let Err(err) = cache_service
                    .set_clone_state(&clone_state_key, CloneState::Success)
                    .await
                {
                    error!("{err}");
                }
            } else {
                //set clone state as failed.
                error!("clone exit with status:{:?}", status);
                if let Err(err) = cache_service
                    .set_clone_state(&clone_state_key, CloneState::Failed)
                    .await
                {
                    error!("{err}");
                }
            }
        }
        Err(err) => {
            error!("wait clone error {:?}", err);
            //set clone state as failed.
            if let Err(err) = cache_service
                .set_clone_state(&clone_state_key, CloneState::Failed)
                .await
            {
                error!("{err}");
            }
        }
    }
}

#[instrument(skip(redis_cache))]
pub async fn clone_state_(
    clone_state_key: String,
    redis_cache: &mut CacheService,
) -> Result<Rsp<Option<String>>, IdpGlobalError> {
    info!("query clone state...");
    //get clone state form redis
    Ok(Rsp::success(
        redis_cache.get_clone_state(&clone_state_key).await?,
    ))
}
