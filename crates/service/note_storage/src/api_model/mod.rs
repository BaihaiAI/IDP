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

pub mod content;
pub mod deploy_service;
pub mod environment;
pub mod hpopt;
pub mod pipeline;
pub mod project;
pub mod service_log;
pub mod workspace;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamIdQueryString {
    pub team_id: u64,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamIdProjectIdQueryString {
    pub team_id: u64,
    pub project_id: u64,
}
