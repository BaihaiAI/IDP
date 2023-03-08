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

use serde::Deserialize;

/*
{
    "apiVersion": "v1",
    "items": [],
    "kind": "PodList",
    "metadata": {
        "resourceVersion": "138348984",
        "selfLink": "/api/v1/namespaces/host/pods"
    }
}
*/
#[derive(Deserialize, Debug)]
pub struct PodListRsp {
    pub items: Vec<Pod>,
}

#[derive(Deserialize, Debug)]
pub struct PodListWatchRsp {
    #[serde(rename = "type")]
    pub type_: NotifyType,
    pub object: Pod,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotifyType {
    Added,
    Modified,
    Deleted,
}

#[derive(Deserialize, Debug)]
pub struct Pod {
    pub metadata: PodMetadata,
    pub spec: PodSpec,
    pub status: PodStatus,
}

#[derive(Deserialize, Debug)]
pub struct PodMetadata {
    pub name: String, // resource_version: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    pub phase: PodStatusPhase,
    // this field is unset if pod is init/pending
    pub container_statuses: Option<Vec<ContainerStatus>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    pub state: ContainerState,
    // last_state: serde_json::Value,
    // ready: bool,
    // started: bool,
}

/// https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.19/#containerstate-v1-core
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ContainerState {
    #[serde(rename_all = "camelCase")]
    Running {
        // started_at: String,
    },
    Waiting {
        // message: String,
        reason: String,
    },
    #[serde(rename_all = "camelCase")]
    Terminated { reason: String, exit_code: u8 },
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "UPPERCASE")]
pub enum PodStatusPhase {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Deserialize, Debug)]
pub struct PodSpec {
    pub containers: Vec<PodSpecContainer>,
}

#[derive(Deserialize, Debug)]
pub struct PodSpecContainer {
    pub name: String,
    pub resources: ContainerResource,
    #[serde(default)]
    pub env: Vec<EnvItem>,
}

#[derive(Deserialize, Debug)]
pub struct ContainerResource {
    pub limits: Option<ResourceLimits>,
}

#[derive(Deserialize, Debug)]
pub struct EnvItem {
    pub name: String,
    // value or valueFrom
    #[serde(default)]
    pub value: String,
    /*
    "valueFrom": {
        "fieldRef": {
            "apiVersion": "v1",
            "fieldPath": "metadata.name"
        }
    }
    */
}

#[derive(Deserialize, Debug)]
pub struct ResourceLimits {
    pub cpu: String,
    pub memory: String,
    #[serde(rename = "nvidia.com/gpu")]
    pub nvidia_gpu_cards: Option<String>,
    #[serde(rename = "ucloud.cn/gpu-mem")]
    pub ucloud_gpu_mem: Option<String>,
}

impl ResourceLimits {
    #[cfg(not)]
    fn resource(&self) -> Resource {
        Resource {
            memory: self.memory_gb(),
            num_cpu: self.cpu_cores(),
            num_gpu: self.gpu(),
            ..Default::default()
        }
    }
    pub fn cpu_cores(&self) -> f64 {
        if self.cpu.ends_with('m') {
            self.cpu
                .trim_end_matches('m')
                .parse::<f64>()
                .expect(&self.cpu)
                / 1000.0
        } else {
            self.cpu.parse::<f64>().expect(&self.cpu)
        }
    }
    pub fn memory_gb(&self) -> f64 {
        self.memory
            .trim_end_matches("Mi")
            .parse::<f64>()
            .expect(&self.memory)
            / 1000.0
    }
    pub fn gpu(&self) -> u16 {
        if let Some(ref gpu_cards) = self.nvidia_gpu_cards {
            gpu_cards.parse().expect(gpu_cards)
        } else if let Some(ref gpu_mem) = self.ucloud_gpu_mem {
            gpu_mem.parse().expect(gpu_mem)
        } else {
            0
        }
    }
}

fn k8s_auth_header() -> String {
    format!(
        "Bearer {}",
        std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token")
            .expect("fail to read K8s token")
    )
}

pub fn k8s_api_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        k8s_auth_header().parse().unwrap(),
    );
    reqwest::ClientBuilder::new()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .connect_timeout(std::time::Duration::from_secs(2))
        .build()
        .expect("reqwest::ClientBuilder")
}

pub fn k8s_api_server_base_url() -> String {
    let api_server_host =
        std::env::var("KUBERNETES_SERVICE_HOST").expect("KUBERNETES_SERVICE_HOST");
    let api_server_port =
        std::env::var("KUBERNETES_SERVICE_PORT").expect("KUBERNETES_SERVICE_PORT");
    let namespace = &*business::kubernetes::NAMESPACE;
    format!("https://{api_server_host}:{api_server_port}/api/v1/namespaces/{namespace}/pods")
}

pub fn volcano_queue() -> String {
    let namespace = &*business::kubernetes::NAMESPACE;
    let team_id_or_executor = &*business::kubernetes::ACCOUNT;
    let region = &*business::kubernetes::REGION;
    format!("volcano-queue-{namespace}-{region}-{team_id_or_executor}")
}

impl Pod {
    pub fn status(&self) -> PodContainerStatus {
        match self.status.phase {
            PodStatusPhase::Running => PodContainerStatus::Running,
            PodStatusPhase::Succeeded => PodContainerStatus::Succeeded,
            PodStatusPhase::Failed => {
                if let Some(containers) = &self.status.container_statuses {
                    if let Some(status) = containers.get(0) {
                        if let ContainerState::Terminated { reason, exit_code } = &status.state {
                            if reason == "Error" {
                                return PodContainerStatus::Failed;
                            }
                            if *exit_code == 137 {
                                return PodContainerStatus::OOMKilled;
                            }
                        }
                    }
                }
                PodContainerStatus::Failed
            }
            PodStatusPhase::Pending => {
                if let Some(containers) = &self.status.container_statuses {
                    if let Some(status) = containers.get(0) {
                        if let ContainerState::Waiting { reason } = &status.state {
                            return match reason.as_str() {
                                "ContainerCreating" => PodContainerStatus::ContainerCreating,
                                "ErrImagePull" | "ImagePullBackOff" => {
                                    PodContainerStatus::ErrImagePull
                                }
                                "CreateContainerError" => PodContainerStatus::CreateContainerError,
                                "CreateContainerConfigError" => {
                                    PodContainerStatus::CreateContainerConfigError
                                }
                                _ => PodContainerStatus::Pending,
                            };
                        }
                    }
                }
                PodContainerStatus::Pending
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum PodContainerStatus {
    Pending,
    ContainerCreating,
    // ImagePullBackOff,
    ErrImagePull,
    // delete pod when ContainerCreating
    CreateContainerConfigError,
    /// context deadline exceeded
    CreateContainerError,

    Running,
    Terminating,
    /// same as Completed
    Succeeded,

    // same as Error, but maybe contains OOMKilled?
    Failed,
    OOMKilled,

    /// custom business status
    Applying,
    /// custom business status
    #[default]
    Closed,
}

impl PodContainerStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
    pub fn is_creating_or_running(&self) -> bool {
        matches!(
            self,
            Self::Applying | Self::Pending | Self::ContainerCreating | Self::Running
        )
    }
    #[cfg(not)]
    pub fn is_finish(&self) -> bool {
        matches!(
            self,
            Self::Failed | Self::OOMKilled | Self::Closed | Self::Succeeded
        )
    }
}

#[test]
#[ignore]
fn test_de() {
    serde_json::from_str::<PodListWatchRsp>(&std::fs::read_to_string("target/k8s.json").unwrap())
        .unwrap();
}
