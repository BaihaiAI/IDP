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

use kernel_common::typedef::CellId;
use kernel_common::Content;
use kernel_common::Message;
use tokio::sync::oneshot;
use tracing::error;

use super::kernel_state::State;

#[derive(Debug)]
pub enum KernelOperate {
    Interrupt,
    Shutdown { is_core_dump: bool },

    GetState(oneshot::Sender<State>),
    GetPendingReq(oneshot::Sender<Vec<CellId>>),

    Pause,
    Resume,
}

impl super::KernelCtx {
    // return whether break loop
    pub async fn handle_kernel_operate(&mut self, kernel_operate: KernelOperate) -> bool {
        let kernel_operate_str = format!("{kernel_operate:?}");
        let start = std::time::Instant::now();
        let mut is_break = false;
        match kernel_operate {
            KernelOperate::Interrupt => {
                tracing::info!("kernel_manage send InterruptRequest");
                // if kernel_manage's IP is same as kernel, we guest it is deploy on standalone version
                // but only parent process send SIGINT to kernel can propagate to python thread
                #[cfg(not)]
                if self.kernel_info.ip == std::net::Ipv4Addr::LOCALHOST {
                    debug_assert_ne!(self.kernel_info.pid, 0);
                    debug_assert_ne!(self.kernel_info.pid, u32::MAX);
                    unsafe {
                        libc::kill(self.kernel_info.pid as _, libc::SIGINT);
                    }
                    return false;
                }
                self.send_req_to_kernel(Message {
                    content: Content::InterruptRequest,
                    ..Default::default()
                })
                .await;
            }
            KernelOperate::Shutdown { is_core_dump } => {
                tracing::info!("KernelOperate::Shutdown");
                if is_core_dump {
                    self.core_dump_cell_id = match self.state {
                        State::Running(ref cell_id) => Some(cell_id.clone()),
                        _ => Some(self.header.cell_id.clone()),
                    };
                }
                is_break = true;
            }
            KernelOperate::GetState(tx) => {
                // if client cancel req, receiver in handler would drop by hyper
                if let Err(err) = tx.send(self.state.clone()) {
                    error!("{err:?}");
                }
            }
            KernelOperate::GetPendingReq(tx) => {
                if let Err(err) = tx.send(
                    self.pending_req
                        .iter()
                        .map(|req| req.header.cell_id.clone())
                        .collect(),
                ) {
                    error!("{err:?}");
                }
            }
            KernelOperate::Pause => match self.state {
                State::Idle => {
                    self.update(State::Paused(None));
                }
                State::Running(ref cell_id) => {
                    let cell_id = cell_id.clone();
                    self.update(State::Paused(Some(cell_id)));
                }
                _ => {
                    tracing::error!("invalid state");
                }
            },
            KernelOperate::Resume => match self.state {
                State::Paused(ref running_cell_id_opt) => match running_cell_id_opt {
                    Some(cell_id) => {
                        let cell_id = cell_id.clone();
                        self.update(State::Running(cell_id));
                    }
                    None => self.update(State::Idle),
                },
                _ => {
                    tracing::error!("invalid state");
                }
            },
        }
        let time_cost = start.elapsed();
        if time_cost > std::time::Duration::from_millis(100) {
            tracing::warn!("op = {kernel_operate_str} slow query {:?}", time_cost);
        }
        is_break
    }
}
