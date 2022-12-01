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

use std::collections::VecDeque;

use common_model::entity::cell::Cell;
use common_model::entity::notebook::Notebook;
use json_structural_diff::JsonDiff;
use serde_json::json;
use serde_json::Value;

use super::diff_models::DiffCell;
use super::diff_models::DiffCellOut;
use super::diff_models::DiffLine;
use super::diff_models::DiffSqlCell;
use super::diff_models::DiffVisCell;
use super::models::SnapshotDiffRes;

pub fn cell_id_update(
    s: &str,
    idx: usize,
    cell_idx1: &mut [usize],
    cell_idx2: &mut [usize],
) -> Result<(), std::num::ParseIntError> {
    let s = s.replace('"', "");
    let _t = s.trim().split(':').collect::<Vec<&str>>();
    let ori_id = _t[1].trim().parse::<usize>()?;
    if _t[0].trim() == "cell1_idx" {
        cell_idx1[ori_id] = idx
    } else {
        cell_idx2[ori_id] = idx
    }

    Ok(())
}

pub fn cells_to_diff_cells(cells: Vec<Cell>) -> Vec<DiffCell> {
    let mut diff_cells = Vec::new();
    for (idx, cell) in cells.iter().enumerate() {
        diff_cells.push(DiffCell::from_cell(cell, idx, None));
    }
    diff_cells
}

fn compare_sql_cell(
    _cell1: &Cell,
    _cell2: &Cell,
    _diff_cell1: &mut DiffSqlCell,
    _diff_cell2: &mut DiffSqlCell,
) {
}

fn compare_vis_cell(
    _cell1: &Cell,
    _cell2: &Cell,
    _diff_cell1: &mut DiffVisCell,
    _diff_cell2: &mut DiffVisCell,
) {
}

