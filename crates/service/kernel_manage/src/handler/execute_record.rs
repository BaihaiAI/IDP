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

use super::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRecord {
    pub code: String,
    pub data_source_list: Vec<common_model::api_model::ActiveDataObj>,
    pub duration_ms: u32,
    pub run_at: chrono::NaiveDateTime,
    pub header: kernel_common::Header,
}

impl ExecuteRecord {
    pub fn key(&self) -> String {
        let header = &self.header;
        format!(
            "{}_{}_{}_{}_{}",
            header.team_id,
            header.project_id,
            header.path,
            header.cell_id,
            self.run_at.timestamp()
        )
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Req {
    team_id: u64,
    project_id: u64,
    path: String,
    cell_id: String,
}

pub fn find_notebook_cell_id_execute_record(
    _ctx: AppContext,
    req: Request<Body>,
) -> Result<Rsp<Vec<ExecuteRecord>>, Error> {
    #[cfg(not)]
    let req = serde_urlencoded::from_str::<Req>(req.uri().query().unwrap_or_default())?;

    #[cfg(not)]
    let prefix = format!(
        "{}_{}_{}_{}_",
        req.team_id, req.project_id, req.path, req.cell_id
    );
    let ret = Vec::new();
    #[cfg(not)]
    for (_k, v) in ctx.execute_record_db.scan_prefix(prefix).flatten() {
        if let Ok(record) = serde_json::from_slice::<ExecuteRecord>(&v) {
            ret.push(record);
        }
    }

    Ok(Rsp::success(ret))
}
