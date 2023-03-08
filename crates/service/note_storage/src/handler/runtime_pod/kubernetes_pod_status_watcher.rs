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

use business::business_term::ProjectId;
use kernel_common::kubernetes_client;
use kernel_common::runtime_pod_status::PodStatusRsp;
use kernel_common::runtime_pod_status::RuntimeStatus;
use kernel_common::spawn_kernel_process::Resource;
use kubernetes_client::NotifyType;
use kubernetes_client::PodContainerStatus;
use kubernetes_client::PodListWatchRsp;
use kubernetes_client::PodStatusPhase;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;
use tracing::error;
use tracing::info;

#[derive(Clone)]
pub struct RuntimeStatusMap(pub Arc<RwLock<HashMap<ProjectId, ProjectRuntimeEntry>>>);

impl RuntimeStatusMap {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn project_pod_status(&self, project_id: ProjectId) -> RuntimeStatus {
        match self.0.read().await.get(&project_id) {
            Some(status) => status.inner.status.clone(),
            None => RuntimeStatus::default(),
        }
    }
    #[cfg(not)]
    pub async fn set_wait_pending_flag(&self, project_id: ProjectId, flag: bool) {
        if let Some(x) = self.0.write().await.get_mut(&project_id) {
            x.inner.wait_pending = flag;
        }
    }

    #[cfg(not)]
    pub async fn get_wait_pending_flag(&self, project_id: ProjectId) -> bool {
        self.0
            .read()
            .await
            .get(&project_id)
            .map(|x| x.wait_pending)
            .unwrap_or_default()
    }
}

pub struct ProjectRuntimeEntry {
    pub inner: PodStatusRsp,
    pub sse_notify_tx: Sender<PodStatusRsp>,
}

impl Default for ProjectRuntimeEntry {
    fn default() -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(65536);
        Self {
            inner: PodStatusRsp::default(),
            sse_notify_tx: tx,
        }
    }
}

impl ProjectRuntimeEntry {
    pub fn update_status_and_notify_sse(&mut self, status: RuntimeStatus) {
        self.inner.status = status;
        if let Err(err) = self.sse_notify_tx.send(self.inner.clone()) {
            // if no frontend connected, would get error channel closed
            tracing::debug!("{err}");
        }
    }
}

struct PanicGuard;

impl Drop for PanicGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            error!(
                "FATAL: {:?} thread panic! exit process now",
                std::thread::current().name()
            );
            std::process::exit(1);
        }
    }
}