pub fn diff_notebooks(
    mut cells1: Vec<Cell>,
    mut cells2: Vec<Cell>,
) -> Result<(Vec<DiffCell>, Vec<DiffCell>), std::num::ParseIntError> {
    tracing::info!("--> diff_notebooks");
    if cells1 == cells2 {
        let diff_cells = cells_to_diff_cells(cells1);
        return Ok((diff_cells.clone(), diff_cells));
    }
    let mut cell_idx1 = vec![];
    let mut cell_idx2 = vec![];
    for (idx, cell) in cells1.iter_mut().enumerate() {
        cell.execution_time = Some(format!("cell1_idx: {}", idx));
        cell_idx1.push(idx);
    }
    for (idx, cell) in cells2.iter_mut().enumerate() {
        cell.execution_time = Some(format!("cell2_idx: {}", idx));
        cell_idx2.push(idx);
    }

    let json_diff = match JsonDiff::diff(
        &serde_json::json!(cells1),
        &serde_json::json!(cells2),
        false,
    )
    .diff
    {
        Some(diff) => diff,
        None => {
            tracing::info!("{}", line!());
            let diff_cells = cells_to_diff_cells(cells1);
            return Ok((diff_cells.clone(), diff_cells));
        }
    };

    tracing::info!("{}", line!());
    let diff_list = match json_diff.as_array() {
        Some(diff_list) => diff_list,
        None => {
            tracing::info!("{}", line!());
            let diff_cells = cells_to_diff_cells(cells1);
            return Ok((diff_cells.clone(), diff_cells));
        }
    };

    tracing::info!("{}", line!());
    for (idx, df) in diff_list.iter().enumerate() {
        let _t = Vec::<serde_json::Value>::new();
        let _tmp = df.as_array().unwrap_or(&_t);
        let _t = "".to_owned();
        let _df_type = _tmp[0].as_str().unwrap_or(&_t);
        let df_content = _tmp[1].to_owned();

        let cell_original_idx = df_content["execution_time"].to_owned();
        match cell_original_idx.as_str() {
            Some(s) => {
                cell_id_update(s, idx, &mut cell_idx1, &mut cell_idx2)?;
            }
            None => {
                let new = cell_original_idx["__new"].to_owned().to_string();
                let old = cell_original_idx["__old"].to_owned().to_string();

                cell_id_update(&new, idx, &mut cell_idx1, &mut cell_idx2)?;
                cell_id_update(&old, idx, &mut cell_idx1, &mut cell_idx2)?;
            }
        }
    }

    let mut output_cells1 = vec![];
    let mut output_cells2 = vec![];

    let mut ori_idx2: usize = 0;
    tracing::info!("{}", line!());
    for (ori_idx1, diff_idx) in cell_idx1.clone().into_iter().enumerate() {
        let cell = cells1[ori_idx1].to_owned();
        match cell_idx2.iter().position(|&x| x == diff_idx) {
            Some(tmp_ori_idx2) => {
                for tmp_idx2 in ori_idx2..tmp_ori_idx2 {
                    output_cells2.push(DiffCell::from_cell(
                        &cells2[tmp_idx2].clone(),
                        cell_idx2[tmp_idx2],
                        Some(true),
                    ));
                }

                ori_idx2 = tmp_ori_idx2;
                let mut code1 = VecDeque::from(cells1[ori_idx1].source.clone());
                let mut code2 = VecDeque::from(cells2[tmp_ori_idx2].source.clone());

                let json_diff = match JsonDiff::diff(&json!(code1), &json!(code2), false).diff {
                    Some(diff) => diff,
                    None => {
                        output_cells1.push(DiffCell::from_cell(&cell, diff_idx, None));
                        output_cells2.push(DiffCell::from_cell(&cell, diff_idx, None));
                        ori_idx2 += 1;
                        continue;
                    }
                };

                let diff_list = match json_diff.as_array() {
                    Some(diff_list) => diff_list,
                    None => {
                        output_cells1.push(DiffCell::from_cell(&cell, diff_idx, None));
                        output_cells2.push(DiffCell::from_cell(&cell, diff_idx, None));
                        ori_idx2 += 1;
                        continue;
                    }
                };

                let mut diff_lines1 = vec![];
                let mut diff_lines2 = vec![];
                let mut flag1 = true;
                let mut flag2 = true;
                let code1_len = code1.len();
                let code2_len = code2.len();

                for (_line_idx, diff) in diff_list.iter().enumerate() {
                    let _t = Vec::<Value>::new();
                    let _tmp = diff.as_array().unwrap_or(&_t);
                    let _t = "".to_owned();
                    let df_type = _tmp[0].as_str().unwrap_or(&_t);
                    let df_content = _tmp[1].to_owned().to_string();
                    let df_content = df_content.strip_prefix('"').unwrap_or(&df_content);
                    let df_content = df_content.strip_suffix('"').unwrap_or(df_content);
                    let df_content = df_content.replace("\\n", "\n");

                    if flag1 && !code1.is_empty() {
                        let current_line1 = code1[0].clone();

                        if df_content == current_line1 {
                            diff_lines1.push(DiffLine {
                                idx: Some(code1_len - code1.len() + 1),
                                colored: df_type != " ",
                                content: df_content.to_string(),
                            });

                            code1.pop_front();
                            if code1.is_empty() {
                                flag1 = false;
                            }
                        } else {
                            diff_lines1.push(DiffLine {
                                idx: None,
                                colored: true,
                                content: "".to_string(),
                            });
                        }
                    } else {
                        diff_lines1.push(DiffLine {
                            idx: None,
                            colored: true,
                            content: "".to_string(),
                        });
                    }
                    if flag2 && !code2.is_empty() {
                        let current_line2 = code2[0].clone();

                        if df_content == current_line2 {
                            diff_lines2.push(DiffLine {
                                idx: Some(code2_len - code2.len() + 1),
                                colored: df_type != " ",
                                content: df_content.to_string(),
                            });

                            code2.pop_front();
                            if code2.is_empty() {
                                flag2 = false;
                            }
                        } else {
                            diff_lines2.push(DiffLine {
                                idx: None,
                                colored: true,
                                content: "".to_string(),
                            });
                        }
                    } else {
                        diff_lines1.push(DiffLine {
                            idx: None,
                            colored: true,
                            content: "".to_string(),
                        });
                    }
                }

                let mut diff_cell1;
                let mut diff_cell2;

                let cell_type1 = cells1[ori_idx1].cell_type.clone();
                let cell_type2 = cells2[ori_idx2].cell_type.clone();

                if cell_type1 == cell_type2 {
                    diff_cell1 = DiffCell::from_cell(&cells1[ori_idx1], diff_idx, None);
                    diff_cell2 = DiffCell::from_cell(&cells2[ori_idx2], diff_idx, None);

                    match (&mut diff_cell1, &mut diff_cell2) {
                        (DiffCell::Sql(c1), DiffCell::Sql(c2)) => {
                            compare_sql_cell(&cells1[ori_idx1], &cells2[ori_idx2], c1, c2);
                        }
                        (DiffCell::Vis(c1), DiffCell::Vis(c2)) => {
                            compare_vis_cell(&cells1[ori_idx1], &cells2[ori_idx2], c1, c2);
                        }
                        _ => {}
                    }

                    diff_cell1.set_lines(diff_lines1);
                    diff_cell2.set_lines(diff_lines2);
                } else {
                    diff_cell1 = DiffCell::from_cell(&cells1[ori_idx1], diff_idx, Some(true));
                    diff_cell2 = DiffCell::from_cell(&cells2[ori_idx2], diff_idx, Some(true));
                }

                output_cells1.push(diff_cell1);
                output_cells2.push(diff_cell2);

                ori_idx2 += 1
            }
            None => {
                output_cells1.push(DiffCell::from_cell(&cell, diff_idx, Some(true)));
            }
        }
    }

    // if cell2 more than cell1, add extra cell2
    tracing::info!("{}", line!());
    for tmp_idx2 in ori_idx2..cell_idx2.len() {
        output_cells2.push(DiffCell::from_cell(
            &cells2[tmp_idx2].clone(),
            cell_idx2[tmp_idx2],
            Some(true),
        ));
    }

    Ok((output_cells1, output_cells2))
}

