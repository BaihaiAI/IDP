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

use std::path::Path;

use chrono;
use chrono::SecondsFormat;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::entity::cell::Cell;

/// @author Kim Huang
/// @date 2022/4/13 pm.5:18
///  
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Notebook {
    #[serde(flatten)]
    pub base: BaseInfo,
    #[serde(default)]
    pub cells: Vec<Cell>,
}

impl Notebook {
    pub fn new(path: &str) -> Self {
        Notebook {
            base: BaseInfo::new(path),
            cells: vec![Cell::default()],
        }
    }
    pub fn path(&self) -> Option<&str> {
        let value = &self.base.metadata;
        match value.get("path") {
            None => None,
            Some(value) => value.as_str(),
        }
    }
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.set_field("path", path.as_ref().to_str().unwrap())
    }
    /// shutdown request require inode from cat API response
    pub fn set_inode(&mut self, inode: u64) {
        // prevent u64 truncate in json
        self.set_field("inode", inode.to_string());
    }

    pub fn update_last_modified_time(&mut self) {
        self.base.update_last_modified_time();
    }

    fn set_field<T>(&mut self, k: &str, v: T)
    where
        T: Serialize,
    {
        self.base.set_metadata_field(k, v);
    }

    pub fn get_cell_by_id(&mut self, id: String) -> Option<Cell> {
        self.cells
            .iter()
            .find(|&cell| {
                if let Some(cell_id) = cell.id() {
                    cell_id.eq(&id)
                } else {
                    false
                }
            })
            .cloned()
    }
    #[cfg(not)]
    fn sort_cells_by_index(&mut self) {
        self.cells.sort_unstable_by(|cell_a, cell_b| {
            cell_a
                .index()
                .unwrap_or(0.0001)
                .partial_cmp(&cell_b.index().unwrap_or(0.0001))
                .unwrap_or(Ordering::Equal)
        });
    }
    #[cfg(not)]
    fn update_info(&mut self, mem_length: usize, inode: u64) {
        self.set_inode(inode);
        //TODO: maybe sort when update cell better than here
        self.sort_cells_by_index();
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Notebook {
            base: BaseInfo::new(""),
            cells: vec![Cell::default()],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseInfo {
    pub nbformat: usize,
    pub nbformat_minor: usize,
    pub metadata: Value,
}

impl BaseInfo {
    pub fn new(path: &str) -> Self {
        BaseInfo {
            nbformat: 4,
            nbformat_minor: 5,
            metadata: json!({
                "kernelspec": {
                    "display_name": "Python 3 (ipykernel)",
                    "language": "python",
                    "name": "python3"
                },
                "language_info": {
                    "codemirror_mode": {
                    "name": "ipython",
                    "version": 3
                },
                "file_extension": ".py",
                    "mimetype": "text/x-python",
                    "name": "python",
                    "nbconvert_exporter": "python",
                    "pygments_lexer": "ipython3",
                    "version": "3.9.7"
                },
                "path": if path.is_empty() {
                    None
                } else {
                    Some(path)
                }
            }),
        }
    }

    pub fn update_last_modified_time(&mut self) {
        let time = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        self.set_metadata_field("last_modified", time)
    }

    fn set_metadata_field<T>(&mut self, k: &str, v: T)
    where
        T: Serialize,
    {
        let metadata = self.metadata.as_object_mut().unwrap();
        metadata.insert(k.to_string(), serde_json::to_value(v).unwrap());
        self.metadata = serde_json::to_value(metadata).unwrap()
    }
}
