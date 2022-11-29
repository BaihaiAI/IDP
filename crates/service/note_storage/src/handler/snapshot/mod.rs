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

pub mod diff;
pub mod diff_models;
pub mod handlers;
pub mod models;
use axum::extract::Extension;
use axum::Json;
use business::path_tool;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;

use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;

pub async fn post_snapshot(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Extension(app_context): Extension<AppContext>,
    Json(req): Json<models::SnapshotReq>,
) -> Result<Rsp<models::SnapshotRes>, IdpGlobalError> {
    // let team_id = 0u64; //todo
    let team_id = get_cookie_value_by_team_id(cookies);
    let file_full_path = path_tool::get_store_full_path(team_id, req.project_id, req.path.clone());

    let res = handlers::snapshot(
        &req.label,
        &file_full_path,
        &app_context.redis_cache,
        req.project_id,
    )
    .await?;
    Ok(Rsp::success(models::SnapshotRes { snapshots: res }))
}

pub async fn post_snapshot_list(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Extension(app_context): Extension<AppContext>,
    Json(req): Json<models::SnapshotListReq>,
) -> Result<Rsp<models::SnapshotListRes>, IdpGlobalError> {
    let team_id = get_cookie_value_by_team_id(cookies);
    let project_id = req.project_id;
    let path = &req.path[0];
    let path = path_tool::get_store_full_path(team_id, project_id, path);
    let path = path.to_str().unwrap();

    Ok(Rsp::success(models::SnapshotListRes {
        snapshots: handlers::snapshot_list(path, &app_context.redis_cache, project_id).await?,
    }))
}

pub async fn post_snapshot_restore(
    Extension(app_context): Extension<AppContext>,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(req): Json<models::SnapshotRestoreReq>,
) -> Result<Rsp<models::SnapshotRestoreRes>, IdpGlobalError> {
    let team_id = get_cookie_value_by_team_id(cookies);
    let path = path_tool::get_store_full_path(team_id, req.project_id, req.path);
    let path = path.to_str().unwrap();
    let project_id = req.project_id;
    Ok(Rsp::success(
        handlers::snapshot_restore(path, req.id, &app_context.redis_cache, project_id).await?,
    ))
}

// api: snapshot/diff
pub async fn post_snapshot_diff(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Extension(app_context): Extension<AppContext>,
    Json(req): Json<models::SnapshotDiffReq>,
) -> Result<Rsp<models::SnapshotDiffRes>, IdpGlobalError> {
    // let team_id = 0u64; //todo
    let team_id = get_cookie_value_by_team_id(cookies);
    let path: String = path_tool::get_store_full_path(team_id, req.project_id, req.path)
        .to_str()
        .unwrap()
        .to_string();
    let project_id = req.project_id;
    Ok(Rsp::success(
        handlers::snapshot_diff(req.id1, req.id2, path, &app_context.redis_cache, project_id)
            .await?,
    ))
}