impl SnapshotDiffRes {
    pub fn from_diff_cells(cells1: Vec<DiffCell>, cells2: Vec<DiffCell>) -> SnapshotDiffRes {
        tracing::info!("--> from_diff_cells");
        let mut diff_res = SnapshotDiffRes::default();

        let mut last_idx1 = 0;
        for (ori_idx, cell) in cells1.iter().enumerate() {
            let cell_idx = cell.get_idx();
            for _ in last_idx1..cell_idx {
                diff_res.cells1.push(DiffCellOut::gen_empty());
            }
            last_idx1 = cell_idx + 1;
            diff_res
                .cells1
                .push(DiffCellOut::from_diff_cell(cell.clone(), ori_idx));
        }
        last_idx1 = last_idx1.saturating_sub(1);

        let mut last_idx2 = 0;
        for (ori_idx, cell) in cells2.iter().enumerate() {
            let cell_idx = cell.get_idx();
            for _ in last_idx2..cell_idx {
                diff_res.cells2.push(DiffCellOut::gen_empty());
            }
            last_idx2 = cell_idx + 1;
            diff_res
                .cells2
                .push(DiffCellOut::from_diff_cell(cell.clone(), ori_idx));
        }
        last_idx2 = last_idx2.saturating_sub(1);

        #[allow(clippy::comparison_chain)]
        let (mut diff_res, _max_idx) = if last_idx1 == last_idx2 {
            (diff_res, last_idx1)
        } else if last_idx2 > last_idx1 {
            for _ in last_idx1..last_idx2 {
                diff_res.cells1.push(DiffCellOut::gen_empty());
            }
            (diff_res, last_idx2)
        } else {
            for _ in last_idx2..last_idx1 {
                diff_res.cells2.push(DiffCellOut::gen_empty());
            }
            (diff_res, last_idx1)
        };

        for idx in 0..(diff_res.cells1.len().min(diff_res.cells2.len())) {
            if diff_res.cells1[idx].data == diff_res.cells2[idx].data {
                diff_res.cells1[idx].same = true;
                diff_res.cells2[idx].same = true;
            }
        }

        diff_res
    }
}

pub async fn diff_notebook1_notebook2(
    notebook1: Notebook,
    notebook2: Notebook,
) -> Result<SnapshotDiffRes, std::num::ParseIntError> {
    tracing::info!("--> diff_notebook1_notebook2");
    let cells1 = notebook1.cells;
    let cells2 = notebook2.cells;

    tracing::debug!("cells1 = {cells1:#?}");
    tracing::debug!("cells2 = {cells2:#?}");
    let (diff_cells1, diff_cells2) = diff_notebooks(cells1, cells2)?;
    let snapshot_diff_res = SnapshotDiffRes::from_diff_cells(diff_cells1, diff_cells2);

    Ok(snapshot_diff_res)
}
