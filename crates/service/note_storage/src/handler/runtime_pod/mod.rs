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

mod applying;
mod delete_project_hook;
mod kubernetes_pod_status_watcher;
mod resource_usage;
mod status;
mod subscribe_status;
// pub mod start_pod;

use axum::routing::get;
use axum::routing::post;
use axum::Router;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectIdQueryString {
    project_id: u64,
}

pub fn routes() -> Router {
    let runtime_status_map = kubernetes_pod_status_watcher::RuntimeStatusMap::new();
    let runtime_status_map_ = runtime_status_map.clone();
    tokio::spawn(async move {
        kubernetes_pod_status_watcher::spawn_runtime_pod_watcher(runtime_status_map_).await;
    });
    Router::new()
        .route("/status", get(status::runtime_pod_status))
        .route(
            "/resource_usage",
            get(resource_usage::runtime_resource_usage),
        )
        .route("/notify_apply", post(applying::notify_pod_apply))
        .route("/subscribe_status", get(subscribe_status::sse_handler))
        .route(
            "/delete_project_hook",
            axum::routing::delete(delete_project_hook::before_delete_project_hook),
        )
        .with_state(runtime_status_map)
}
