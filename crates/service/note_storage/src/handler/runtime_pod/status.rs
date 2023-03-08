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
use axum::extract::State;
use common_model::Rsp;
use err::ErrorTrace;
use kernel_common::runtime_pod_status::PodStatusRsp;
use kernel_common::runtime_pod_status::RuntimeStatus;

use super::kubernetes_pod_status_watcher::RuntimeStatusMap;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatusReq {
    project_id: u64,
    // if resource not enough pod is pending, user choose keep wait pending pod
    wait_pending: Option<bool>,
}

#[allow(clippy::unused_async)]
pub async fn runtime_pod_status(
    Query(PodStatusReq {
        project_id,
        wait_pending,
    }): Query<PodStatusReq>,
    State(runtime_status_map): State<RuntimeStatusMap>,
) -> Result<Rsp<PodStatusRsp>, ErrorTrace> {
    if !business::kubernetes::is_k8s() {
        return Ok(Rsp::success(PodStatusRsp {
            status: RuntimeStatus::Running,
            ..Default::default()
        }));
    }
    let rsp = if let Some(wait_pending) = wait_pending {
        if let Some(entry) = runtime_status_map.0.write().await.get_mut(&project_id) {
            entry.inner.wait_pending = wait_pending;
            entry.inner.clone()
        } else {
            PodStatusRsp::default()
        }
    } else if let Some(entry) = runtime_status_map.0.read().await.get(&project_id) {
        entry.inner.clone()
    } else {
        PodStatusRsp::default()
    };
    Ok(Rsp::success(rsp))
}
