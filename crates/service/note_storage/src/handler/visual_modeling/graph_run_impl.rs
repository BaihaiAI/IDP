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

use std::collections::HashMap;

use err::ErrorTrace;
use tracing::error;
use tracing::info;

use super::manual_run_job_instance::RunNodeType;
use super::prelude::*;
use crate::handler::visual_modeling::helper::k8s_svc::account;
pub use crate::handler::visual_modeling::helper::k8s_svc::delete_vc_job;
use crate::handler::visual_modeling::helper::k8s_svc::NewVolcanoJobReq;
use crate::handler::visual_modeling::helper::k8s_svc::DEPLOY_MODE;
use crate::handler::visual_modeling::helper::k8s_svc::K8S_SVC_BASE_URL;
use crate::handler::visual_modeling::helper::k8s_svc::PLATFORM;
use crate::pojo::component::graph::Graph;
use crate::pojo::component::graph::Node;
use crate::pojo::component::graph::PortType;

type JobInstanceId = i32;
pub struct GraphRunCtx {
    pub job_id: i32,
    pub job_instance_id: JobInstanceId,
    // task_instance_id is pod_id
    // pub task_instance_id: i32,
    pub team_id: i64,
    pub user_id: i64,
    pub project_id: i32,
    pub region: String,
    pub run_node_type: RunNodeType,
    /// start up node_id if not empty
    pub node_id: String,
}

impl Graph {
    #[tracing::instrument(skip_all)]
    pub async fn before_run(
        self,
        graph_run_ctx: &GraphRunCtx,
        transaction: sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Self, ErrorTrace> {
        let mut graph = self;
        let graph_json = serde_json::to_string(&graph).unwrap();
        let team_id = graph_run_ctx.team_id;
        let project_id = graph_run_ctx.project_id;
        let job_id = graph_run_ctx.job_id;
        let job_instance_id = graph_run_ctx.job_instance_id;

        // /store/1623008438406889472/projects/101/visual_modeling/
        let output_dir = format!("/store/{team_id}/visual_modeling/{job_instance_id}/");
        if !std::path::Path::new(&output_dir).exists() {
            std::fs::create_dir_all(&output_dir).unwrap();
        }
        std::fs::write(format!("{output_dir}graph.json"), graph_json).unwrap();
        std::fs::write(format!("{output_dir}metadata.toml"), format!("team_id={team_id}\nproject_id={project_id}\njob_id={job_id}\njob_instance_id={job_instance_id}")).unwrap();

        graph.validate()?;
        graph.topologic_sort()?;
        if matches!(
            graph_run_ctx.run_node_type,
            RunNodeType::Single | RunNodeType::Below
        ) && !graph
            .nodes
            .iter()
            .any(|node| node.id == graph_run_ctx.node_id)
        {
            return Err(ErrorTrace::new(
                "node_id not found in graph, try save before run",
            ));
        }

        // filter nodes by run_node_type
        match graph_run_ctx.run_node_type {
            RunNodeType::Single => {
                graph.nodes.retain(|x| x.id == graph_run_ctx.node_id);
                debug_assert!(graph.nodes.len() == 1);
            }
            RunNodeType::Below => {
                let nodes = graph
                    .nodes
                    .into_iter()
                    .map(|node| (node.id.clone(), node))
                    .collect::<std::collections::HashMap<_, _>>();
                let mut src_port_id_to_dst_node_id = std::collections::HashMap::new();
                for edge in &graph.edges {
                    src_port_id_to_dst_node_id
                        .insert(edge.source_port_id.clone(), edge.dst_node_id.clone());
                }

                let mut queue = std::collections::VecDeque::new();
                queue.push_back(nodes[&graph_run_ctx.node_id].clone());
                let mut nodes_to_keep = Vec::new();
                while let Some(node) = queue.pop_front() {
                    nodes_to_keep.push(node.clone());
                    for port in node.ports {
                        if matches!(port.type_, PortType::Input) {
                            continue;
                        }
                        if let Some(dst_node_id) = src_port_id_to_dst_node_id.get(&port.id) {
                            queue.push_back(nodes[dst_node_id].clone());
                        }
                    }
                }
                graph.nodes = nodes_to_keep;
            }
            RunNodeType::All => {}
        }
        debug_assert!(!graph.nodes.is_empty());

        let run_status = RunStatus {
            job_instance_id,
            job_instance_status: JobStatus::Running,
            nodes_status: {
                let mut nodes = Vec::new();
                for node in &graph.nodes {
                    nodes.push(NodeStatusEntry::new(&node.id));
                }
                nodes
            },
            end_time: chrono::Utc::now().naive_utc(),
        };
        run_status.to_db_with_transaction(transaction).await?;

        Ok(graph)
    }
    pub async fn run(self, graph_run_ctx: GraphRunCtx) {
        run_graph(self, graph_run_ctx).await;
    }
    pub fn validate(&self) -> Result<(), ErrorTrace> {
        if self.nodes.is_empty() {
            return Err(ErrorTrace::new("nodes is empty"));
        }
        let mut unique_node_ids = std::collections::HashSet::new();
        for node in &self.nodes {
            if !unique_node_ids.insert(node.id.clone()) {
                return Err(ErrorTrace::new("detect duplicate node_id"));
            }
            if node.resource.cpu == 0.0 {
                return Err(ErrorTrace::new(&format!("node {} cpu is not set", node.id)));
            }
            if node.resource.memory == 0.0 {
                return Err(ErrorTrace::new(&format!(
                    "node {} memory is not set",
                    node.id
                )));
            }
            for param in &node.params {
                /*
                if !["number", "text", "textarea", "radio", "select"]
                    .contains(&param.component_type.as_str())
                {
                    return Err(ErrorTrace::new(&format!(
                        "node {} param {} component_type is invalid",
                        node.id, param.key
                    )));
                }
                */
                if param.component_value.is_empty() {
                    return Err(ErrorTrace::new(&format!(
                        "node {} param {} is can't empty",
                        node.id, param.key
                    )));
                }
            }
        }
        self.all_input_port_is_connected()?;
        Ok(())
    }
    pub fn topologic_sort(&mut self) -> Result<(), ErrorTrace> {
        let mut src_to_dsts = HashMap::<String, Vec<String>>::new();
        for edge in &self.edges {
            src_to_dsts
                .entry(edge.src_node_id.clone())
                .or_default()
                .push(edge.dst_node_id.clone());
        }
        let mut zero_indegree_queue = std::collections::VecDeque::new();
        let mut nodes_indegree = HashMap::new();
        for node in &self.nodes {
            let indegree = node
                .ports
                .iter()
                .filter(|port| matches!(port.type_, PortType::Input))
                .count();
            nodes_indegree.insert(node.id.clone(), indegree);
            if indegree == 0 {
                zero_indegree_queue.push_back(node.id.clone());
            }
        }

        let nodes = self
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node))
            .collect::<HashMap<_, _>>();
        let mut topological_order = Vec::new();
        while let Some(node_id) = zero_indegree_queue.pop_front() {
            if let Some(dst_node_ids) = src_to_dsts.get(&node_id) {
                for dst_node_id in dst_node_ids {
                    *nodes_indegree.get_mut(dst_node_id).unwrap() -= 1;
                    if nodes_indegree[dst_node_id] == 0 {
                        zero_indegree_queue.push_back(dst_node_id.clone());
                    }
                }
            }
            topological_order.push(nodes[&node_id].clone());
        }
        if topological_order.len() != self.nodes.len() {
            return Err(ErrorTrace::new("detect circle in graph"));
        }
        self.nodes = topological_order;

