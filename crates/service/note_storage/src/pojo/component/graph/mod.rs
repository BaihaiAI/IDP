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
use serde::Serialize;

pub use crate::handler::visual_modeling::helper::k8s_svc::delete_vc_job;

// type JobInstanceId = i32;
// type NodeId = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub ports: Vec<Port>,
    /// docker image name
    pub image: String,
    pub fields: Vec<serde_json::Value>,
    pub params: Vec<NodeParams>,
    pub resource: NodeResource,
    // pub category_id: i32,
    pub render_key: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub key: String,
    pub label: String,
    /// whether display `show_run_result` on node's right click menu
    #[serde(default)]
    pub display_data: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeParams {
    /* option of radio or select
    "componentOptionValues": [{
        "label":"key_volov",
        "value":"yolov5s"
    }]
    */
    #[serde(default)]
    pub component_option_values: Vec<serde_json::Value>,
    pub key: String,
    /// one of [number, text, textarea, radio, select]
    pub component_type: String,
    pub component_value: String,
    /// current only used in frontend
    pub data_type: String,
    // #[serde(default)]
    // pub is_show: bool,
    pub name: String,
}

impl Node {
    pub fn timeout_secs(&self) -> u64 {
        for param in &self.params {
            if param.key == "timeout" {
                return param.component_value.parse().unwrap();
            }
        }
        3600
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeResource {
    pub cpu: f64,
    pub memory: f64,
    pub gpu: f64,
    // pub priority: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: PortType,
    pub data_type: String,
    pub group: String,
    #[serde(default)]
    pub key: String,
    pub name: String,
    pub tooltip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PortType {
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    #[serde(rename = "source")]
    pub src_node_id: String,
    #[serde(rename = "target")]
    pub dst_node_id: String,
    pub source_port_id: String,
    pub target_port_id: String,
}
