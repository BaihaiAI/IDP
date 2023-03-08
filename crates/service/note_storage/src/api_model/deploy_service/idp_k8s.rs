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

use super::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDeployBody {
    pub project: Option<String>,
    pub action: &'static str,
    pub service: &'static str,
    pub region: String,
    pub team_id: String,
    pub user_id: String,
    pub account: String,
    pub image: String,
    pub pod_request_memory: Option<String>,
    pub pod_limit_memory: Option<String>,
    pub pod_request_cpu: Option<String>,
    pub pod_limit_cpu: Option<String>,
    pub pod_request_gpu: Option<String>,
    pub pod_limit_gpu: Option<String>,
    pub env: Option<String>,
    pub mode: String,
    pub replicas: Option<String>,
    pub service_type: String,
    pub service_id: String,
    pub schedule: Option<String>,
    pub node_sn: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDestroyBody {
    pub account: String,
    pub region: String,
    pub action: &'static str,
    pub service: &'static str,
    pub platform: &'static str,
    pub service_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RspBody {
    pub code: i32,
    pub data: serde_json::Value,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanikoImageBody {
    pub team_id: String,
    pub image_name: String,
    pub action: &'static str,
    pub service: &'static str,
    pub region: String,
    pub docker_file_path: String,
    pub image_tar_path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceStatusReq {
    pub account: String,
    pub namespace: String,
    pub service_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetImageBody {
    pub repo_name: &'static str,
    pub image_name: String,
    pub tag_name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceData {
    pub info: Vec<ResourceRsp>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRsp {
    pub memory: String,
    pub cpu: String,
    pub gpu: String,
}
impl ResourceRsp {
    pub fn modify_unit(&mut self) {
        let mut memory = self
            .memory
            .to_uppercase()
            .strip_suffix("MI")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        memory /= 1024;
        let mut cpu = self
            .cpu
            .to_uppercase()
            .strip_suffix('M')
            .unwrap()
            .parse::<i32>()
            .unwrap();
        cpu /= 1000;
        self.memory = memory.to_string();
        self.cpu = cpu.to_string();
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResourceBody {
    pub action: &'static str,
    pub service: &'static str,
    pub account: String,
    pub region: String,
    pub namespace: String,
}
