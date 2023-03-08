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

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;
use kernel_common::kubernetes_client::PodContainerStatus;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::broadcast;
use tracing::info;

use super::manual_run_job_instance::RunNodeType;
use super::prelude::JobInstanceId;
use crate::app_context::DB_POOL;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Req {
    pub job_instance_id: JobInstanceId,
}
#[derive(Serialize)]
pub struct RunStatusRsp {
    run_node_type: RunNodeType,
    #[serde(flatten)]
    inner: RunStatus,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunStatus {
    pub job_instance_id: JobInstanceId,
    pub job_instance_status: JobStatus,
    pub nodes_status: Vec<NodeStatusEntry>,
    pub end_time: chrono::NaiveDateTime,
}

/// read committed
#[cfg(not)]
#[derive(Debug, sqlx::FromRow)]
struct A {
    transaction_isolation: String,
}

impl RunStatus {
    pub async fn from_db(job_instance_id: JobInstanceId) -> Result<Self, ErrorTrace> {
        let pg_pool = &*DB_POOL;
        // dbg!(sqlx::query_as::<_, A>("SHOW TRANSACTION ISOLATION LEVEL").fetch_one(&mut tx).await?);
        let db_rsp = sqlx::query!(
            "select status, nodes_status, end_time from idp_model_job_instance where job_instance_id=$1 for share",
            job_instance_id
        )
        .fetch_one(pg_pool)
        .await?;
        let job_instance_status = JobStatus::from_str(&db_rsp.status);
        let nodes_status = serde_json::from_value(db_rsp.nodes_status)?;

        Ok(Self {
            job_instance_id,
            job_instance_status,
            nodes_status,
            end_time: db_rsp.end_time,
        })
    }

    pub async fn to_db_with_transaction(
        &self,
        mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), ErrorTrace> {
        let job_instance_id = self.job_instance_id;
        let nodes_status = serde_json::to_value(self.nodes_status.clone())?;
        let job_status = format!("{:?}", self.job_instance_status);
        info!("to_db: {job_instance_id} job_status={job_status}");
        let res = sqlx::query(&format!("select 1 from idp_model_job_instance where job_instance_id={job_instance_id} for update")).execute(&mut transaction).await?;
        debug_assert!(res.rows_affected() == 1);

        let res = sqlx::query!(
            "update idp_model_job_instance
        set status=$2,nodes_status=$3,end_time=$4
        where job_instance_id=$1",
            self.job_instance_id,
            job_status,
            nodes_status,
            self.end_time
        )
        .execute(&mut transaction)
        .await?;
        debug_assert!(res.rows_affected() == 1);
        transaction.commit().await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn to_db(&self) -> Result<(), ErrorTrace> {
        let pg_pool = &*DB_POOL;
        let transaction = pg_pool.begin().await?;
        self.to_db_with_transaction(transaction).await
    }

    pub fn node(&self, node_id: &str) -> Result<&NodeStatusEntry, ErrorTrace> {
        match self
            .nodes_status
            .iter()
            .find(|node| node.node_id == node_id)
        {
            Some(x) => Ok(x),
            None => {
                dbg!(&self, node_id);
                Err(ErrorTrace::new("node_id not found"))
            }
        }
    }

    pub fn node_mut(&mut self, node_id: &str) -> Result<&mut NodeStatusEntry, ErrorTrace> {
        match self
            .nodes_status
            .iter_mut()
            .find(|node| node.node_id == node_id)
        {
            Some(x) => Ok(x),
            None => {
                dbg!(node_id);
                Err(ErrorTrace::new("node_id not found"))
            }
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum JobStatus {
    Running,
    Success,
    Fail,
    Canceling,
}

impl JobStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Running" => Self::Running,
            "Success" => Self::Success,
            "Fail" => Self::Fail,
            "Canceling" => Self::Canceling,
            _ => unreachable!(),
        }
    }
    pub fn is_finish(&self) -> bool {
        matches!(self, Self::Success | Self::Fail | Self::Canceling)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatusEntry {
    pub node_id: String,
    pub status: NodeStatus,
    pub start_at: u64,
    pub end_at: Option<u64>,
    pub duration: u64,
}

impl std::fmt::Debug for NodeStatusEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.node_id)
            .field("status", &self.status)
            .finish()
    }
}

impl NodeStatusEntry {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            status: NodeStatus::Pending,
            start_at: 0,
            end_at: None,
            duration: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NodeStatus {
    Pending,
    /// pod Pending or ContainerCreating
    Preparing,
    Running,
    Success,
    Fail,
    FailCancel,
    FailTimeout,
    FailOutOfMemory,
    FailPodNotFound,
    FailImagePull,
    FailParentFail,
    FailCreatePod, // FailCancel
}

impl NodeStatus {
    pub fn fail(&self) -> bool {
        matches!(
            self,
            Self::Fail
                | Self::FailCancel
                | Self::FailTimeout
                | Self::FailOutOfMemory
                | Self::FailPodNotFound
                | Self::FailImagePull
                | Self::FailParentFail
                | Self::FailCreatePod
        )
    }

