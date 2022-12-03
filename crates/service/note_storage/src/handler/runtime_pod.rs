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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectId {
    project_id: u64,
}

#[allow(clippy::unused_async)]
pub async fn runtime_pod_status(
    Query(ProjectId { project_id }): Query<ProjectId>,
) -> Result<Rsp<bool>, ErrorTrace> {
    if !business::kubernetes::is_k8s() {
        return Ok(Rsp::success(true));
    }
    Ok(Rsp::success(business::kubernetes::runtime_pod_is_running(
        project_id,
    )))
}
