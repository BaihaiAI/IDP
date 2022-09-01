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

use std::fmt::Debug;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
pub use uuid::Uuid;

use crate::entity::cell::CellType::Markdown;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Cell {
    pub cell_type: CellType,
    #[serde(default)]
    pub outputs: Vec<serde_json::Map<String, serde_json::Value>>,
    pub source: Vec<String>, // List<String> source;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<usize>,
    pub metadata: Value,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        let cell_id = Uuid::new_v4();
        if cell_type == Markdown {
            return Cell {
                cell_type,
                outputs: Vec::new(),
                source: vec![],
                execution_time: None,
                execution_count: None,
                metadata: json!({ "id": cell_id }),
            };
        }

        Cell {
            cell_type,
            outputs: Vec::new(),
            source: vec![],
            execution_time: Some("0.0".to_string()),
            execution_count: Some(0),
            metadata: json!({ "id": cell_id }),
        }
    }

    pub fn id(&self) -> Option<String> {
        // TODO user upload ipynb has no id, should handle here
        if let Some(id_value) = self.metadata.get("id") {
            return Some(id_value.as_str().unwrap().to_string());
        }
        None
    }
    pub fn index(&self) -> Option<f64> {
        self.metadata
            .get("index")
            .map(|index| index.as_f64().unwrap())
    }
    fn set_field<T>(&mut self, k: &str, v: T)
    where
        T: Serialize,
    {
        let metadata = self.metadata.as_object_mut().unwrap();
        metadata.insert(k.to_string(), serde_json::to_value(v).unwrap());
        self.metadata = serde_json::to_value(metadata).unwrap()
    }
    pub fn set_id(&mut self, id: Uuid) {
        self.set_field("id", id);
    }
    pub fn set_index(&mut self, index: f64) {
        self.set_field("index", index);
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //print all properties except outputs when cell_type not equal to markdown.
        if self.cell_type != Markdown {
            write!(
                f,
                "Cell {{ cell_type: {:?}, source: {:?}, metadata: {:?} }}",
                self.cell_type, self.source, self.metadata
            )
        } else {
            write!(
                f,
                "Cell {{ cell_type: {:?}, source: '...', metadata: {:?} }}",
                self.cell_type, self.metadata
            )
        }
    }
}
impl Default for Cell {
    fn default() -> Self {
        let cell_id = Uuid::new_v4();
        Cell {
            cell_type: CellType::Code,
            outputs: Vec::new(),
            source: vec![],
            execution_time: Some("0.0".to_string()),
            execution_count: Some(0),
            metadata: json!({ "id": cell_id }),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CellType {
    Code,
    Sql,
    Markdown,
    Visualization,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct CellUpdate {
    pub id: String,
    pub updates: Updates,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct Updates {
    pub cell_type: Option<CellType>,
    pub outputs: Option<Vec<serde_json::Map<String, serde_json::Value>>>,
    pub source: Option<Vec<String>>,
    pub execution_time: Option<String>,
    pub execution_count: Option<usize>,
    pub metadata: Option<Value>,
}
