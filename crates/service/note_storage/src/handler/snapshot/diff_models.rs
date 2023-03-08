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

use common_model::entity::cell::Cell;
use common_model::entity::cell::CellType;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiffCellOut {
    pub idx: Option<usize>,
    pub same: bool,
    pub cell_type: OutCellType,
    pub data: Option<serde_json::Value>,
}

impl DiffCellOut {
    pub fn from_diff_cell(cell: DiffCell, idx: usize) -> DiffCellOut {
        DiffCellOut {
            idx: Some(idx),
            cell_type: cell.get_type(),
            same: false,
            data: match cell {
                DiffCell::Sql(c) => Some(json!(c)),
                DiffCell::Vis(c) => Some(json!(c)),
                DiffCell::Code(c) => Some(json!(c)),
                DiffCell::Markdown(c) => Some(json!(c)),
            },
        }
    }

    pub fn gen_empty() -> DiffCellOut {
        Self {
            idx: None,
            same: false,
            cell_type: OutCellType::Empty,
            data: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffLine {
    pub idx: Option<usize>,
    pub colored: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum DiffCell {
    Code(DiffCodeCell),
    Markdown(DiffMarkdownCell),
    Vis(DiffVisCell),
    Sql(DiffSqlCell),
}

#[derive(Serialize, Debug, Clone)]
pub enum OutCellType {
    Code,
    Markdown,
    Sql,
    Vis,
    Empty,
}

impl DiffCell {
    pub fn from_cell(cell: &Cell, idx: usize, colored: Option<bool>) -> DiffCell {
        let colored = colored.unwrap_or(false);

        match cell.cell_type {
            CellType::Code => DiffCell::Code(DiffCodeCell::from_cell(cell, idx, colored)),
            CellType::Markdown => {
                DiffCell::Markdown(DiffMarkdownCell::from_cell(cell, idx, colored))
            }
            CellType::Sql => DiffCell::Sql(DiffSqlCell::from_cell(cell, idx, colored)),
            CellType::Visualization => DiffCell::Vis(DiffVisCell::from_cell(cell, idx, colored)),
            CellType::DataExploration => DiffCell::Code(DiffCodeCell {
                idx,
                lines: Vec::new(),
            }),
        }
    }

    pub fn set_lines(&mut self, lines: Vec<DiffLine>) {
        match self {
            DiffCell::Code(cell) => cell.set_lines(lines),
            DiffCell::Markdown(cell) => cell.set_lines(lines),
            DiffCell::Sql(cell) => cell.set_lines(lines),
            DiffCell::Vis(cell) => cell.set_lines(lines),
        }
    }

    pub fn get_idx(&self) -> usize {
        match self {
            DiffCell::Code(cell) => cell.idx,
            DiffCell::Markdown(cell) => cell.idx,
            DiffCell::Sql(cell) => cell.idx,
            DiffCell::Vis(cell) => cell.idx,
        }
    }

    pub fn get_type(&self) -> OutCellType {
        match self {
            DiffCell::Code(_) => OutCellType::Code,
            DiffCell::Markdown(_) => OutCellType::Markdown,
            DiffCell::Sql(_) => OutCellType::Sql,
            DiffCell::Vis(_) => OutCellType::Vis,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffCodeCell {
    pub idx: usize,
    pub lines: Vec<DiffLine>,
}
impl DiffCodeCell {
    pub fn from_cell(cell: &Cell, cell_idx: usize, colored: bool) -> DiffCodeCell {
        let mut lines = Vec::new();
        for (idx, line) in cell.source.iter().enumerate() {
            lines.push(DiffLine {
                idx: Some(idx),
                colored,
                content: line.to_string(),
            });
        }
        DiffCodeCell {
            idx: cell_idx,
            lines,
        }
    }

    pub fn set_lines(&mut self, lines: Vec<DiffLine>) {
        self.lines = lines;
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DiffMarkdownCell {
    pub idx: usize,
    pub lines: Vec<DiffLine>,
}
impl DiffMarkdownCell {
    pub fn from_cell(cell: &Cell, cell_idx: usize, colored: bool) -> DiffMarkdownCell {
        let mut lines = Vec::new();
        for (idx, line) in cell.source.iter().enumerate() {
            lines.push(DiffLine {
                idx: Some(idx),
                colored,
                content: line.to_string(),
            });
        }
        DiffMarkdownCell {
            idx: cell_idx,
            lines,
        }
    }

    pub(crate) fn set_lines(&mut self, lines: Vec<DiffLine>) {
        self.lines = lines;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffVisCell {
    pub idx: usize,
    pub df_name: (bool, Option<String>),
    pub show_table: (bool, Option<String>),
    pub x_col: (bool, String),
    pub y_col: (bool, String),
    pub color_col: (bool, String),
    pub pic_type: (bool, Option<String>),
    pub title: (bool, Option<String>),
}
impl DiffVisCell {
    pub fn from_cell(_cell: &Cell, idx: usize, colored: bool) -> DiffVisCell {
        DiffVisCell {
            idx,
            df_name: (colored, None),
            show_table: (colored, None),
            x_col: (colored, "".to_string()),
            y_col: (colored, "".to_string()),
            color_col: (colored, "".to_string()),
            pic_type: (colored, None),
            title: (colored, None),
        }
    }

    pub fn set_lines(&mut self, _lines: Vec<DiffLine>) {}
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffSqlCell {
    pub idx: usize,
    pub df_name: (bool, Option<String>),
    pub data_source: (bool, String),
    pub lines: Vec<DiffLine>,
}
impl DiffSqlCell {
    pub fn from_cell(cell: &Cell, cell_idx: usize, colored: bool) -> DiffSqlCell {
        let mut lines = Vec::new();
        for (idx, line) in cell.source.iter().enumerate() {
            lines.push(DiffLine {
                idx: Some(idx),
                colored,
                content: line.to_string(),
            });
        }
        DiffSqlCell {
            idx: cell_idx,
            df_name: (colored, None),
            data_source: (colored, "".to_string()),
            lines,
        }
    }

    pub fn set_lines(&mut self, lines: Vec<DiffLine>) {
        self.lines = lines;
    }
}
