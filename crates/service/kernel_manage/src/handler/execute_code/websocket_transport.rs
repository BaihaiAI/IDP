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
use tracing::debug;
use tracing::error;

use super::ExecuteCodeReq;
use crate::app_context::KernelWsConn;
use crate::handler::prelude::team_id_from_cookie;
use crate::AppContext;
use crate::Error;

// #[cfg(feature = "tcp")]
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
        let (req_tx, mut req_rx) = tokio::sync::mpsc::channel(100);
        let (rsp_tx, _rsp_rx) = tokio::sync::broadcast::channel(100);
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
                            tracing::error!("{err}");
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
        tracing::info!("enter of ws idp_kernel->kernel_manage");
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
    // .map_err(hyper_tungstenite::tungstenite::Error::Protocol)?;

    tokio::spawn(async move {
        let ws_stream = match websocket.await {
            Ok(ws_stream) => ws_stream,
            Err(err) => {
                tracing::error!("{err}");
                return;
            }
        };
        let (mut ws_write, ws_read) = futures_util::StreamExt::split(ws_stream);
        if let Err(err) = handle_ws(ctx, team_id, project_id, &mut ws_write, ws_read).await {
            if err.message.starts_with("WebSocket protocol") {
                error!("handle_ws thread exit: {}", err.message);
            } else {
                error!("handle_ws thread exit: {err:#?}");
            }
        }
        debug!("websocket connection close");
    });
    Ok(response)
}

async fn handle_ws(
    ctx: AppContext,
    team_id: u64,
    project_id: u64,
    ws_write: &mut SplitSink<WebSocketStream<Upgraded>, Message>,
    mut ws_read: SplitStream<WebSocketStream<Upgraded>>,
    // last_req_header: &mut kernel_common::Header,
) -> Result<(), Error> {
    let mut resp_receiver = ctx.output_to_ws_sender.subscribe();
    loop {
        tokio::select! {
            msg_res_opt = ws_read.next() => {
                let msg_res = match msg_res_opt {
                    Some(msg_res) => msg_res,
                    None => break
                };
                let msg = match msg_res? {
                    Message::Text(msg) => {
                        if msg == "ping" {
                            ws_write.send(Message::text("pong")).await?;
                            continue;
                        }
                        msg
                    },
                    Message::Close(_) => break,
                    _ => continue,
                };
                let req = serde_json::from_str::<ExecuteCodeReq>(&msg)?;
                let req_header = req.header.clone();
                // subscribe_list.insert(req.header.path.clone());
                let ws_write_tx = ctx.output_to_ws_sender.clone();
                let ctx = ctx.clone();
                // NOTE add send req to kernel queue must BLOCKING, otherwise run all cell order maybe wrong
                // tokio::spawn(async move {
                if let Err(err) = super::add_req_to_pending::add_req_to_pending(&ctx, req).await {
                    tracing::warn!("{err:#?}");
                    let msg = kernel_common::Message {
                        content: kernel_common::Content::RuntimeError {
                            message: err.message,
                        },
                        header: req_header,
                        ..Default::default()
                    };
                    if let Err(err) = ws_write_tx.send(msg) {
                        tracing::error!("{err}");
                    }
                }
                // });
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
                    ws_write.send(Message::text(serde_json::to_string(&output).unwrap())).await?;
                }
            }
        }
    }

    Ok(())
}
