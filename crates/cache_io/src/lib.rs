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
mod redis;

use common_model::entity::cell::Cell;
pub use common_model::entity::cell::CellUpdate;
pub use common_model::entity::cell::Updates;
use common_model::entity::notebook::Notebook;
use err::ErrorTrace;
pub use keys::snapshot_key;

pub use crate::redis::CacheService;
pub(crate) const IPYNB_CACHE_TTL: usize = 12 * 60;
// pub use refresh_disk::RefreshDto;

#[derive(Debug, Clone)]
pub struct RefreshDto {
    pub project_id: u64,
    pub key: String,
    pub path: String,
    // pub file_type: common_model::enums::mime::Mimetype,
}

impl RefreshDto {
    pub(crate) fn new(key: String, path: String, project_id: u64) -> RefreshDto {
        RefreshDto {
            project_id,
            key,
            path,
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
pub enum OptimizeState {
    Running,
    Success,
    Failed,
}
impl std::fmt::Display for OptimizeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizeState::Running => write!(f, "running"),
            OptimizeState::Success => write!(f, "success"),
            OptimizeState::Failed => write!(f, "failed"),
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

pub(crate) fn redis_hvals_to_notebook(val_vec: Vec<String>) -> Result<Notebook, ErrorTrace> {
    if val_vec.is_empty() {
        return Err(ErrorTrace::new("cells is empty"));
    }

    let mut cells: Vec<Cell> = vec![];
    for value_item in val_vec {
        let cell = serde_json::from_str::<Cell>(&value_item)?;
        if cell.index().is_none() {
            return Err(ErrorTrace::new("panicked cellId in redis hvals"));
        }
        cells.push(cell);
    }

    cells.sort_unstable_by(|cell_a, cell_b| {
        cell_a
            .index()
            .unwrap()
            .partial_cmp(&cell_b.index().unwrap())
            .unwrap()
    });

    #[cfg(not)]
    if notebook.path().is_none() && notebook.cells.len() < 2 {
        tracing::warn!("this notebook path is None and cells len < 2");
    }

    let notebook = Notebook::new(cells);
    Ok(notebook)
}
