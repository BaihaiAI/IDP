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
use axum::response::IntoResponse;
use axum::Json;
use business::path_tool;
use common_model::entity::cell::Cell;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;
use tracing::debug;
use tracing::info;

use crate::api_model::workspace::DirFullLoadPara;
use crate::api_model::workspace::DirLazyLoadPara;
use crate::api_model::workspace::DirSearchPara;
use crate::api_model::workspace::FileRenameReq;
use crate::api_model::workspace::FileTreeNode;
use crate::api_model::workspace::FullFileTreeNode;
use crate::api_model::workspace::GlobalKeywordSearchPara;
use crate::api_model::workspace::GlobalSearchResult;
use crate::api_model::workspace::KeywordSearchResult;
use crate::api_model::workspace::ModelExportReq;
use crate::api_model::workspace::ModelName;
use crate::api_model::workspace::ModelUploadReq;
use crate::api_model::workspace::WorkspaceFile;
use crate::api_model::workspace::WorkspaceMove;
use crate::api_model::TeamIdQueryString;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler::workspace as workspace_handler;
use crate::handler::workspace::is_valid_to_create;

#[derive(serde::Deserialize)]
#[cfg_attr(test, derive(serde::Serialize, Debug))]
#[serde(rename_all = "camelCase")]
pub struct NewFileReq {
    pub path: String,
    pub project_id: u64,
}

pub async fn new_file(
    Query(TeamIdQueryString { team_id }): Query<TeamIdQueryString>,
    Json(NewFileReq { path, project_id }): Json<NewFileReq>,
) -> Result<Rsp<()>, err::ErrorTrace> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    info!("new_file: path={path:?}");

    if path.exists() {
        return Err(ErrorTrace::new("file already exists."));
    }

    if !is_valid_to_create(path.as_path().to_str().unwrap()) {
        return Err(ErrorTrace::new("name contains invalid format: _()"));
    }

    let path = path.to_str().unwrap();
    if path.ends_with(".ipynb") || path.ends_with(".idpnb") {
        let notebook = common_model::entity::notebook::Notebook::new(vec![Cell::default()]);
        debug_assert_eq!(notebook.cells.len(), 1);
        common_tools::file_tool::write_notebook_to_disk(&path, &notebook).await?;
    } else {
        tokio::fs::File::create(&path).await?;
    }
    Ok(Rsp::success(()))
}

/// FIXME url maybe not found
#[test]
fn test_new_file_it() {
    let ctx = test_runner::IntegrationTestCtx::get();
    let resp = ctx
        .client
        .post("http://127.0.0.1:3003/a/api/v2/idp-note-rs/workspace/file?teamId=1")
        .json(&NewFileReq {
            path: "demo2.ipynb".to_string(),
            project_id: ctx.project_id,
        })
        .send()
        .unwrap();
    dbg!(resp.status());
    dbg!(resp.text().unwrap());
}

pub async fn file_rename(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(file_rename_req): Json<FileRenameReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access file_rename api");
    debug!("file_rename_req => {:?}", file_rename_req);
    let team_id = get_cookie_value_by_team_id(cookies);

    workspace_handler::file_rename(
        file_rename_req.path,
        file_rename_req.source,
        file_rename_req.dest,
        team_id,
        file_rename_req.project_id,
        file_rename_req.auto_close,
    )
    .await
}

pub async fn file_dir_move(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(workspace_move_req): Json<WorkspaceMove>,
    Extension(mut app_context): Extension<AppContext>,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access file_dir_move api");
    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::file_dir_move(
        workspace_move_req.origin_path,
        workspace_move_req.target_path,
        team_id,
        workspace_move_req.project_id,
        workspace_move_req.auto_close,
        &mut app_context.redis_cache,
    )
    .await
}

