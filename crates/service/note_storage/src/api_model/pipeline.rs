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

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2022/7/14
 * Time: 14:37
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */
use serde::Deserialize;
use serde::Serialize;

#[cfg(not)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStatusDto {
    pub notebook_path: String,
    pub pipeline_name: String,
    pub pipeline_identity: String,
    pub state: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineCatResult {
    pub start: i32,
    pub limit: u32,
    pub job_id: String,
    pub job_instance_id: String,
    pub task_instance_id: String,
    pub path: String,
    pub team_id: String,
    pub project_id: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineResultRequest {
    pub job_id: String,
    pub job_instance_id: String,
    pub task_instance_id: String,
    pub path: String,
    pub project_id: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineResultDto {
    pub content: String,
    pub total_line: u32,
}
