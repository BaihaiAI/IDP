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

#![deny(unused_crate_dependencies)]
pub mod keys;
#[cfg(feature = "redis")]
mod redis;

use common_model::entity::cell::Cell;
pub use common_model::entity::cell::CellUpdate;
pub use common_model::entity::cell::Updates;
use common_model::entity::notebook::BaseInfo;
use common_model::entity::notebook::Notebook;
pub use keys::snapshot_key;
use tracing::debug;
use tracing::error;

#[cfg(feature = "redis")]
pub use crate::redis::CacheService;
// pub use refresh_disk::RefreshDto;

#[derive(Debug, Clone)]
pub struct RefreshDto {
    pub project_id: u64,
    pub key: String,
    pub path: String,
    pub file_type: common_model::enums::mime::Mimetype,
}

impl RefreshDto {
    pub(crate) fn new(
        key: String,
        path: String,
        file_type: common_model::enums::mime::Mimetype,
        project_id: u64,
    ) -> RefreshDto {
        RefreshDto {
            project_id,
            key,
            path,
            file_type,
        }
    }
}

pub enum CloneState {
    Cloning,
    Success,
    Failed,
}
impl std::fmt::Display for CloneState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloneState::Cloning => write!(f, "cloning"),
            CloneState::Success => write!(f, "success"),
            CloneState::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotRedisListItem {
    pub id: u64,
    // pub path: String,
    pub label: String,
    // pub time: DateTime<Utc>,
    // wanted format `2022-06-18 07:34:28.150690440`
    pub time: String,
    pub content: String,
}

pub(crate) fn vec_string_into_notebook(
    val_vec: Vec<String>,
) -> Result<Notebook, serde_json::Error> {
    let mut notebook = Notebook::default();
    let mut cells: Vec<Cell> = vec![];

    let val_vec_len = val_vec.len();
    for value_item in val_vec {
        match serde_json::from_str::<Cell>(&value_item) {
            Ok(cell_item) => cells.push(cell_item),
            Err(expect_err) => {
                debug!(
                    "serde_json deserialize expect_err:{:?}\n , value_item_str:{}",
                    expect_err, &value_item
                );
                match serde_json::from_str::<BaseInfo>(&value_item) {
                    Ok(base_info) => {
                        notebook.base = base_info;
                    }
                    Err(unexpected_err) => {
                        error!(
                            "serde_json deserialize expect_err:{:?}\n , value_item_str:{}",
                            unexpected_err, &value_item
                        );
                        return Err(unexpected_err);
                    }
                }
            }
        }
    }
    debug!(
        "notebook path: {:?}, values length: {}",
        notebook.path(),
        val_vec_len
    );

    if cells.is_empty() {
        error!(
            "cells is empty!,notebook path: {:?}, values length: {}",
            notebook.path(),
            val_vec_len
        );
    } else {
        cells.sort_unstable_by(|cell_a, cell_b| {
            cell_a
                .index()
                .unwrap_or(1.0)
                .partial_cmp(&cell_b.index().unwrap_or(1.0))
                .unwrap()
        });
    }
    if notebook.path() == None && notebook.cells.len() < 2 {
        error!(
            "this notebook path is None! Maybe occur error,cells: {:?}",
            val_vec_len
        );
    }

    notebook.cells = cells;
    Ok(notebook)
}
