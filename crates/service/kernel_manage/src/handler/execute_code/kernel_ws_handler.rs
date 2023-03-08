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

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::headers::HeaderMap;
use axum::response::Response;
use err::ErrorTrace;
use futures_util::SinkExt;
use futures_util::StreamExt;
use kernel_common::KernelInfo;

use crate::app_context::KernelWsConn;
use crate::AppContext;

#[allow(clippy::unused_async)]
pub async fn kernel_ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(ctx): State<AppContext>,
) -> Result<Response, ErrorTrace> {
    if !headers.contains_key(KernelInfo::HTTP_HEADER) {
        return Err(ErrorTrace::new("missing header"));
    }
    let kernel_info = headers[KernelInfo::HTTP_HEADER].to_str()?;
    let kernel_info = urlencoding::decode(kernel_info)?;
    let kernel_info = serde_json::from_str::<KernelInfo>(&kernel_info)?;
    Ok(ws.on_upgrade(move |socket| kernel_ws_socket_handler(socket, ctx, kernel_info)))
}

async fn kernel_ws_socket_handler(socket: WebSocket, ctx: AppContext, kernel_info: KernelInfo) {
    // browser -> kernel
    let (req_tx, mut req_rx) = tokio::sync::mpsc::channel(1000);
    // kernel -> browser
    let (rsp_tx, _rsp_rx) = tokio::sync::broadcast::channel(1000);
    let header = kernel_info.header.clone();
    let kernel = KernelWsConn {
        inode: kernel_info.header.inode(),
        kernel_info,
        req: req_tx,
        rsp: rsp_tx.clone(),
    };
    ctx.kernel_ws_conn_insert.send(kernel).await.unwrap();
    let (mut ws_write, mut ws_read) = socket.split();
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
                ws_write.send(Message::Text(serde_json::to_string(&req).unwrap())).await.unwrap();
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
}