        Ok(())
    }
    fn all_input_port_is_connected(&self) -> Result<(), ErrorTrace> {
        let mut dst_port_id_to_src_node_id = std::collections::HashMap::new();
        for edge in &self.edges {
            if dst_port_id_to_src_node_id
                .insert(edge.target_port_id.clone(), edge.src_node_id.clone())
                .is_some()
            {
                return Err(ErrorTrace::new("input port can only has one input"));
            }
        }

        for node in &self.nodes {
            for port in &node.ports {
                if matches!(port.type_, PortType::Input)
                    && !dst_port_id_to_src_node_id.contains_key(&port.id)
                {
                    return Err(ErrorTrace::new(&format!(
                        "node {} port {} require input",
                        node.id, port.id
                    )));
                }
            }
        }
        Ok(())
    }
}

#[tracing::instrument(skip(graph, graph_run_ctx))]
async fn run_graph(graph: Graph, graph_run_ctx: GraphRunCtx) {
    let job_instance_id = graph_run_ctx.job_instance_id;
    info!("--> run_graph {job_instance_id}");

    let mut dst_port_id_to_src_node_id = std::collections::HashMap::new();
    for edge in graph.edges {
        dst_port_id_to_src_node_id.insert(edge.target_port_id, edge.src_node_id);
    }
    let mut nodes_run_status = graph
        .nodes
        .iter()
        .map(|node| (node.id.clone(), NodeStatus::Pending))
        .collect::<std::collections::HashMap<_, _>>();

    let mut queue = graph
        .nodes
        .into_iter()
        .collect::<std::collections::VecDeque<_>>();
    while let Some(node) = queue.pop_front() {
        if !matches!(nodes_run_status[&node.id], NodeStatus::Pending) {
            continue;
        }
        let has_indegree = node
            .ports
            .iter()
            .any(|port| matches!(port.type_, PortType::Input));
        let mut parent_iter = node
            .ports
            .iter()
            .filter(|port| matches!(port.type_, PortType::Input))
            .map(|port| dst_port_id_to_src_node_id[&port.id].as_str())
            .filter(|&parent_output_port| nodes_run_status.contains_key(parent_output_port));
        let parent_any_fail = parent_iter
            .clone()
            .any(|parent_output_port| nodes_run_status[parent_output_port].fail());
        let parent_all_success = parent_iter.all(|parent_output_port| {
            let parent = &nodes_run_status[parent_output_port];
            !parent.fail() && !matches!(parent, NodeStatus::Pending)
        });
        if !has_indegree {
            *nodes_run_status.get_mut(&node.id).unwrap() = run_node(&node, &graph_run_ctx).await;
        } else if parent_any_fail {
            *nodes_run_status.get_mut(&node.id).unwrap() = NodeStatus::FailParentFail;
            let mut run_status = RunStatus::from_db(job_instance_id).await.unwrap();
            run_status.node_mut(&node.id).unwrap().status = NodeStatus::FailParentFail;
            run_status.to_db().await.unwrap();
        } else if parent_all_success {
            *nodes_run_status.get_mut(&node.id).unwrap() = run_node(&node, &graph_run_ctx).await;
        } else {
            // *nodes_run_status.get_mut(&node.id).unwrap() = NodeStatus::Pending;
            queue.push_back(node);
        }
    }

    let mut run_status = RunStatus::from_db(job_instance_id).await.unwrap();
    let job_status = if run_status
        .nodes_status
        .iter()
        .any(|node| node.status.fail())
    {
        JobStatus::Fail
    } else {
        JobStatus::Success
    };
    run_status.end_time = chrono::Utc::now().naive_utc();
    run_status.job_instance_status = job_status;
    run_status.to_db().await.unwrap();
}

