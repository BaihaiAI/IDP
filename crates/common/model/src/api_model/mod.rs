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

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUpdateCellReq {
    pub path: String,
    pub project_id: u64,
    pub cells: Vec<crate::entity::cell::CellUpdate>,
}

pub async fn get_develop_active_connect_data(
    project_id: u64,
) -> Result<ActiveDataRet, reqwest::Error> {
    // /ga/api/v1/command/database/active
    // curl -X POST http://127.0.0.1:8083/api/command/database/active -H 'Content-Type: application/json' -d '{"project":"1248"}'
    const ACTIVE_DATASOURCE_URL: &str = "http://127.0.0.1:8083/api/command/database/active";
    let ret = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_millis(1200))
        .build()?
        .post(ACTIVE_DATASOURCE_URL)
        .json(&serde_json::json!({ "project": project_id.to_string() }))
        .send()
        .await?
        .json::<ActiveDataRet>()
        .await?;
    if ret.code != 200 {
        eprintln!(
            "database/active request panicked at code = 500, message = {}",
            ret.message
        );
    }
    Ok(ret)
}

#[derive(serde::Deserialize)]
pub struct ActiveDataRet {
    pub code: u32,
    pub message: String,
    pub data: Vec<ActiveDataObj>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDataObj {
    pub alias: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data_source_type: String,
}
