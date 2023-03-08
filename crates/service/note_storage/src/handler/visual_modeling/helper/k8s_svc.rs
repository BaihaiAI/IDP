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

use once_cell::sync::Lazy;
use serde::Serialize;

pub const K8S_SVC_BASE_URL: &str = "http://idp-k8s-service-svc:8084/api/command/k8s";
pub const PLATFORM: &str = "visual";

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewVolcanoJobReq {
    pub service: &'static str,
    pub action: &'static str,
    pub platform: &'static str,

    pub region: String,
    pub account: String,
    pub mode: DeployMode,
    pub project: String,

    pub pod_request_cpu: String,
    pub pod_limit_cpu: String,
    pub pub_request_memory: String,
    pub pub_limit_memory: String,
    pub pod_request_gpu: String,
    pub pod_limit_gpu: String,

    pub image: Option<String>,
    pub env: String,
    // args: Vec<String>

    /*
    from idp-note-rs repo
    struct ResourceReportPayload {
        team_id: Option<String>,
        user_id: Option<String>,
        executor_id: Option<String>,
        project_id: Option<i32>,
        status: Option<String>,
        cpu_used: Option<f32>,
        mem_used: Option<f32>,
        gpu_used: Option<f32>,
        job_id: Option<i32>,
        job_instance_id: Option<i32>,
        task_instance_id: Option<i32>,
        kernel_source: Option<String>,
        path: Option<String>,
        ip: Option<String>,
        inode: Option<String>,
        priority: Option<i32>,
    }
    */
    pub annotations: std::collections::HashMap<&'static str, String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeployMode {
    Public,
    Private,
}
pub static DEPLOY_MODE: Lazy<DeployMode> = Lazy::new(|| {
    let deploy_mode = &crate::app_context::CONFIG.deploy_mode;
    if deploy_mode == "host" {
        DeployMode::Private
    } else if deploy_mode == "saas" {
        DeployMode::Public
    } else {
        panic!("unknown deploy_mode {deploy_mode}");
    }
});

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteVolcanoJobReq {
    action: &'static str,
    platform: &'static str,
    service: &'static str,

    account: String,
    region: String,
}

pub fn delete_vc_job(job_instance_id: i32, node_id: &str) -> reqwest::RequestBuilder {
    let req = DeleteVolcanoJobReq {
        service: "volcano",
        action: "worker-destroy",
        platform: PLATFORM,
        region: business::kubernetes::REGION.clone(),
        account: account(job_instance_id, node_id),
    };
    let client = reqwest::ClientBuilder::new().build().unwrap();
    client
        .post(format!("{K8S_SVC_BASE_URL}/destroy"))
        .json(&req)
}

/// only used in NewVolcanoJobReq
pub fn account(job_instance_id: i32, node_id: &str) -> String {
    let team_id = &*business::kubernetes::ACCOUNT;
    let region = &*business::kubernetes::REGION;

    let node_id = node_id.replace('-', "");
    // each label in K8s pod with max length of 63 bytes
    let node_id_max_len =
        63 - format!("{PLATFORM}-{region}-{team_id}-{job_instance_id}--job-0").len();
    if node_id.len() > node_id_max_len {
        let node_id_crop = &node_id.as_bytes()[..node_id_max_len];

        let node_id_crop = unsafe { String::from_utf8_unchecked(node_id_crop.to_vec()) };
        debug_assert!(!node_id_crop.ends_with('-'));
        format!("{team_id}-{job_instance_id}-{node_id_crop}")
    } else {
        format!("{team_id}-{job_instance_id}-{node_id}")
    }
}

pub fn pod_name(job_instance_id: i32, node_id: &str) -> String {
    let region = &*business::kubernetes::REGION;
    format!(
        "{PLATFORM}-{region}-{}-job-0",
        account(job_instance_id, node_id)
    )
}
