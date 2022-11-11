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

use axum::Json;
use common_model::enums::mime::Mimetype;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveReq {
    pub path: String,
    pub content: String,
    pub project_id: u64,
}

pub async fn save(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(save_req): Json<SaveReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("access save file content api.path:{}", save_req.path);
    let team_id = get_cookie_value_by_team_id(cookies);

    save_(
        save_req.path,
        save_req.content,
        team_id,
        save_req.project_id,
    )
    .await
}

/// how IDP save non-ipynb text files
/// 1. editor send save file in 5s interval
/// 2. if editor onblur(loss focus, e.g. close tab) then send save req
pub async fn save_(
    path: String,
    content: String,
    team_id: u64,
    project_id: u64,
) -> Result<Rsp<()>, ErrorTrace> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    let mime = crate::handler::content::cat::file_mime_magic::get_mime_type(&path)?;
    let file_ext = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };
    let file_type =
        if mime == "application/x-ipynb+json" || file_ext == "ipynb" || file_ext == "idpnb" {
            return Err(ErrorTrace::new("ipynb file can't use save API"));
        } else {
            Mimetype::Text
        };
    // debug_assert_ne!(file_type, Mimetype::Notebook);
    common_tools::file_tool::write_large_to_nfs(path.to_str().unwrap(), content.clone(), file_type)
        .await?;
    Ok(Rsp::success(()))
}
