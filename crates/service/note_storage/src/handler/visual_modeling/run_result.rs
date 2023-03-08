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

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;

pub use super::prelude::*;
use super::run_log::NodeLogReq;

pub async fn result(
    Query(NodeLogReq {
        job_instance_id,
        node_id,
        team_id,
    }): Query<NodeLogReq>,
) -> Result<Rsp<Vec<serde_json::Value>>, ErrorTrace> {
    let result_path =
        format!("/store/{team_id}/visual_modeling/{job_instance_id}/{node_id}.display_data");
    let display_data = tokio::fs::read_to_string(result_path)
        .await
        .unwrap_or_else(|_| "[]".to_string());
    Ok(Rsp::success(serde_json::from_str(&display_data)?))
}