pub async fn spawn_runtime_pod_watcher(runtime_status_map: RuntimeStatusMap) {
    if !business::kubernetes::is_k8s() {
        return;
    }
    let _panic_guard = PanicGuard;

    let api_url = kubernetes_client::k8s_api_server_base_url();
    let volcano_queue_name = kubernetes_client::volcano_queue();

    let client = kubernetes_client::k8s_api_client();
    loop {
        let rsp = client
            .get(&api_url)
            .query(&[
                ("watch", "true"),
                (
                    "labelSelector",
                    &format!("volcano.sh/queue-name={volcano_queue_name}"),
                ),
            ])
            .send()
            .await
            .expect("req to K8s api server");

        if !rsp.status().is_success() {
            let rsp_status = rsp.status();
            let rsp_text = rsp.text().await.expect("rsp.text()");
            error!("request to K8s api server fail {rsp_status} {rsp_text}");
            std::process::exit(1);
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
        let mut stream = futures::AsyncBufReadExt::lines(stream);

        let stream_start = std::time::Instant::now();
        while let Some(data_res) = futures_util::StreamExt::next(&mut stream).await {
            let msg = match data_res {
                Ok(x) => x,
                Err(err) => {
                    error!("stream.next() {err}");
                    continue;
                }
            };
            let notify = serde_json::from_str::<PodListWatchRsp>(&msg)
                .unwrap_or_else(|_| panic!("{}", msg.to_string()));

            let pod_name = &notify.object.metadata.name;
            let notify_type = &notify.type_;
            let status = &notify.object.status.phase;
            info!(
                "{pod_name} notify_type={notify_type:?} status={status:?} container_statuses={:?}",
                notify.object.status.container_statuses
            );

            // e.g. idp-kernel-b-123-45-pipeline-8-job-0
            if pod_name.starts_with("idp-kernel") && !pod_name.contains("pipeline") {
                handle_runtime_pod_watch_notify(notify, &runtime_status_map).await;
            } else if pod_name.starts_with("visual") {
                handle_visual_model_pod_watch_notify(notify).await;
            }
        }
        error!(
            "receive EOF from K8s api server, stream duration {:?}, reconnecting...",
            stream_start.elapsed()
        );
    }
}

async fn handle_runtime_pod_watch_notify(
    notify: PodListWatchRsp,
    runtime_status_map: &RuntimeStatusMap,
) {
    let pod_name = &notify.object.metadata.name;
    let notify_type = notify.type_;
    // Input:  idp-kernel-b-executor-107-runtime-job-0
    // Output: 107
    let project_id = pod_name
        .rsplit('-')
        .skip(3)
        .take(1)
        .next()
        .unwrap()
        .parse::<ProjectId>()
        .unwrap();
    let mut map = runtime_status_map.0.write().await;
    let mut status_entry = map.entry(project_id).or_default();
    let last_status = &status_entry.inner.status;
    let mut curr_status = notify.object.status();
    match notify_type {
        NotifyType::Added => {
            if curr_status == PodContainerStatus::Pending {
                for container in notify.object.spec.containers {
                    if !container.name.starts_with("idp-kernel") {
                        continue;
                    }
                    if let Some(resource) = container.resources.limits {
                        status_entry.inner.resource = {
                            Resource {
                                memory: resource.memory_gb(),
                                num_cpu: resource.cpu_cores(),
                                num_gpu: resource.gpu(),
                                ..Default::default()
                            }
                        };
                    }
                    break;
                }
                status_entry.inner.pending_start_at = os_utils::get_timestamp();
            }
        }
        NotifyType::Modified => {}
        NotifyType::Deleted => {
            if let Err(err) =
                crate::handler::kernel::shutdown_by_project_id_and_kernel_idpnb_starts_with_path(
                    project_id, "",
                )
                .await
            {
                error!("{err}");
            };
            curr_status = RuntimeStatus::Closed;
        }
    }

    if *last_status != curr_status {
        status_entry.update_status_and_notify_sse(curr_status);
    } else {
        error!("{pod_name} status no change {curr_status:?}");
    }
}

/// current pod ttlSecondsAfterFinished: 15
async fn handle_visual_model_pod_watch_notify(notify: PodListWatchRsp) {
    use crate::handler::visual_modeling::prelude::RunStatus;
    use crate::handler::visual_modeling::prelude::RUN_STATUS_MSG_QUEUE;
    let status = notify.object.status();

    let container = &notify.object.spec.containers[0];
    debug_assert!(container.name.starts_with("visual"));
    let mut envs = std::collections::HashMap::new();
    for env in &container.env {
        if env.value.is_empty() {
            continue;
        }
        envs.insert(env.name.clone(), env.value.clone());
    }
    let job_instance_id = envs["job_instance_id"].parse::<i32>().unwrap();
    let node_id = &envs["node_id"];
    let mut run_status = match RunStatus::from_db(job_instance_id).await {
        Ok(x) => x,
        Err(_) => {
            return;
        }
    };
    let node = match run_status.node_mut(node_id) {
        Ok(node) => node,
        Err(err) => {
            error!(
                "dirty data in db job_instance_id={job_instance_id} nodes_status is empty {err}"
            );
            return;
        }
    };
    let has_change = node.status.update_from_container_status(status);
    let is_finish = node.status.is_finish();
    if has_change {
        /*
        if !has_change && is_delete {
            node.status = NodeStatus::FailCancel;
        }
        */
        if RUN_STATUS_MSG_QUEUE.send(run_status).is_err() {
            error!("RUN_STATUS_MSG_QUEUE.send(run_status).is_err()");
        }
    }

    let log_path = crate::handler::visual_modeling::run_log::NodeLogReq {
        job_instance_id,
        team_id: envs["team_id"].parse().unwrap(),
        node_id: node_id.clone(),
    }
    .log_path();
    if !matches!(
        notify.object.status.phase,
        PodStatusPhase::Failed | PodStatusPhase::Succeeded
    ) {
        return;
    }
    if !is_finish {
        return;
    }
    if std::path::Path::new(&log_path).exists() {
        return;
    }

    let pod_name = notify.object.metadata.name;
    let url = format!(
        "{}/{pod_name}/log",
        kubernetes_client::k8s_api_server_base_url()
    );
    let start = std::time::Instant::now();
    let rsp = kubernetes_client::k8s_api_client()
        .get(&url)
        .send()
        .await
        .unwrap();
    let status = rsp.status();
    if !status.is_success() {
        tracing::error!("K8s log {pod_name} {status}");
        return;
    }
    let rsp = rsp.text().await.unwrap();
    tokio::fs::write(log_path, rsp).await.unwrap();
    info!("save {pod_name} log time cost = {:?}", start.elapsed());
}