async fn run_node(node: &Node, graph_run_ctx: &GraphRunCtx) -> NodeStatus {
    let start_at = os_utils::get_timestamp();
    let job_instance_id = graph_run_ctx.job_instance_id;

    let mut run_status = RunStatus::from_db(job_instance_id).await.unwrap();
    {
        let node_status = run_status.node_mut(&node.id).unwrap();
        node_status.status = NodeStatus::Running;
        node_status.start_at = start_at;
    }
    run_status.to_db().await.unwrap();

    let node_id = &node.id;
    info!("before run_node {node_id}");
    let status = run_node_inner(node, graph_run_ctx).await;
    let end_at = os_utils::get_timestamp();

    let mut run_status = RunStatus::from_db(job_instance_id).await.unwrap();
    let node_status = run_status.node_mut(&node.id).unwrap();
    node_status.status = status.clone();
    node_status.end_at = Some(end_at);
    node_status.duration = end_at - start_at;
    info!("after run_node {node_id} status={status:?}");
    run_status.to_db().await.unwrap();

    status
}

async fn run_node_inner(node: &Node, graph_run_ctx: &GraphRunCtx) -> NodeStatus {
    let team_id = graph_run_ctx.team_id;
    let project_id = graph_run_ctx.project_id;
    let job_instance_id = graph_run_ctx.job_instance_id;
    let node_id = &node.id;
    let account = account(job_instance_id, node_id);

    let cpu = node.resource.cpu;
    let memory_mb = node.resource.memory * 1000.0;
    let memory_mb = format!("{memory_mb}Mi");
    let gpu = format!("{}", node.resource.gpu);

    let mut map = std::collections::HashMap::new();
    map.insert("team_id", team_id.to_string());
    map.insert("project_id", project_id.to_string());
    map.insert("job_id", graph_run_ctx.job_id.to_string());
    map.insert("job_instance_id", graph_run_ctx.job_instance_id.to_string());
    map.insert("node_id", node_id.clone());

    let env = map
        .into_iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(",");

    let json = NewVolcanoJobReq {
        service: "volcano",
        action: "worker-apply",
        platform: PLATFORM,
        region: graph_run_ctx.region.clone(),
        account: account.clone(),
        mode: DEPLOY_MODE.clone(),
        project: project_id.to_string(),
        pod_request_cpu: cpu.to_string(),
        pod_limit_cpu: cpu.to_string(),
        pub_request_memory: memory_mb.clone(),
        pub_limit_memory: memory_mb,
        pod_request_gpu: gpu.clone(),
        pod_limit_gpu: gpu,
        image: Some(node.image.clone()),
        env,
        annotations: k8s_pod_annotation(node, graph_run_ctx),
    };
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let rsp = client
        .post(format!("{K8S_SVC_BASE_URL}/volcano-job"))
        .json(&json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let pod_name = format!("{}-{}-{}-job-0", PLATFORM, graph_run_ctx.region, account);
    if rsp.contains("500") || rsp.to_lowercase().contains("fail") {
        error!(
            "create vcjob pod_name={pod_name}, req = {}\nrsp = {rsp}",
            serde_json::to_string_pretty(&json).unwrap()
        );
        return NodeStatus::FailCreatePod;
    }

    let timeout = node.timeout_secs();
    let node_id_ = node_id.clone();
    /// `node` escapes the function body here, argument requires that `'1` must outlive `'static`
    async fn timeout_hook(timeout: u64, node_id: String, job_instance_id: JobInstanceId) {
        tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
        let mut run_status = RunStatus::from_db(job_instance_id).await.unwrap();
        if let Ok(node_) = run_status.node_mut(&node_id) {
            if !node_.status.is_finish() {
                node_.status = NodeStatus::FailTimeout;
                run_status.to_db().await.unwrap();
            }
        } else {
            error!("node {node_id} not found in {job_instance_id}, can't timeout");
        }
    }
    tokio::spawn(timeout_hook(timeout, node_id_, job_instance_id));

    let mut rx = RUN_STATUS_MSG_QUEUE.subscribe();
    loop {
        let run_status = match rx.recv().await {
            Ok(x) => x,
            Err(err) => {
                error!("err on rx.recv {err}");
                continue;
            }
        };
        info!("while let Ok {run_status:?}",);
        if let Ok(node) = run_status.node(node_id) {
            if node.status.is_finish() {
                return node.status.clone();
            }
        } else {
            error!("node_id not found {run_status:?}");
        }
        if matches!(run_status.job_instance_status, JobStatus::Canceling) {
            return NodeStatus::FailCancel;
        }
        run_status.to_db().await.unwrap();
    }
    // unreachable!();

    // let client = kubernetes_client::k8s_api_client();
    // let k8s_api_url = format!(
    //     "{}/{pod_name}",
    //     kubernetes_client::k8s_api_server_base_url()
    // );
    // let mut pod_404_count = 0;
    #[cfg(not)]
    for tick in 0..node.timeout_secs() {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if tick % 2 != 0 {
            continue;
        }
        let rsp = client.get(&k8s_api_url).send().await.unwrap();
        if !rsp.status().is_success() {
            pod_404_count += 1;
            error!(
                "{pod_name} {node_id} {}, maybe vcjob is pending or shutdown(cancel)",
                rsp.status()
            );
            if pod_404_count > 4 {
                return NodeStatus::FailPodNotFound;
            }
            continue;
        }
        debug_assert!(rsp.status().is_success());
        let pod = rsp.json::<kubernetes_client::Pod>().await.unwrap();
        let status = pod.status();
        if status.is_finish() {
            if matches!(status, PodContainerStatus::Succeeded) {
                return NodeStatus::Success;
            }
            return NodeStatus::Fail;
        }
    }

    // error!("{pod_name} {node_id} timeout {}s", node.timeout_secs());
    // NodeStatus::FailTimeout
}

/*
"annotations": {
    "teamId": f"{team_id}",
    # TODO
    "userId": f"{team_id}",
    "projectId": f"{project_id}",
    "executorId": f"{pod_id}",
    "kernelSource": f"{kernel_or_pipeline.replace('kernel', 'workspace')}",
    "path": f"{pod_id}",
    "cpuUsed": f"{cpu_cores}",
    "memUsed": f"{memory_gb}",
    "gpuUsed": f"{num_gpu}",
    "priority": f"{priority}",
}
*/
fn k8s_pod_annotation(node: &Node, graph_run_ctx: &GraphRunCtx) -> HashMap<&'static str, String> {
    let mut map = HashMap::new();
    map.insert("teamId", graph_run_ctx.team_id.to_string());
    map.insert("userId", graph_run_ctx.user_id.to_string());
    map.insert("projectId", graph_run_ctx.project_id.to_string());
    let pod_name = super::helper::k8s_svc::pod_name(graph_run_ctx.job_instance_id, &node.id);
    map.insert("executorId", pod_name.clone());
    map.insert("kernelSource", "visual".to_string());
    map.insert("path", pod_name);
    map.insert("cpuUsed", node.resource.cpu.to_string());
    map.insert("memUsed", node.resource.memory.to_string());
    map.insert("gpuUsed", node.resource.gpu.to_string());
    map.insert("priority", "3".to_string());
    map
}
