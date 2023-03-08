// Copyright 2023 BaihaiAI, Inc.
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
use axum::extract::TypedHeader;
use common_model::Rsp;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;

use crate::service::component::dag_service::job_run_dag_service;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelJobReq {
    pub job_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub run_node_type: RunNodeType,
    pub node_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RunNodeType {
    /// run current node only
    Single,
    /// run current node and it's all children nodes
    Below,
    All,
}

impl RunNodeType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Single" => Self::Single,
            "Below" => Self::Below,
            "All" => Self::All,
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelJobInstanceDto {
    pub job_instance_id: i32,
}

pub async fn manual_run_job_instance(
    TypedHeader(cookies): TypedHeader<axum::headers::Cookie>,
    Json(req): Json<ModelJobReq>,
) -> Result<Rsp<ModelJobInstanceDto>, ErrorTrace> {
    let job_id = req.job_id;
    let team_id = req.team_id;
    let user_id =
        common_tools::cookies_tools::get_cookie_value_by_key(&cookies, "userId").parse::<i64>()?;
    let run_node_type = req.run_node_type;
    let node_id = req.node_id;
    let run_type = "Manual";

    let id =
        job_run_dag_service(job_id, team_id, user_id, run_type, run_node_type, node_id).await?;
    Ok(Rsp::success(ModelJobInstanceDto {
        job_instance_id: id,
    }))
}
