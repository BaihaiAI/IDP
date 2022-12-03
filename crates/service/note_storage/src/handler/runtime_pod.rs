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
use kernel_common::spawn_kernel_process::Resource;
use tracing::warn;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Req {
    project_id: u64,
    team_id: u64,
}

#[derive(serde::Serialize)]
pub struct Resp {
    running: bool,
    resource: Resource,
}

#[allow(clippy::unused_async)]
pub async fn runtime_pod_status(
    Query(Req {
        project_id,
        team_id,
    }): Query<Req>,
) -> Result<Rsp<Resp>, ErrorTrace> {
    if !business::kubernetes::is_k8s() {
        return Ok(Rsp::success(Resp {
            running: true,
            resource: Resource::default(),
        }));
    }
    let is_running = business::kubernetes::runtime_pod_is_running(project_id);
    let path = format!("/store/{team_id}/projects/{project_id}/last_resource_setting.json");
    let resource = match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(x) => x,
            Err(err) => {
                warn!("{path} {err}");
                Resource::default()
            }
        },
        Err(err) => {
            warn!("{path} {err}");
            Resource::default()
        }
    };
    Ok(Rsp::success(Resp {
        running: is_running,
        resource,
    }))
}
