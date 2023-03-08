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

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/1/19
 * Time: 10:54
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommInfo {
    pub script: String,
    pub task_name: String,
    pub task_id: u64,
    pub task_edge: Vec<TaskEdge>,
    pub ex_info: ExInfo,
    pub task_type: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExInfo {
    pub position_x: i32,
    pub position_y: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEdge {
    pub to_task: u64,
    pub from_task: u64,
}
