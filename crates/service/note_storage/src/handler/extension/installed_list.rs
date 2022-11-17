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
use common_model::Rsp;
use err::ErrorTrace;

use super::models::InstalledExtensionResp;
use super::models::ListReq;

pub async fn installed_list(
    Query(req): Query<ListReq>,
) -> Result<Rsp<Vec<InstalledExtensionResp>>, ErrorTrace> {
    let team_id = req.team_id;
    let user_id = req.user_id;
    installed_list_handler(team_id, user_id).await
}

pub async fn installed_list_handler(
    team_id: u64,
    user_id: u64,
) -> Result<Rsp<Vec<InstalledExtensionResp>>, ErrorTrace> {
    let extensions_path = business::path_tool::user_extensions_path(team_id, user_id);
    let extension_config_path =
        std::path::Path::new(&extensions_path).join("extensions_config.json");

    if !extension_config_path.exists() {
        std::fs::create_dir_all(&extensions_path)?;
        std::fs::File::create(&extension_config_path)?;
    }

    let mut installed_content = get_installed_extensions_config(extension_config_path).await?;

    let recommended_extensions = business::path_tool::recommended_extensions();
    let recommended_config_path =
        std::path::Path::new(&recommended_extensions).join("extensions_config.json");
    let recommended_content = super::get_extensions_config(recommended_config_path).await?;

    for i in &recommended_content {
        for j in &mut installed_content {
            if i.name == j.name && i.version != j.version {
                j.optional_version = Some(vec![i.version.clone()]);
            }
        }
    }

    Ok(Rsp::success(installed_content))
}

async fn get_installed_extensions_config(
    extension_config_path: std::path::PathBuf,
) -> Result<Vec<InstalledExtensionResp>, ErrorTrace> {
    let jdata = match std::fs::read_to_string(&extension_config_path) {
        Ok(jdata) => jdata,
        Err(err) => {
            let path = extension_config_path;
            tracing::error!("{err},path:{:?}", path);
            return Err(ErrorTrace::new("extension config no exist"));
        }
    };
    match serde_json::from_str::<Vec<InstalledExtensionResp>>(&jdata) {
        Ok(content) => Ok(content),
        Err(err) => {
            tracing::error!("{err}");
            let empty: Vec<InstalledExtensionResp> = Vec::new();
            Ok(empty)
        }
    }
}
