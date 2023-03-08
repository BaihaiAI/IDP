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
use axum::extract::State;
use common_model::Rsp;
use kernel_common::runtime_pod_status::RuntimeStatus;

use super::kubernetes_pod_status_watcher::RuntimeStatusMap;
use super::ProjectIdQueryString;

pub async fn notify_pod_apply(
    Query(ProjectIdQueryString { project_id }): Query<ProjectIdQueryString>,
    State(runtime_status_map): State<RuntimeStatusMap>,
) -> Rsp<()> {
    let mut map = runtime_status_map.0.write().await;
    let status_entry = map.entry(project_id).or_default();
    status_entry.update_status_and_notify_sse(RuntimeStatus::Applying);
    Rsp::success(())
}
