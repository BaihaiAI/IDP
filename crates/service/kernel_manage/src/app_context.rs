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

use std::collections::HashMap;
use std::sync::Arc;

use err::ErrorTrace;
use kernel_common::spawn_kernel_process::Resource;
use kernel_common::typedef::Inode;
use kernel_common::Header;
use kernel_common::Message;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::RwLock;
use tracing::error;

use crate::kernel_entry::KernelEntry;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub broadcast_output_to_all_ws_write: tokio::sync::broadcast::Sender<kernel_common::Message>,

    // kernel websocket connection
    pub kernel_ws_conn_insert: mpsc::Sender<KernelWsConn>,
    pub kernel_ws_conn_take: mpsc::Sender<(Inode, oneshot::Sender<Option<KernelWsConn>>)>,

    pub kernel_entry_ops_tx: mpsc::Sender<KernelEntryOps>,
    // pub kernel_entry_get: mpsc::Sender<(Inode, oneshot::Sender<Option<Arc<KernelEntry>>>)>,
    // pub kernel_entry_delete: mpsc::Sender<Inode>,
    // pub kernel_entry_insert: mpsc::Sender<KernelEntry>,
    pub execute_record_db: sled::Db,
}

#[derive(Debug)]
pub enum KernelEntryOps {
    Get(Inode, oneshot::Sender<Option<Arc<KernelEntry>>>),
    GetAll(oneshot::Sender<Vec<Arc<KernelEntry>>>),
    Delete(Inode),
    Insert {
        header: Box<Header>,
        resource: Resource,
        ctx: AppContext,
        tx: oneshot::Sender<Result<Arc<KernelEntry>, ErrorTrace>>,
    },
}

impl KernelEntryOps {
    fn variant(&self) -> &'static str {
        match self {
            KernelEntryOps::Get(_, _) => "Get",
            KernelEntryOps::GetAll(_) => "GetAll",
            KernelEntryOps::Delete(_) => "Delete",
            KernelEntryOps::Insert { .. } => "Insert",
        }
    }
}

#[derive(Clone, Debug)]
pub struct KernelWsConn {
    pub inode: u64,
    pub kernel_info: kernel_common::KernelInfo,
    // ws_write: &mut SplitSink<WebSocketStream<Upgraded>, Message>,
    // mut ws_read: SplitStream<WebSocketStream<Upgraded>>,
    pub req: mpsc::Sender<Message>,
    pub rsp: tokio::sync::broadcast::Sender<Message>,
}

impl AppContext {
    pub async fn get_kernel_by_inode(
        &self,
        inode: Inode,
    ) -> Result<Option<Arc<KernelEntry>>, crate::Error> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.kernel_entry_ops_tx
            .send(KernelEntryOps::Get(inode, tx))
            .await?;
        Ok(rx.await?)
    }
    pub fn new() -> Self {
        let sled_kv_db = match sled::open("kernel_manage.db") {
            Ok(db) => db,
            Err(err) => {
                tracing::error!(
                    "crate sled db on current_dir={:?} fail {err}",
                    std::env::current_dir().unwrap()
                );
                sled::open("/tmp/kernel_manage.db").expect("open /tmp/kernel_manage.db failed")
            }
        };

        let (output_to_ws_sender, _) = tokio::sync::broadcast::channel(50000);
        // only kernel_connect websocket can insert
        let (kernel_ws_conn_insert_tx, mut kernel_ws_conn_insert_rx) =
            mpsc::channel::<KernelWsConn>(1000);
        let (kernel_ws_conn_take_tx, mut kernel_ws_conn_take_rx) =
            mpsc::channel::<(u64, oneshot::Sender<Option<KernelWsConn>>)>(1000);

        // let (kernel_entry_get, mut kernel_entry_get_rx) = mpsc::channel::<(Inode, oneshot::Sender<Arc<KernelEntry>>)>(10);
        // let (kernel_entry_get_all, mut kernel_entry_get_all_rx) = mpsc::channel::<(Inode, oneshot::Sender<Vec<Arc<KernelEntry>>>)>(25);
        // let (kernel_entry_delete, mut kernel_entry_delete_rx) = mpsc::channel::<Inode>(10);
        // let (kernel_entry_insert, mut kernel_entry_insert_rx) = mpsc::channel::<KernelEntry>(10);
        let (kernel_entry_ops_tx, mut kernel_entry_ops_rx) = mpsc::channel::<KernelEntryOps>(200);

        tokio::spawn(async move {
            let mut kernel_ws_conn_mapping = HashMap::<u64, KernelWsConn>::new();
            // can't use RwLock here, use RwLock run_all_cell sometime 3st cell would run before 2nd cell
            let kernel_entry_mapping =
                Arc::new(RwLock::new(HashMap::<u64, Arc<KernelEntry>>::new()));
            loop {
                tokio::select! {
                    Some(kernel) = kernel_ws_conn_insert_rx.recv() => {
                        tracing::warn!("kernel {} already start, shutdown old one replace with new connection", kernel.inode);
                        // if kernel_ws_conn_mapping.contains_key(&kernel.inode) {
                        //     continue;
                        // }
                        if let Some(old_kernel) = kernel_ws_conn_mapping.insert(kernel.inode, kernel) {
                            if old_kernel.req.send(kernel_common::Message { content: kernel_common::Content::ShutdownRequest { restart: false }, ..Default::default() }).await.is_err() {
                                error!("old_kernel.req send shutdown error");
                            }
                        }
                    }
                    // take KernelWsConn to KernelCtx
                    Some((inode, tx)) = kernel_ws_conn_take_rx.recv() => {
                        match kernel_ws_conn_mapping.remove(&inode) {
                            Some(kernel) => {
                                tx.send(Some(kernel.clone())).unwrap();
                            },
                            None => {
                                tx.send(None).unwrap();
                            }
                        }
                    }

                    Some(op) = kernel_entry_ops_rx.recv() => {
                        // kernel_entry_ops_handler may blocking so we spawn a new task
                        // insert new kernel would wait kernel_ws_conn_take_rx write lock then dead lock
                        // so we spawn kernel insert to new task prevent block kernel_ws_conn_take_rx
                        let kernel_entry_mapping = kernel_entry_mapping.clone();
                        tokio::spawn(kernel_entry_ops_handler(kernel_entry_mapping, op));
                    }
                }
            }
        });

        Self {
            broadcast_output_to_all_ws_write: output_to_ws_sender,
            kernel_ws_conn_take: kernel_ws_conn_take_tx,
            kernel_ws_conn_insert: kernel_ws_conn_insert_tx,
            kernel_entry_ops_tx,
            execute_record_db: sled_kv_db,
        }
    }
}

