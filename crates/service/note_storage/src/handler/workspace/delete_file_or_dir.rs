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
use common_model::Rsp;

use crate::api_model::workspace::FullFileTreeNode;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler::workspace::dir_recursive_load;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFileOrDirReq {
    pub team_id: u64,
    pub project_id: u64,
    pub path: String,
    // pub auto_close: Option<bool>,
}

pub async fn delete_file_or_dir(
    Query(req): Query<DeleteFileOrDirReq>,
    State(ctx): State<AppContext>,
) -> Result<Rsp<FullFileTreeNode>, IdpGlobalError> {
    tracing::info!("--> delete_file_or_dir_ {req:?}");

    let DeleteFileOrDirReq {
        team_id,
        project_id,
        path,
    } = req;

    let abs_path = business::path_tool::get_store_full_path(team_id, project_id, &path);

    tracing::debug!("--> abs_path={:?}", abs_path);
    if !abs_path.exists() {
        let none_node = FullFileTreeNode {
            absolute_path: abs_path.display().to_string(),
            browser_path: path.to_string(),
            project_id: project_id.to_string(),
            file_name: path.to_string(),
            file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
            has_children: false,
            children: vec![],
        };
        return Ok(Rsp::success(none_node));
    }

    let only_pipeline_support = false;
    let res = dir_recursive_load(team_id, project_id, path.clone(), only_pipeline_support).await;
    crate::handler::kernel::shutdown_by_dir_path(project_id, path.clone()).await?;
    let meta = std::fs::metadata(&abs_path)?;
    if meta.is_file() {
        tokio::fs::remove_file(&abs_path).await?;
        if path.ends_with(".ipynb") || path.ends_with(".idpnb") {
            ctx.redis_cache
                .del_file_cache_key(&cache_io::keys::ipynb_key(
                    abs_path.to_str().unwrap(),
                    project_id,
                ))
                .await?;
        }
    } else {
        tokio::fs::remove_dir_all(abs_path).await?;
    }
    res
}
