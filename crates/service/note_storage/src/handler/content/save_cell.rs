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

use axum::Extension;
use axum::Json;
use cache_io::CacheService;
use common_model::enums::mime::Mimetype;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use tower_cookies::Cookies;

use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveReq {
    pub path: String,
    pub content: String,
    pub project_id: u64,
}

pub async fn save(
    cookies: Cookies,
    Json(save_req): Json<SaveReq>,
    Extension(mut app_context): Extension<AppContext>,
) -> Result<Rsp<()>, IdpGlobalError> {
    tracing::info!("access save file content api.path:{}", save_req.path);
    let team_id = get_cookie_value_by_team_id(cookies);

    save_(
        save_req.path,
        save_req.content,
        team_id,
        save_req.project_id,
        &mut app_context.redis_cache,
    )
    .await
}

///
///Update or save expect notebook file.
pub async fn save_(
    path: String,
    content: String,
    team_id: u64,
    project_id: u64,
    redis_cache: &mut CacheService,
) -> Result<Rsp<()>, IdpGlobalError> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    let mime = crate::handler::content::cat::file_mime_magic::get_mime_type(&path)?;
    let file_ext = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };
    let mimetype =
        if mime == "application/x-ipynb+json" || file_ext == "ipynb" || file_ext == "idpnb" {
            Mimetype::Notebook
        } else {
            Mimetype::Text
        };
    redis_cache
        .update_file_content(&path, content, mimetype, project_id)
        .await?;
    Ok(Rsp::success(()))
}
