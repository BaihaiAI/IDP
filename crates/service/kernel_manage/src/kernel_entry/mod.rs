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

pub mod handle_kernel_execute_resp;
pub mod kernel_main_loop;
pub mod kernel_operate;
pub mod kernel_state;
mod prelude {
    pub use kernel_common::Content;
    pub use kernel_common::Message;
    pub use tokio::sync::broadcast;
    pub use tokio::sync::mpsc;
    pub use tokio::sync::mpsc::channel;
    pub use tokio::sync::mpsc::Receiver;
    pub use tokio::sync::mpsc::Sender;
    pub use tokio::sync::oneshot;
}

use kernel_common::spawn_kernel_process::Resource;
use kernel_common::spawn_kernel_process::SpawnKernel;
use kernel_common::typedef::CellId;
use kernel_common::Header;
use kernel_common::KernelInfo;
use kernel_state::State;
use prelude::*;
use tracing::debug;

use self::kernel_operate::KernelOperate;
use crate::app_context::KernelWsConn;
use crate::error::Error;

/// one ipynb map to one kernel
#[derive(Debug)]
pub struct KernelEntry {
    pub req_sender: Sender<Message>,
    pub kernel_operate_tx: mpsc::Sender<KernelOperate>,

    // handle would abort/break before Kernel Drop
    // pub output_handle: JoinHandle<()>,
    // connection_handle: JoinHandle<Result<(), Error>>,
    pub inode: u64,
    pub kernel_info: KernelInfo,
    /// this field only used in kernel restart?
    pub header: kernel_common::Header,
    /// this field only used in kernel restart
    pub resource: Resource,
}

#[cfg(not)]
impl Drop for KernelEntry {
    fn drop(&mut self) {
        // self.output_handle.abort();
        self.connection_handle.abort();
    }
}

impl KernelEntry {
    pub async fn new(
        header: Header,
        resource: Resource,
        ctx: &crate::AppContext,
    ) -> Result<Self, Error> {
        let start = std::time::Instant::now();
        tracing::info!("--> KernelEntry::new");
        let inode = header.inode();

        #[cfg(feature = "fifo")]
        let pid = kernel_common::spawn_kernel_process::spawn_kernel_process(SpawnKernel {
            header: header.clone(),
            resource: resource.clone(),
        })?;
        ctx.output_to_ws_sender.send(kernel_common::Message {
            header: header.clone(),
            request: None,
            content: kernel_common::Content::StartKernel {},
        })?;
        #[cfg(feature = "tcp")]
        {
            let header = header.clone();
            let resource = resource.clone();
            kernel_common::spawn_kernel_process::req_submitter_spawn_kernel(SpawnKernel {
                header,
                resource,
            })
            .await?;
        };
        #[cfg(feature = "fifo")]
        let kernel_info = KernelInfo { pid, ..kernel_info };

        #[cfg(feature = "tcp")]
        let kernel_ws_conn = {
            let mut retry = 0;
            loop {
                let (tx, rx) = tokio::sync::oneshot::channel();
                ctx.kernel_ws_conn_take.send((inode, tx)).await?;
                if let Some(kernel) = rx.await? {
                    break kernel;
                }
                retry += 1;
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                if retry % 5 == 0 {
                    tracing::info!(
                        "retry:{retry} get kernel {inode} project_id={} path={} is_pipeline={}",
                        header.project_id,
                        header.path,
                        header.pipeline_opt.is_some()
                    );
                }
                if retry == 50 {
                    return Err(Error::new("timeout: no enough resource"));
                }
            }
        };

        #[cfg(feature = "fifo")]
        let write_to_kernel_fifo_path = kernel_common::transport::kernel_req_pipe(pid as _);
        // if pod restart want to restore paused kernel, these would not success
        #[cfg(feature = "fifo")]
        let req_to_kernel = match tokio::time::timeout(
            std::time::Duration::from_secs(10),
            tokio::fs::OpenOptions::new()
                .write(true)
                .open(&write_to_kernel_fifo_path),
        )
        .await
        {
            Ok(req_to_kernel) => req_to_kernel?,
            Err(_) => {
                return Err(Error::new("timeout: no enough resource"));
            }
        };

        // fifo: this closure only used to convert sync serde_json::from_reader code to async channel to loop-select
        // spawn_blocking alternative block_in_place
        #[cfg(not)]
        let output_handle = tokio::task::spawn_blocking(move || {
            loop {
                let read_from_kernel = std::fs::OpenOptions::new()
                    .read(true)
                    .open(kernel_common::transport::kernel_rsp_pipe(pid, inode))
                    .unwrap();
                for msg in serde_json::Deserializer::from_reader(read_from_kernel)
                    .into_iter::<kernel_common::Message>()
                {
                    let msg = match msg {
                        Ok(msg) => msg,
                        Err(err) => {
                            tracing::error!("Deserializer::from_reader {err}");
                            continue;
                        }
                    };
                    if let Err(err) = output_tx.send(msg) {
                        tracing::error!("{err}");
                    }
                }
                tracing::warn!(
                    "read_from_kernel get EOF! maybe kernel paused, blocking to reconnect..."
                );
            }
        });

        let (req_sender, req_receiver) = channel::<kernel_common::Message>(100);
        // watch::channel is single producer multi consumer similar to broadcast::channel(1)
        // but watch channel not satisfied Send when share between future
        let (kernel_operate_tx, kernel_operate_rx) = channel::<KernelOperate>(15);

        let kernel_info = kernel_ws_conn.kernel_info.clone();
        let execute_record_db = ctx.execute_record_db.clone();
        tokio::spawn({
            debug!("enter tokio::spawn, time cost = {:?}", start.elapsed());
            // start of **clone channel from outer scope**
            let output_to_ws_sender = ctx.output_to_ws_sender.clone();
            let kernel_entry_ops_tx = ctx.kernel_entry_ops_tx.clone();
            let header = header.clone();
            // end of **clone channel from outer scope**

            debug!("after write to kernel, time cost = {:?}", start.elapsed());

            let state = State::Idle;
            let pending_req = std::collections::VecDeque::<kernel_common::Message>::new();

            let shutdown_idle_interval_minute =
                match std::env::var("SHUTDOWN_IDLE_KERNEL_DURATION_MINUTE") {
                    Ok(val) => val,
                    Err(_) => {
                        if header.pipeline_opt.is_some() {
                            "15".to_string()
                        } else {
                            "120".to_string()
                        }
                    }
                };
            let shutdown_idle_interval_minute = shutdown_idle_interval_minute
                .parse::<u64>()
                .unwrap_or_else(|_| {
                    panic!("{shutdown_idle_interval_minute} parse to minute error")
                });
            let shutdown_idle_interval_duration =
                std::time::Duration::from_secs(shutdown_idle_interval_minute * 60);

            let kernel_ctx = KernelCtx {
                kernel_ws_conn,
                state,
                pending_req,
                output_to_ws_sender,
                header,
                err_cell_ids: std::collections::HashSet::new(),
                cell_update: std::collections::HashMap::new(),

                shutdown_idle_interval_duration,
                kernel_shutdown_time: std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    + shutdown_idle_interval_duration,
                core_dump_cell_id: None,
                execute_record_db,
            };

            kernel_main_loop::kernel_main_loop(
                kernel_ctx,
                kernel_entry_ops_tx,
                req_receiver,
                kernel_operate_rx,
            )
        });
        tracing::info!("<-- KernelEntry::new, time cost = {:?}", start.elapsed());
        Ok(Self {
            req_sender,
            kernel_operate_tx,
            kernel_info,
            header,
            inode,
            resource,
        })
    }
}

