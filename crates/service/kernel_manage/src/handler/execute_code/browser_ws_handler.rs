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

use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::Query;
use axum::extract::State;
use axum::extract::TypedHeader;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use err::ErrorTrace;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::RwLock;
use tracing::error;
use tracing::info;

use super::ExecuteCodeReq;
use crate::handler::kernel_list::ProjectIdQueryString;
use crate::AppContext;

pub async fn browser_ws_handler(
    ws: WebSocketUpgrade,
    Query(req): Query<ProjectIdQueryString>,
    TypedHeader(cookies): TypedHeader<axum::headers::Cookie>,
    State(ctx): State<AppContext>,
) -> Result<Response, ErrorTrace> {
    let team_id = match cookies.get("teamId") {
        Some(team_id) => team_id.parse::<u64>()?,
        None => return Err(ErrorTrace::new("no teamId found in cookie")),
    };
    let project_id = req.project_id;
    Ok(ws.on_upgrade(move |socket| async move {
        if let Err(err) = browser_ws_socket_handler(socket, team_id, project_id, ctx).await {
            error!("browser ws close {err}");
        }
    }))
}

async fn browser_ws_socket_handler(
    socket: WebSocket,
    team_id: u64,
    project_id: u64,
    ctx: AppContext,
) -> Result<(), ErrorTrace> {
    let (mut ws_write, mut ws_read) = socket.split();
    // NOTE can't use ws_write directly, must send output to broadcast first, make sure client ws disconnect and second connect can receive message
    let mut resp_receiver = ctx.broadcast_output_to_all_ws_write.subscribe();

    let stop_on_error_batch_ids = Arc::new(RwLock::new(lru::LruCache::<u64, ()>::new(
        std::num::NonZeroUsize::new(5).unwrap(),
    )));
    let (req_producer, mut req_consumer) = tokio::sync::mpsc::channel::<ExecuteCodeReq>(2000);
    let stop_on_error_batch_ids_ = stop_on_error_batch_ids.clone();
    let ws_write_producer = ctx.broadcast_output_to_all_ws_write.clone();
    let ws_write_producer_ = ws_write_producer.clone();

    tokio::spawn(async move {
        let ws_write_producer = ws_write_producer_.clone();
        let stop_on_error_batch_ids = stop_on_error_batch_ids_;
        // let ctx = ctx_.clone();
        while let Some(req) = req_consumer.recv().await {
            // info!("--> req_consumer.recv()");
            let req_header = req.header.clone();
            let batch_id = req.batch_id;
            if stop_on_error_batch_ids.read().await.contains(&batch_id) {
                error!("reply on stop because previous req error in same batch");
                let msg = kernel_common::Message {
                    content: kernel_common::Content::ReplyOnStop {},
                    header: req_header,
                };
                if ws_write_producer.send(msg).is_err() {
                    tracing::error!("ws_write_producer send err");
                }
                continue;
            }
            // NOTE add send req to kernel queue must BLOCKING, otherwise run all cell order maybe wrong
            if let Err(err) = super::add_req_to_pending::add_req_to_pending(&ctx, req).await {
                tracing::warn!("{err:#?}");
                stop_on_error_batch_ids.write().await.put(batch_id, ());

                let msg = kernel_common::Message {
                    content: kernel_common::Content::RuntimeError {
                        message: err.message,
                    },
                    header: req_header,
                };
                if let Err(err) = ws_write_producer.send(msg) {
                    tracing::error!("{err}");
                }
            }
            info!("req_consumer after add_req_to_pending");
        }
    });

    let client_id = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    loop {
        tokio::select! {
            msg_res_opt = ws_read.next() => {
                if execute_code_ws_read_handler(&ws_write_producer, &req_producer, msg_res_opt, &stop_on_error_batch_ids, team_id, project_id, client_id).await {
                    break
                }
            }
            output_res = resp_receiver.recv() => {
                let output = match output_res {
                    Ok(output) => output,
                    Err(err) => {
                        error!("resp_receiver.recv() {err}");
                        continue;
                    }
                };
                if output.header.team_id == team_id && output.header.project_id == project_id {
                    // info!("--> resp_receiver.recv()");
                    if let kernel_common::Content::Pong{client_id: client_id_ } = output.content {
                        if client_id_ == client_id {
                            ws_write.send(Message::Text("pong".to_string())).await?;
                        }
                    } else {
                        ws_write.send(Message::Text(serde_json::to_string(&output).unwrap())).await?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// return is break
async fn execute_code_ws_read_handler(
    ws_write_producer: &tokio::sync::broadcast::Sender<kernel_common::Message>,
    req_producer: &tokio::sync::mpsc::Sender<ExecuteCodeReq>,
    msg_res_opt: Option<Result<Message, axum::Error>>,
    stop_on_error_batch_ids: &Arc<RwLock<lru::LruCache<u64, ()>>>,
    team_id: u64,
    project_id: u64,
    client_id: u128,
    // _ctx: &AppContext
) -> bool {
    let msg_res = match msg_res_opt {
        Some(msg_res) => msg_res,
        None => return true,
    };
    let msg = match msg_res {
        Ok(msg) => match msg {
            Message::Text(msg) => {
                if msg == "ping" {
                    if ws_write_producer
                        .send(kernel_common::Message {
                            header: kernel_common::Header {
                                project_id,
                                team_id,
                                ..Default::default()
                            },
                            content: kernel_common::Content::Pong { client_id },
                        })
                        .is_err()
                    {
                        error!("ws rsp pong fail, close connection");
                        return true;
                    }
                    return false;
                }
                msg
            }
            Message::Close(_) => return true,
            _ => return false,
        },
        Err(err) => {
            tracing::warn!("browser-kernel_manage {err}");
            return true;
        }
    };
    let req = match serde_json::from_str::<ExecuteCodeReq>(&msg) {
        Ok(req) => req,
        Err(err) => {
            tracing::error!("{err}");
            let (cell_id, path, batch_id) = if let Ok(req) =
                serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&msg)
            {
                (
                    req.get("cellId")
                        .map(|v| v.as_str().unwrap_or_default())
                        .unwrap_or_default()
                        .to_string(),
                    req.get("path")
                        .map(|v| v.as_str().unwrap_or_default())
                        .unwrap_or_default()
                        .to_string(),
                    req.get("batchId")
                        .map(|v| v.as_u64().unwrap_or_default())
                        .unwrap_or_default(),
                )
            } else {
                ("".to_string(), "".to_string(), 0)
            };
            stop_on_error_batch_ids.write().await.put(batch_id, ());
            let msg = kernel_common::Message {
                content: kernel_common::Content::RuntimeError {
                    message: err.to_string(),
                },
                header: kernel_common::Header {
                    team_id,
                    project_id,
                    path,
                    cell_id,
                    ..Default::default()
                },
            };
            if ws_write_producer.send(msg).is_err() {
                tracing::error!("ws_write_producer send err");
            }
            return false;
        }
    };
    let req_header = req.header.clone();
    // subscribe_list.insert(req.header.path.clone());
    let batch_id = req.batch_id;
    if stop_on_error_batch_ids.read().await.contains(&batch_id) {
        error!("reply on stop because previous req error in same batch");
        let msg = kernel_common::Message {
            content: kernel_common::Content::ReplyOnStop {},
            header: req_header,
        };
        if ws_write_producer.send(msg).is_err() {
            tracing::error!("ws_write_producer send err");
        }
        return false;
    }

    if req_producer.send(req).await.is_err() {
        error!("req_producer send error");
    }

    // NOTE add send req to kernel queue must BLOCKING, otherwise run all cell order maybe wrong
    /*
    let req_header = req.header.clone();
    let batch_id = req.batch_id;
    if let Err(err) = super::add_req_to_pending::add_req_to_pending(&ctx, req).await {
        tracing::warn!("{err:#?}");
        stop_on_error_batch_ids.write().await.put(batch_id, ());

        let msg = kernel_common::Message {
            content: kernel_common::Content::RuntimeError {
                message: err.message,
            },
            header: req_header,
            ..Default::default()
        };
        if let Err(err) = ws_write_producer.send(msg) {
            tracing::error!("{err}");
        }
    }
    */

    false
}
