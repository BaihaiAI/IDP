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

use std::convert::Infallible;

use axum::extract::Query;
use axum::extract::State;
use axum::response::sse::Event;
use axum::response::sse::Sse;
use err::ErrorTrace;
use futures::stream::Stream;
use kernel_common::runtime_pod_status::PodStatusRsp;

// use futures::StreamExt;
use super::kubernetes_pod_status_watcher::RuntimeStatusMap;
use super::ProjectIdQueryString;

fn status_to_sse_event(status: PodStatusRsp) -> Event {
    Event::default().data(serde_json::to_string(&status).unwrap())
}

// #[cfg(not)]
pub async fn sse_handler(
    Query(ProjectIdQueryString { project_id }): Query<ProjectIdQueryString>,
    State(runtime_status_map): State<RuntimeStatusMap>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ErrorTrace> {
    // TODO check project_id is_delete in db
    let mut entry = runtime_status_map.0.write().await;
    let entry = entry.entry(project_id).or_default();
    let curr_status = entry.inner.clone();
    let mut rx = entry.sse_notify_tx.subscribe();

    let stream = async_stream::try_stream! {
        yield status_to_sse_event(curr_status);
        loop {
            match rx.recv().await {
                Ok(status) => yield status_to_sse_event(status),
                Err(err) => {
                    tracing::error!("{err}");
                    break;
                }
            }
        }
    };

    let sse = Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new().interval(std::time::Duration::from_secs(5)),
    );
    Ok(sse)
}

#[cfg(not)]
pub async fn sse_handler2() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut stream = futures::stream::iter(1..9)
        .map(|num| Event::default().data(num.to_string()))
        .map(Ok);
    let sse = Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(30))
            .text("keep-alive-text"),
    );
    sse
}
