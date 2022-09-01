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

use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectId {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub index: Option<u32>,
    pub total: Option<u32>,
    pub project_dto_str: ProjectDtoStr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct ProjectRet {
    pub code: u32,
    pub message: Option<String>,
    pub data: ProjectDataObj,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectDataObj {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CommResultRet<T>
where
    T: Serialize,
{
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDtoStr {
    pub team_id: Option<String>,
    pub creator: Option<String>,
    pub project_name: Option<String>,
    pub project_type: ProjectType,
    pub git_url: Option<String>,
    pub git_info: Option<GitInfoObj>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitInfoObj {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Git,
    File,
    Default,
}