async fn kernel_entry_ops_handler(
    mapping: Arc<RwLock<HashMap<u64, Arc<KernelEntry>>>>,
    op: KernelEntryOps,
) {
    let op_fmt = op.variant().to_string();
    let start = std::time::Instant::now();
    match op {
        KernelEntryOps::Get(inode, tx) => {
            let mapping = mapping.read().await;
            let kernel_opt = mapping.get(&inode).map(Clone::clone);
            if tx.send(kernel_opt).is_err() {
                tracing::error!("send back to oneshot::channel rx fail");
            }
        }
        KernelEntryOps::GetAll(tx) => {
            let mapping = mapping.read().await;
            let kernel_list = mapping
                .values()
                .filter(|x| {
                    if let Some(ref pipeline) = x.header.pipeline_opt {
                        tracing::debug!(
                            "kernel_list skip pipeline task_instance_id={}",
                            pipeline.job_instance_id
                        );
                        return false;
                    }
                    true
                })
                .map(Clone::clone)
                .collect::<Vec<_>>();
            if tx.send(kernel_list).is_err() {
                error!("send back to oneshot::channel rx fail");
            }
        }
        KernelEntryOps::Delete(inode) => {
            let mut mapping = mapping.write().await;
            match mapping.remove(&inode) {
                Some(kernel_ws_conn) => {
                    tracing::debug!("remove {:?}", kernel_ws_conn.header);
                }
                None => {
                    error!("delete {inode} fail: not found");
                }
            }
        }
        KernelEntryOps::Insert {
            header,
            resource,
            ctx,
            tx,
        } => {
            match KernelEntry::new(*header, resource, ctx).await {
                Ok(kernel) => {
                    let mut mapping = mapping.write().await;
                    let kernel = Arc::new(kernel);
                    mapping.insert(kernel.inode, kernel.clone());
                    if tx.send(Ok(kernel)).is_err() {
                        error!("KernelEntryOps::Insert tx send back err, rx closed");
                    }
                }
                Err(err) => {
                    // error!("{err:#?}");
                    if tx.send(Err(err)).is_err() {
                        error!("KernelEntryOps::Insert tx send back err, rx closed");
                    }
                }
            };
        }
    }
    let time_cost = start.elapsed();
    if time_cost > std::time::Duration::from_millis(500) {
        tracing::warn!("op = {op_fmt} slow query {:?}", time_cost);
    }
}

#[cfg(not)]
pub async fn dump_ctx_to_disk_task(ctx: AppContext) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(4));
    loop {
        let mut kernel_list = vec![];
        tracing::trace!("{}", line!());
        for (inode, kernel) in &*ctx.inode_kernel_mapping.read().await {
            tracing::trace!("{}", line!());
            let (tx, rx) = tokio::sync::oneshot::channel();
            if let Err(err) = kernel.get_kernel_state.send(tx).await {
                tracing::error!("{err}");
                // coredump_closed_kernel_list.push(*inode);
                continue;
            }
            tracing::trace!("{}", line!());
            let mut state = match rx.await {
                Ok(state) => state,
                Err(err) => {
                    tracing::error!("{err}");
                    continue;
                }
            };
            tracing::trace!("{}", line!());
            if state.is_busy() {
                state = State::Idle;
            }

            if matches!(state, State::Paused { .. }) {
                continue;
            }

            kernel_list.push(KernelDump {
                inode: *inode,
                state,
                header: kernel.header.clone(),
            });
        }
        tracing::trace!("{}", line!());
        if !kernel_list.is_empty() {
            tokio::fs::write(
                ctx_dump_to_disk_path(),
                serde_json::to_string(&kernel_list).unwrap(),
            )
            .await
            .unwrap();
        }

        interval.tick().await;
    }
}

#[cfg(not)]
pub async fn restore_ctx_from_disk(ctx: &AppContext) {
    tracing::debug!("--> restore_ctx_from_disk");
    let content = match tokio::fs::read_to_string(ctx_dump_to_disk_path()).await {
        Ok(content) => content,
        Err(err) => {
            tracing::error!("{err}");
            return;
        }
    };
    let kernel_list = match serde_json::from_str::<Vec<KernelDump>>(&content) {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("{err}");
            return;
        }
    };
    if kernel_list.is_empty() {
        return;
    }
    let mut mapping = ctx.inode_kernel_mapping.write().await;
    for KernelDump {
        inode,
        state,
        header,
    } in kernel_list
    {
        let kernel = match KernelEntry::new(header, state, ctx).await {
            Ok(kernel) => kernel,
            Err(err) => {
                tracing::error!("{err}");
                continue;
            }
        };
        mapping.insert(inode, Arc::new(kernel));
    }
}
