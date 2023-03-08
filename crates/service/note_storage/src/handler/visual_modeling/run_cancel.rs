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

use axum::Json;
use common_model::Rsp;
use err::ErrorTrace;

pub use super::prelude::*;
use super::run_status::Req;

#[tracing::instrument]
pub async fn cancel(Json(req): Json<Req>) -> Result<Rsp<()>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    let mut run_status = RunStatus::from_db(job_instance_id).await?;
    if run_status.job_instance_status.is_finish()
        || matches!(run_status.job_instance_status, JobStatus::Canceling)
    {
        return Ok(Rsp::success(()));
    }
    if run_status.nodes_status.is_empty() {
        tracing::warn!("dirty data {job_instance_id} in db nodes_status is empty");
        run_status.job_instance_status = JobStatus::Fail;
        run_status.to_db().await?;
        return Ok(Rsp::success(()));
    }
    run_status.job_instance_status = JobStatus::Canceling;
    for node in &mut run_status.nodes_status {
        if node.status == NodeStatus::Running {
            let node_id = &node.node_id;
            let rsp = crate::pojo::component::graph::delete_vc_job(job_instance_id, node_id)
                .send()
                .await?;
            let rsp = rsp.text().await?;
            tracing::info!("{job_instance_id} {node_id} {rsp}");
            if rsp.contains("notfound") {
                node.status = NodeStatus::FailPodNotFound;
            } else {
                node.status = NodeStatus::FailCancel;
            }
        }
    }
    if RUN_STATUS_MSG_QUEUE.send(run_status.clone()).is_err() {
        tracing::error!("panic RUN_STATUS_MSG_QUEUE.send(run_status).is_err()");
        run_status.job_instance_status = JobStatus::Fail;
        run_status.to_db().await?;
    }
    Ok(Rsp::success(()))
}

#[cfg(not)]
pub async fn cancel(Json(req): Json<Req>) -> Result<Rsp<()>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    RunStatus::from_db(job_instance_id);
    if !JOB_INSTANCE_STATUS
        .read()
        .await
        .contains_key(&job_instance_id)
    {
        return Err(ErrorTrace::new(&format!(
            "job instance {job_instance_id} is not running or not found"
        )));
    }
    let api_url = kubernetes_client::k8s_api_server_base_url();
    let client = kubernetes_client::k8s_api_client();
    let rsp = client
        .get(&api_url)
        .query(&[(
            "labelSelector",
            &format!(
                "volcano.sh/queue-name={}",
                kubernetes_client::volcano_queue()
            ),
        )])
        .send()
        .await?;
    debug_assert!(rsp.status().is_success());
    let pods = rsp.json::<kubernetes_client::PodListRsp>().await?.items;
    let prefix = format!(
        "visual-{}-{}-{job_instance_id}",
        &*business::kubernetes::REGION,
        &*business::kubernetes::ACCOUNT,
    );
    // TODO use try join to send parallel
    for pod in pods {
        let pod_name = pod.metadata.name;
        if !pod_name.starts_with(&prefix) {
            continue;
        }

        let container = &pod.spec.containers[0];
        debug_assert!(container.name.starts_with("visual"));
        let mut node_id = None;
        for env in &container.env {
            if env.name == "node_id" {
                node_id = Some(env.value.clone());
                break;
            }
        }

        let node_id = node_id.unwrap();
        let rsp = crate::pojo::component::graph::delete_vc_job(job_instance_id, &node_id)
            .send()
            .await?;
        tracing::info!("{job_instance_id} {node_id} {}", rsp.text().await?);
    }
    if let Some(job) = JOB_INSTANCE_STATUS.write().await.get_mut(&job_instance_id) {
        job.job_instance_status = "Canceling".to_string();
    }

    Ok(Rsp::success(()))
}
