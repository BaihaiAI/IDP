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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatus {
    pub code: i32,
    pub data: Vec<DeploymentStatusData>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentStatusData {
    pub available_replicas: Option<i32>,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub last_transition_time: String,
    pub last_update_time: String,
    pub message: String,
    pub reason: String,
    pub status: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatusData {
    pub active: Option<Vec<Active>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Active {
    pub api_version: Option<String>,
    pub field_path: Option<serde_json::Value>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub resource_version: Option<String>,
    pub uid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    pub code: i64,
    pub data: Vec<Vec<PodStatusData>>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatusData {
    pub last_probe_time: Option<String>,
    pub last_transition_time: Option<String>,
    pub message: Option<String>,
    pub reason: Option<String>,
    pub status: String,
    pub r#type: String,
}