    pub fn is_finish(&self) -> bool {
        self.fail() || matches!(self, Self::Success)
    }

    pub fn update_from_container_status(&mut self, status: PodContainerStatus) -> bool {
        let new_status = match status {
            PodContainerStatus::Failed | PodContainerStatus::CreateContainerError => Self::Fail,
            PodContainerStatus::OOMKilled => Self::FailOutOfMemory,
            PodContainerStatus::ErrImagePull => Self::FailImagePull,
            PodContainerStatus::Running => Self::Running,
            PodContainerStatus::ContainerCreating | PodContainerStatus::Pending => Self::Preparing,
            PodContainerStatus::Succeeded => Self::Success,
            PodContainerStatus::CreateContainerConfigError => Self::FailCancel,
            PodContainerStatus::Terminating => return false,
            PodContainerStatus::Applying | PodContainerStatus::Closed => unreachable!(),
        };
        // info!("{status:?} {new_status:?}");
        if *self != new_status {
            *self = new_status;
            true
        } else {
            false
        }
    }
}

pub static RUN_STATUS_MSG_QUEUE: Lazy<broadcast::Sender<RunStatus>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(65536);
    tx
});

#[cfg(not)]
#[tracing::instrument]
pub fn spawn_run_status_msg_queue_write_db_consumer() {
    tokio::spawn(async {
        let mut rx = RUN_STATUS_MSG_QUEUE.subscribe();
        while let Ok(run_status) = rx.recv().await {
            if let Err(err) = run_status.to_db().await {
                error!("{err} {run_status:?}")
            }
        }
        error!("spawn_run_status_msg_queue_write_db_consumer stop!!!");
    });
}

pub async fn status(Query(req): Query<Req>) -> Result<Rsp<RunStatusRsp>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    let pg_pool = &*DB_POOL;
    let db_rsp = sqlx::query!(
        "select run_node_type from idp_model_job_instance where job_instance_id=$1 limit 1 for share",
        job_instance_id
    )
    .fetch_one(pg_pool)
    .await?;
    let run_node_type = RunNodeType::from_str(&db_rsp.run_node_type);
    Ok(Rsp::success(RunStatusRsp {
        run_node_type,
        inner: RunStatus::from_db(job_instance_id).await?,
    }))
}

#[cfg(not)]
pub async fn status_old(Query(req): Query<Req>) -> Result<Rsp<RunStatus>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    if let Some(status) = JOB_INSTANCE_STATUS.read().await.get(&job_instance_id) {
        let now = os_utils::get_timestamp();
        let nodes_status = status
            .nodes_status
            .values()
            .cloned()
            .map(|mut status| {
                if status.start_at != 0 && status.end_at.is_none() {
                    status.duration = now - status.start_at;
                }
                status
            })
            .collect();
        Ok(Rsp::success(RunStatus {
            job_instance_id,
            job_instance_status: status.job_instance_status.clone(),
            nodes_status,
        }))
    } else {
        let pg_pool = &*DB_POOL;
        let db_rsp = sqlx::query_as!(
            DbRsp,
            "select status, nodes_status from idp_model_job_instance where job_instance_id=$1",
            req.job_instance_id
        )
        .fetch_optional(pg_pool)
        .await?;
        let db_rsp = match db_rsp {
            Some(db_rsp) => db_rsp,
            None => return Err(ErrorTrace::new("id not found")),
        };
        #[cfg(not)]
        if db_rsp.status == "Success" || db_rsp.status == "Fail" {
            return Err(ErrorTrace::new("job not run, is job in init state?"));
        }
        let nodes_status = serde_json::from_value::<Vec<NodeStatusEntry>>(db_rsp.nodes_status)?;
        if nodes_status.is_empty() {
            tracing::warn!("job_instance_id {job_instance_id} run abnormal");
        }
        Ok(Rsp::success(RunStatus {
            job_instance_id,
            job_instance_status: db_rsp.status,
            nodes_status,
        }))
    }
}
