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

use std::io::Write;

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub user_id: u64,
    pub name: String,
}

pub async fn uninstall(Query(req): Query<ExtensionReq>) -> Result<Rsp<()>, ErrorTrace> {
    let extensions_path = business::path_tool::user_extensions_path(req.team_id, req.user_id);

    let uninstall_extension_path = std::path::Path::new(&extensions_path).join(&req.name);
    tracing::info!("run extensions uninstall api, path:{uninstall_extension_path:?}");

    if let Err(err) = std::fs::remove_dir_all(uninstall_extension_path) {
        tracing::error!("{:?}", err);
        return Err(ErrorTrace::new("uninstall extension failed"));
    };

    let extensions_config_path =
        std::path::Path::new(&extensions_path).join("extensions_config.json");

    let mut content = super::get_extensions_config(&extensions_config_path)?;
    content.retain(|extension| extension.name != req.name);

    let data = serde_json::to_string(&content)?;
    let mut f = std::fs::File::create(extensions_config_path)?;
    f.write_all(data.as_bytes())?;

    Ok(Rsp::success_without_data())
}
