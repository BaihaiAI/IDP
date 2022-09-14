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

use kernel_common::Content;
use kernel_common::Message;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use super::kernel_operate::KernelOperate;
use super::KernelCtx;
use crate::app_context::KernelEntryOps;
use crate::handler::prelude::State;

pub async fn kernel_main_loop(
    mut kernel_ctx: KernelCtx,
    kernel_entry_ops_tx: Sender<KernelEntryOps>,
    mut req_receiver: Receiver<Message>,
    mut kernel_operate_rx: Receiver<KernelOperate>,
) {
    let mut shutdown_idle_kernel_detect_interval =
        tokio::time::interval(std::time::Duration::from_secs(120));
    let mut output_rx = kernel_ctx.kernel_ws_conn.rsp.subscribe();
    loop {
        // let start = std::time::Instant::now();
        tokio::select! {
            // #[cfg(feature = "fifo")]
            Ok(resp) = output_rx.recv() => {
                kernel_ctx.handle_kernel_execute_resp(resp).await;
            }
            Some(req) = req_receiver.recv() => {
                if matches!(kernel_ctx.state, State::Idle) || matches!(req.content, kernel_common::Content::InputReply { .. }) {
                    kernel_ctx.update(State::Running(req.header.cell_id.clone()));
                    kernel_ctx.header.cell_id = req.header.cell_id.clone();
                    kernel_ctx.send_req_to_kernel(req).await;
                } else {
                    // tracing::debug!("kernel is busy push cur req to pending: {:?}", req.header);
                    kernel_ctx.pending_req.push_back(req);
                }
                // debug!("after req_receiver.recv() {:?}", start.elapsed());
            }
            Some(kernel_operate) = kernel_operate_rx.recv() => {
                let is_break = kernel_ctx.handle_kernel_operate(kernel_operate).await;
                if is_break {
                    break;
                }
            },
            _ = shutdown_idle_kernel_detect_interval.tick() => {
                if kernel_ctx.handle_shutdown_idle_kernel_callback() {
                    break;
                }
            }
        }
    }

    if let Err(err) = kernel_ctx
        .kernel_ws_conn
        .req
        .send(Message {
            content: Content::ShutdownRequest { restart: false },
            ..Default::default()
        })
        .await
    {
        tracing::error!("(maybe idp_kernel core dump) {err}");
    }
    kernel_ctx.shutdown();
    #[cfg(feature = "fifo")]
    output_handle.abort();

    if let Err(err) = kernel_entry_ops_tx
        .send(KernelEntryOps::Delete(kernel_ctx.header.inode()))
        .await
    {
        tracing::error!("{err}");
    }
    // Ok(())
}

impl KernelCtx {
    fn handle_shutdown_idle_kernel_callback(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        if now < self.kernel_shutdown_time {
            return false;
        }
        tracing::warn!("shutdown idle kernel");
        let team_id = self.header.team_id;
        let project_id = self.header.project_id;
        let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
        let python_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);
        std::process::Command::new(python_path)
            .arg("-c")
            .arg("__import__('baihai_aid').update()")
            .spawn()
            .unwrap();
        true
    }
}
