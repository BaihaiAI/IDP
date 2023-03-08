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

use std::sync::Arc;

use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use hyper::upgrade::Upgraded;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper_tungstenite::tungstenite::Message;
use hyper_tungstenite::WebSocketStream;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::error;
use tracing::info;

use super::ExecuteCodeReq;
use crate::app_context::KernelWsConn;
use crate::handler::prelude::team_id_from_cookie;
use crate::AppContext;
use crate::Error;

pub fn accept_ws_kernel_connect(
    ctx: AppContext,
    req: Request<Body>,
    kernel_info: kernel_common::KernelInfo,
) -> Result<Response<Body>, Error> {
    if !hyper_tungstenite::is_upgrade_request(&req) {
        return Err(Error::new("not a ws upgrade request"));
    }
    let inode = kernel_info.header.inode();
    let (response, websocket) = hyper_tungstenite::upgrade(req, None)
        .map_err(hyper_tungstenite::tungstenite::Error::Protocol)?;

    tokio::spawn(async move {
        let ws_stream = match websocket.await {
            Ok(ws_stream) => ws_stream,
            Err(err) => {
                tracing::error!("{err}");
                return;
            }
        };
        // browser -> kernel
        let (req_tx, mut req_rx) = tokio::sync::mpsc::channel(1000);
        // kernel -> browser
        let (rsp_tx, _rsp_rx) = tokio::sync::broadcast::channel(1000);
        let header = kernel_info.header.clone();
        let kernel = KernelWsConn {
            inode,
            kernel_info,
            req: req_tx,
            rsp: rsp_tx.clone(),
        };
        ctx.kernel_ws_conn_insert.send(kernel).await.unwrap();
        let (mut ws_write, mut ws_read) = futures_util::StreamExt::split(ws_stream);
        loop {
            tokio::select! {
                msg_res_opt = ws_read.next() => {
                    let msg_res = match msg_res_opt {
                        Some(msg_res) => msg_res,
                        None => {
                            tracing::warn!("EOF");
                            break;
                        }
                    };
                    let msg = match msg_res {
                        Ok(msg) => msg,
                        Err(err) => {
                            // e.g. Protocol(ResetWithoutClosingHandshake)
                            tracing::error!("kernel disconnect! {err}");
                            break;
                        }
                    };
                    let msg = match msg {
                        Message::Text(msg) => {
                            msg
                        },
                        Message::Ping(_) => {
                            if ws_write.send(Message::Pong(Vec::new())).await.is_err() {
                                tracing::error!("send pong back fail! break");
                                break;
                            }
                            continue;
                        }
                        Message::Close(_) => break,
                        _ => continue,
                    };
                    let msg = serde_json::from_str::<kernel_common::Message>(&msg).unwrap();
                    // let msg = kernel_common::Message::from_json(&msg.bytes().to_vec());
                    rsp_tx.send(msg).unwrap();
                }
                Some(req) = req_rx.recv() => {
                    ws_write.send(Message::text(serde_json::to_string(&req).unwrap())).await.unwrap();
                }
            }
        }
        // Ok::<(), Error>(())
        rsp_tx
            .send(kernel_common::Message {
                header,
                content: kernel_common::Content::ShutdownKernel {},
            })
            .unwrap();
        tracing::info!("end of ws idp_kernel->kernel_manage");
    });

    Ok(response)
}

pub fn accept_browser_execute_ws(
    ctx: AppContext,
    req: Request<Body>,
) -> Result<Response<Body>, Error> {
    if !hyper_tungstenite::is_upgrade_request(&req) {
        return Err(Error::new("not a ws upgrade request"));
    }

    let team_id = match team_id_from_cookie(&req) {
        Ok(team_id) => team_id,
        Err(err) => {
            tracing::error!("{err:#?}");
            match req.headers().get("Team-Id") {
                Some(team_id_header) => team_id_header.to_str()?.parse()?,
                None => {
                    return Err(Error::new(
                        "team_id not found in cookie or team_id is invalid",
                    ));
                }
            }
        }
    };
    let query_string = match req.uri().query() {
        Some(qs) => qs,
        None => return Err(Error::new("missing projectId in query string")),
    };
    let project_id = query_string
        .trim_start_matches("projectId=")
        .parse::<u64>()?;
    tracing::debug!("ws handshake req: team_id = {team_id}, project_id = {project_id}");
    debug_assert!(hyper_tungstenite::is_upgrade_request(&req));

    let (response, websocket) = hyper_tungstenite::upgrade(req, None)?;

    tokio::spawn(async move {
        let ws_stream = match websocket.await {
            Ok(ws_stream) => ws_stream,
            Err(err) => {
                tracing::error!("{err}");
                return;
            }
        };
        let (ws_write, ws_read) = futures_util::StreamExt::split(ws_stream);
        if let Err(err) = handle_ws(ctx, team_id, project_id, ws_write, ws_read).await {
            error!("handle_ws thread exit: {err:#?}");
        }
        debug!("websocket connection close");
    });
    Ok(response)
}

async fn handle_ws(
    ctx: AppContext,
    team_id: u64,
    project_id: u64,
    mut ws_write: SplitSink<WebSocketStream<Upgraded>, Message>,
    mut ws_read: SplitStream<WebSocketStream<Upgraded>>,
    // last_req_header: &mut kernel_common::Header,
) -> Result<(), Error> {
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
    msg_res_opt: Option<Result<Message, hyper_tungstenite::tungstenite::Error>>,
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
