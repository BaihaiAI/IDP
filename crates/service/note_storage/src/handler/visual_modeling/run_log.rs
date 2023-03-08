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
use std::time::Duration;

use axum::extract::Query;
use axum::response::sse::Event;
use axum::response::sse::KeepAlive;
use axum::response::sse::Sse;
use axum::response::IntoResponse;
use axum::response::Response;
use err::ErrorTrace;
use futures::stream::Stream;
use futures::StreamExt;
use kernel_common::kubernetes_client;
use tracing::error;
use tracing::info;

use super::prelude::*;
use crate::handler::visual_modeling::helper::k8s_svc::pod_name;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLogReq {
    pub job_instance_id: i32,
    pub team_id: u64,
    pub node_id: String,
}

impl NodeLogReq {
    pub fn log_path(&self) -> String {
        let &NodeLogReq {
            job_instance_id,
            team_id,
            node_id,
        } = &self;
        format!("/store/{team_id}/visual_modeling/{job_instance_id}/{node_id}.log")
    }
}

pub async fn logs(Query(req): Query<NodeLogReq>) -> axum::response::Response {
    let job_instance_id = req.job_instance_id;
    let node_id = &req.node_id;

    let status = match RunStatus::from_db(job_instance_id).await {
        Ok(status) => status,
        Err(err) => {
            let mut rsp = error_trace_to_sse(err);
            *rsp.status_mut() = axum::http::StatusCode::NOT_FOUND;
            return rsp;
        }
    };
    let node = match status.node(node_id) {
        Ok(node) => node,
        Err(err) => {
            let stream = futures::stream::once(async {
                Ok::<_, Infallible>(Event::default().data(err.message))
            });
            let mut rsp = Sse::new(stream).into_response();
            *rsp.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return rsp;
        }
    };
    let node_is_finish = status.job_instance_status.is_finish() || node.status.is_finish();

    if node_is_finish {
        match read_finish_pod_log(req).await {
            Ok(x) => x.into_response(),
            Err(err) => error_trace_to_sse(err),
        }
    } else {
        if node.status != NodeStatus::Running {
            return error_trace_to_sse(ErrorTrace::new(
                "pod is not running or finish, can' get log",
            ));
        }
        match read_log_from_k8s(req).await {
            Ok(x) => x.into_response(),
            Err(err) => error_trace_to_sse(err),
        }
    }
}

fn error_trace_to_sse(err: ErrorTrace) -> Response {
    error!("{err}");
    let stream =
        futures::stream::once(async { Ok::<_, Infallible>(Event::default().data(err.message)) });
    let mut rsp = Sse::new(stream).into_response();
    *rsp.status_mut() = axum::http::StatusCode::BAD_REQUEST;
    rsp
}

async fn read_finish_pod_log(
    req: NodeLogReq,
) -> Result<Sse<impl Stream<Item = Result<Event, std::io::Error>>>, ErrorTrace> {
    let log_path = req.log_path();
    info!("log_path = {log_path}");
    let file = match tokio::fs::File::open(log_path).await {
        Ok(x) => x,
        Err(_) => {
            return Err(ErrorTrace::new(
                "pod is not running or finish, no log file found",
            ));
        }
    };
    let file = tokio::io::BufReader::new(file);
    let file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
    let stream = futures::AsyncBufReadExt::lines(file).map(|x| match x {
        Ok(line) => Ok(Event::default().data(line)),
        Err(x) => Err(x),
    });
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(45))))
}

async fn read_log_from_k8s(
    req: NodeLogReq,
) -> Result<Sse<impl Stream<Item = Result<Event, std::io::Error>>>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    let node_id = &req.node_id;
    let pod_name = pod_name(job_instance_id, node_id);
    let url = format!(
        "{}/{pod_name}/log?follow=true",
        kubernetes_client::k8s_api_server_base_url()
    );
    let rsp = kubernetes_client::k8s_api_client().get(&url).send().await?;
    let status = rsp.status().as_u16();
    if !rsp.status().is_success() {
        error!("{pod_name} {status}")
    }
    if status == 400 {
        return Err(ErrorTrace::new("pod is creating, no log"));
    }
    if status == 404 {
        return Err(ErrorTrace::new("pod not found"));
    }
    let stream = rsp.bytes_stream();
    let stream = futures::StreamExt::map(stream, |x| match x {
        Ok(x) => Ok(x),
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        )),
    });
    let stream = futures::TryStreamExt::into_async_read(stream);
    let stream = futures::AsyncBufReadExt::lines(stream).map(|x| match x {
        Ok(line) => Ok(Event::default().data(line)),
        Err(x) => Err(x),
    });
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(45))))
}

#[cfg(not)]
pub async fn logs(Query(req): Query<NodeLogReq>) -> Result<Rsp<String>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    let status = RunStatus::from_db(job_instance_id).await?;

    if JOB_INSTANCE_STATUS
        .read()
        .await
        .get(&job_instance_id)
        .is_some()
    {
        let pod_name = pod_name(job_instance_id, &req.node_id);
        let url = format!(
            "{}/{pod_name}/log",
            kubernetes_client::k8s_api_server_base_url()
        );
        let rsp = kubernetes_client::k8s_api_client().get(&url).send().await?;
        let status = rsp.status().as_u16();
        if !rsp.status().is_success() {
            tracing::error!("{pod_name} {status}")
        }
        if status == 400 {
            return Err(ErrorTrace::new("pod is creating, no log"));
        }
        if status == 404 {
            return Err(ErrorTrace::new("pod not found"));
        }
        let rsp = rsp.text().await?;
        Ok(Rsp::success(rsp))
    } else {
        let log_path = req.log_path();
        info!("log_path = {log_path}");
        match tokio::fs::read_to_string(&log_path).await {
            Ok(log) => Ok(Rsp::success(log)),
            Err(_) => Err(ErrorTrace::new("pod not run or fail, no log file found")),
        }
    }
}
