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
use serde_json::json;

use crate::entity::cell::Cell;

#[derive(Serialize, Deserialize)]
pub struct Notebook {
    #[serde(default)]
    pub cells: Vec<Cell>,

    nbformat: usize,
    nbformat_minor: usize,
    metadata: serde_json::Map<String, serde_json::Value>,
}

impl Notebook {
    pub fn new(cells: Vec<Cell>) -> Self {
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "kernelspec".to_string(),
            json!({
                "display_name": "Python 3 (IdpKernel)",
                "language": "python",
                "name": "python3"
            }),
        );
        metadata.insert(
            "language_info".to_string(),
            json!({
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
            }),
        );

        Notebook {
            cells,

            nbformat: 4,
            nbformat_minor: 5,
            metadata,
        }
    }
    /// only used in cat API for frontend require
    pub fn set_path(&mut self, path: &str) {
        self.metadata_insert_kv("path", path)
    }
    /// shutdown request require inode from cat API response
    pub fn set_inode(&mut self, inode: u64) {
        // prevent u64 truncate in json
        self.metadata_insert_kv("inode", inode.to_string());
    }

    fn metadata_insert_kv<T>(&mut self, k: &str, v: T)
    where
        T: Serialize,
    {
        self.metadata
            .insert(k.to_string(), serde_json::to_value(v).unwrap());
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
}