pub struct KernelCtx {
    // #[cfg(feature = "fifo")]
    // req_to_kernel: tokio::fs::File,
    kernel_ws_conn: KernelWsConn,
    state: State,
    pending_req: std::collections::VecDeque<kernel_common::Message>,
    output_to_ws_sender: tokio::sync::broadcast::Sender<kernel_common::Message>,
    header: Header,
    err_cell_ids: std::collections::HashSet<String>,
    cell_update: std::collections::HashMap<CellId, common_model::entity::cell::Updates>,

    shutdown_idle_interval_duration: std::time::Duration,
    kernel_shutdown_time: std::time::Duration,

    core_dump_cell_id: Option<String>, // kernel_info: kernel_common::KernelInfo,
    execute_record_db: sled::Db,
}

impl KernelCtx {
    #[cfg(feature = "fifo")]
    async fn send_req_to_kernel(&mut self, msg: kernel_common::Message) {
        if let Err(err) =
            tokio::io::AsyncWriteExt::write_all(&mut self.req_to_kernel, &msg.to_json()).await
        {
            tracing::error!("{err}");
        }
    }

    #[cfg(feature = "tcp")]
    async fn send_req_to_kernel(&mut self, msg: kernel_common::Message) {
        if let Err(err) = self.kernel_ws_conn.req.send(msg).await {
            tracing::error!("{err}");
        }
    }

    fn shutdown(&mut self) {
        tracing::info!("--> shutdown");

        // send stop to all pending req
        while let Some(req) = self.pending_req.pop_front() {
            let mut rsp = req;
            rsp.content = Content::ReplyOnStop {};
            if let Err(err) = self.output_to_ws_sender.send(rsp) {
                tracing::error!("{err}");
            };
        }
        if let Some(ref cell_id) = self.core_dump_cell_id {
            self.header.cell_id = cell_id.clone();
            // core dump or OOM kill occur
            let msg = kernel_common::Message {
                header: self.header.clone(),
                content: kernel_common::Content::RuntimeError {
                    message: "kernel crash".to_string(),
                },
                ..Default::default()
            };
            if let Err(err) = self.output_to_ws_sender.send(msg) {
                tracing::error!("{err}");
            };
        }

        tracing::info!("<-- shutdown");
    }
}
