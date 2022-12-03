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
    uninstall_handler(req.team_id, req.user_id, &req.name).await
}

pub async fn uninstall_handler(
    team_id: u64,
    user_id: u64,
    name: &str,
) -> Result<Rsp<()>, ErrorTrace> {
    let extensions_path = business::path_tool::user_extensions_path(team_id, user_id);

    let uninstall_extension_path = std::path::Path::new(&extensions_path).join(name);
    tracing::info!("run extensions uninstall api, path:{uninstall_extension_path:?}");

    let mut cmd = tokio::process::Command::new("rm");
    cmd.arg("-rf").arg(&uninstall_extension_path);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output().await.unwrap();
    if !output.status.success() {
        tracing::error!(
            "fail to unistall: {:#?},err: {}",
            uninstall_extension_path,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let extensions_config_path =
        std::path::Path::new(&extensions_path).join("extensions_config.json");

    let mut content = super::get_extensions_config(&extensions_config_path).await?;
    content.retain(|extension| extension.name != name);

    let data = serde_json::to_string(&content)?;
    let mut f = std::fs::File::create(extensions_config_path)?;
    f.write_all(data.as_bytes())?;

    Ok(Rsp::success_without_data())
}
