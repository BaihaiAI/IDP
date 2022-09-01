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

use super::diff_models::DiffCellOut;

/////////// /snapshot/diff ///////////
#[derive(Serialize, Debug, Clone)]
#[cfg(not)]
pub enum OutCellType {
    Code,
    Markdown,
    Sql,
    Vis,
    Empty,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct SnapshotDiffRes {
    pub cells1: Vec<DiffCellOut>,
    pub cells2: Vec<DiffCellOut>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiffReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub id1: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub id2: u64,
    pub path: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
}

/////////// /snapshot/list ///////////

#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct SnapshotListItem {
    pub id: String,
    // pub path: String,
    pub label: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotListReq {
    pub path: [String; 1],
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotListRes {
    pub snapshots: Vec<SnapshotListItem>,
}

/////////// /snapshot ///////////

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotReq {
    pub path: String,
    pub label: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRes {
    pub snapshots: Vec<SnapshotListItem>,
}

/////////// /snapshot/restore ///////////

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRestoreReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub id: u64,
    pub path: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotRestoreRes {
    pub id: String,
    pub path: String,
    pub label: String,
}
