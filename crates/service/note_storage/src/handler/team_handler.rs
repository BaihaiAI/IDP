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
use common_model::service::rsp::Rsp;

use crate::common::error::ErrorTrace;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamReq {
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
}

pub async fn init_team(Json(payload): Json<TeamReq>) -> Result<Rsp<()>, err::ErrorTrace> {
    tracing::debug!("access init_team api");
    let team_id = payload.team_id;
    init_team_handler(team_id).await
}

pub async fn init_team_handler(team_id: i64) -> Result<Rsp<()>, err::ErrorTrace> {
    tracing::debug!("access init_team_handler api");
    let team_dir = format!("/store/{team_id}");
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("/opt/terminal/addRoot.sh").arg(team_dir);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }
    Ok(Rsp::success_without_data())
}
