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

use axum::extract::Json;
use common_model::Rsp;
use err::ErrorTrace;

use super::models::ExtensionReq;
use crate::handler::extension::models::ExtensionResp;
use crate::handler::extension::write_file_lock;

pub async fn update(Json(req): Json<ExtensionReq>) -> Result<Rsp<String>, ErrorTrace> {
    tracing::info!("run update extensions,{:?}", req);
    update_handler(req).await
}

async fn update_handler(req: ExtensionReq) -> Result<Rsp<String>, ErrorTrace> {
    let installed_extensions_path =
        business::path_tool::user_extensions_path(req.team_id, req.user_id);
    let extension_name = format!("{}/{}", req.name, req.version);
    tracing::info!("run extensions update path:{installed_extensions_path} ,name:{extension_name}");
    let recommended_extension_path =
        business::path_tool::recommended_extensions().join(&extension_name);

    let extension_path = format!("{}/{}", installed_extensions_path, req.name);
    common_tools::command_tools::copy(
        recommended_extension_path.to_str().unwrap(),
        &extension_path,
    )
    .await?;

    let jdata = tokio::fs::read_to_string(recommended_extension_path.join("config.json")).await?;
    let mut new_extension_config = serde_json::from_str::<ExtensionResp>(&jdata)?;

    let url = format!("{installed_extensions_path}/{extension_name}/");
    new_extension_config.url = Some(url.clone());

    let extensions_config_path =
        std::path::Path::new(&installed_extensions_path).join("extensions_config.json");

    let contents = super::get_extensions_config(&extensions_config_path).await?;

    let contents = contents
        .into_iter()
        .map(|mut extension| {
            if extension.name == req.name {
                extension = new_extension_config.to_owned();
            };
            extension
        })
        .collect::<Vec<ExtensionResp>>();

    let data = serde_json::to_string(&contents).unwrap();

    write_file_lock(extensions_config_path, data).await?;

    Ok(Rsp::success(url))
}