pub async fn file_dir_copy(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<WorkspaceMove>,
) -> Result<Rsp<()>, ErrorTrace> {
    info!("access file_dir_cp api");
    let (origin_path, target_path, project_id) =
        (payload.origin_path, payload.target_path, payload.project_id);

    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::file_dir_copy(origin_path, target_path, team_id, project_id).await
}
pub async fn model_export(
    Json(payload): Json<ModelExportReq>,
) -> Result<Rsp<ModelName>, ErrorTrace> {
    let team_id = payload.team_id.parse::<u64>().unwrap_or(0);
    let user_id = payload.user_id.parse::<u64>().unwrap_or(0);
    let base_path = path_tool::get_store_path(
        team_id,
        payload.project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    workspace_handler::model_export(
        payload.path,
        base_path,
        team_id,
        user_id,
        payload.project_id,
    )
    .await
}

pub async fn model_export_dir(
    Json(payload): Json<ModelExportReq>,
) -> Result<Rsp<ModelName>, ErrorTrace> {
    let team_id = payload.team_id.parse::<u64>().unwrap_or(0);
    let user_id = payload.user_id.parse::<u64>().unwrap_or(0);
    workspace_handler::model_export_dir(payload.path, team_id, user_id, payload.project_id).await
}

pub async fn model_upload(Json(payload): Json<ModelUploadReq>) -> Result<Rsp<()>, ErrorTrace> {
    tracing::debug!("access workspace model_upload api");

    let (path, team_id, user_id, project_id, model_name, version, intro) = (
        payload.path,
        payload.team_id,
        payload.user_id,
        payload.project_id,
        payload.model_name,
        payload.version,
        payload.intro,
    );

    let mut team_id_u64 = 0u64;
    let mut user_id_u64 = 0u64;

    if let Ok(t) = team_id.parse::<u64>() {
        team_id_u64 = t;
    }
    if let Ok(t) = user_id.parse::<u64>() {
        user_id_u64 = t;
    }

    workspace_handler::model_upload(
        path,
        team_id_u64,
        user_id_u64,
        project_id,
        model_name,
        version,
        intro,
    )
    .await

    // Ok(Res::success_without_data())
}

pub async fn dir_lazy_load(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<DirLazyLoadPara>,
) -> Result<Rsp<FileTreeNode>, IdpGlobalError> {
    info!("access dir_lazy_load api");

    let (project_id, _team_id, path, only_pipeline_support) = (
        payload.project_id,
        payload.team_id,
        payload.path,
        payload.only_pipeline_support,
    );

    let team_id = get_cookie_value_by_team_id(cookies);
    let region = "x".to_string();

    workspace_handler::dir_lazy_load(team_id, project_id, region, path, only_pipeline_support).await
}

pub async fn dir_search(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<DirSearchPara>,
) -> Result<Rsp<FileTreeNode>, IdpGlobalError> {
    info!("access dir_search api");

    let (project_id, keyword, only_pipeline_support) = (
        payload.project_id,
        payload.keyword,
        payload.only_pipeline_support,
    );

    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::dir_search(team_id, project_id, keyword, only_pipeline_support).await
    // if business::kubernetes::is_k8s() {
    //     workspace_kubernetes::dir_search(team_id, project_id, keyword, only_pipeline_support).await
    // } else {
    //     workspace_handler::dir_search(team_id, project_id, keyword, only_pipeline_support).await
    // }
}

pub async fn keyword_search(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<GlobalKeywordSearchPara>,
) -> Result<Rsp<Vec<KeywordSearchResult>>, IdpGlobalError> {
    info!("access keyword_search api");

    info!("access dir_search api");

    let (project_id, keyword) = (payload.project_id, payload.keyword);

    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::keyword_search(team_id, project_id, keyword).await
}

pub async fn global_keyword_search(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<GlobalKeywordSearchPara>,
) -> Result<Rsp<Vec<GlobalSearchResult>>, IdpGlobalError> {
    info!("access dir_search api");

    let (project_id, keyword) = (payload.project_id, payload.keyword);

    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::global_keyword_search(team_id, project_id, keyword).await
    // if business::kubernetes::is_k8s() {
    //     workspace_kubernetes::global_keyword_search(team_id, project_id, keyword).await
    // } else {
    //     workspace_handler::global_keyword_search(team_id, project_id, keyword).await
    // }
}

pub async fn global_keyword_search_dir_file(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<GlobalKeywordSearchPara>,
) -> Result<Rsp<Vec<GlobalSearchResult>>, IdpGlobalError> {
    info!("access dir_search api");

    let (project_id, keyword) = (payload.project_id, payload.keyword);

    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::global_keyword_search_dir_file(team_id, project_id, keyword).await
}

#[axum_macros::debug_handler]
pub async fn dir_new(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<WorkspaceFile>,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access dir_new api");
    let (path, project_id) = (payload.path, payload.project_id);

    // let team_id = 0u64;
    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::dir_new(path, team_id, project_id).await
}

pub async fn export_as(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(payload): Query<WorkspaceFile>,
    Extension(app_context): Extension<AppContext>,
) -> impl IntoResponse {
    info!("access export_as api");
    let (path, output_type_op, project_id) =
        (payload.path, payload.output_type, payload.project_id);

    let output_type = output_type_op.unwrap_or_default();
    // let team_id = 0u64;
    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::export_as(
        path,
        output_type,
        team_id,
        project_id,
        &app_context.redis_cache,
    )
    .await
}

pub async fn convert_to(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(payload): Query<WorkspaceFile>,
    Extension(app_context): Extension<AppContext>,
) -> impl IntoResponse {
    info!("access export_as api");
    let (path, output_type_op, project_id) =
        (payload.path, payload.output_type, payload.project_id);

    let output_type = output_type_op.unwrap();
    // let team_id = 0u64;
    let team_id = get_cookie_value_by_team_id(cookies);
    workspace_handler::convert_to(
        path,
        output_type,
        team_id,
        project_id,
        &app_context.redis_cache,
    )
    .await
}

/// used for pipeline browse file list
pub async fn dir_recursive_load(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<DirFullLoadPara>,
) -> Result<Rsp<FullFileTreeNode>, IdpGlobalError> {
    info!("access dir_lazy_load api");
    let (project_id, _team_id, path, _only_pipeline_support) = (
        payload.project_id,
        payload.team_id,
        payload.path,
        payload.only_pipeline_support,
    );

    let team_id = get_cookie_value_by_team_id(cookies);
    // if business::kubernetes::is_k8s() {
    //     workspace_kubernetes::dir_recursive_load(team_id, project_id, path, _only_pipeline_support)
    //         .await
    // } else {
    //     workspace_handler::dir_recursive_load(team_id, project_id, path, _only_pipeline_support)
    //         .await
    // }
    workspace_handler::dir_recursive_load(team_id, project_id, path, _only_pipeline_support).await
}
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct IdpExampleReq {
    pub project_id: u64,
    pub team_id: String,
    pub version: String,
}

pub async fn example_project(Json(payload): Json<IdpExampleReq>) -> Result<Rsp<()>, ErrorTrace> {
    info!("access example_project api");
    let project_id = payload.project_id;
    let team_id = payload.team_id.parse::<u64>().unwrap();
    let version = payload.version;
    workspace_handler::example_project::example_project(team_id, project_id, &version).await
}

#[cfg(test)]
#[test]
#[ignore = "require admin kubernetes micro service"]
fn test_example_project() {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    client
        .post("http://127.0.0.1:8082/api/v2/idp-note-rs/workspace/file/example")
        .json(&IdpExampleReq {
            project_id: 109,
            team_id: "1546774368495616000".to_string(),
            version: "".to_string(),
        })
        .send()
        .unwrap();
}
