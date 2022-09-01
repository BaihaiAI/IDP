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

use common_model::entity::cell::Cell;
use common_model::entity::notebook::BaseInfo;
use common_model::enums::mime::Mimetype;
use err::ErrorTrace;
use redis::AsyncCommands;
use tracing::debug;
use tracing::error;

use super::CacheService;
use crate::keys::ipynb_key;

mod status {
    pub const CELL_LACK_FIELD_ERROR_CODE: u32 = 51_002_007;
    pub const CELL_LACK_FIELD_ERROR_MSG: &str =
        "cell lack required field!,index or id or others...";
}

impl CacheService {
    pub async fn read_cell<P: AsRef<Path>>(
        &self,
        path: P,
        cell_id: String,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        let cell_option = self
            .pool
            .get()
            .await?
            .hget::<_, _, Option<String>>(key, &cell_id)
            .await?;

        //If cell's cache is empty,try to load it into the cache.
        match cell_option {
            None => {
                tracing::info!("cache not found cell, synchronize_notebook from disk");

                self.synchronize_notebook(&path, project_id).await?;
                let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
                let cell_opt: Option<String> = self.pool.get().await?.hget(key, &cell_id).await?;

                match cell_opt {
                    None => {
                        tracing::warn!(
                            "(cell_id not found in file)path:{:?} cell_id:{}",
                            path.as_ref(),
                            cell_id
                        );
                        Err(ErrorTrace::new("not found cell in disk and cache."))
                    }
                    Some(cell_str) => Ok(serde_json::from_str::<Cell>(&cell_str)?),
                }
            }
            Some(cell_str) => Ok(serde_json::from_str::<Cell>(&cell_str)?),
        }
    }

    pub async fn update_cell<P: AsRef<Path>>(
        &self,
        path: P,
        mut cell: Cell,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        //The front-end will not call the cat api when the tab page tab file is not closed,so it must be judged to ensure that cell is loaded into cache
        self.synchronize_notebook_if_not_exists(&path, project_id)
            .await?;
        let path = path.as_ref().to_str().unwrap();
        let key = ipynb_key(path, project_id);
        let base_info: Option<String> = self.pool.get().await?.hget(key, "base").await?;

        if let Some(base_info) = base_info {
            let mut base_info_obj = match serde_json::from_str::<BaseInfo>(&base_info) {
                Ok(obj) => obj,
                Err(err) => {
                    tracing::error!("wrong base_info json_str {base_info} in {path}");
                    return Err(ErrorTrace::from(err));
                }
            };
            base_info_obj.update_last_modified_time();

            let key = ipynb_key(path, project_id);

            self.pool
                .get()
                .await?
                .hset(key, "base", serde_json::to_string(&base_info_obj).unwrap())
                .await?;
        }

        if cell.id().is_none() {
            let cell_id = common_model::entity::cell::Uuid::new_v4();
            tracing::warn!(
                "this cell has no cell_id:{:?}, new uuid str:{}",
                cell,
                cell_id.to_string()
            );
            cell.set_id(cell_id);
        }
        let cell_id = cell.id().unwrap();
        debug!("update_cell cache:{}", cell_id);
        let key = ipynb_key(path, project_id);
        self.pool
            .get()
            .await?
            .hset(key, cell_id, serde_json::to_string::<Cell>(&cell)?)
            .await?;

        Ok(())
    }

    pub async fn partial_update_cell(
        &self,
        path: &std::path::PathBuf,
        cell_update: common_model::entity::cell::CellUpdate,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let mut cell = self.read_cell(&path, cell_update.id, project_id).await?;
        if let Some(cell_type) = cell_update.updates.cell_type {
            cell.cell_type = cell_type;
        }
        if let Some(outputs) = cell_update.updates.outputs {
            cell.outputs = outputs;
        }
        if let Some(source) = cell_update.updates.source {
            cell.source = source;
        }
        if let Some(execution_time) = cell_update.updates.execution_time {
            cell.execution_time = Some(execution_time);
        }
        if let Some(execution_count) = cell_update.updates.execution_count {
            cell.execution_count = Some(execution_count);
        }

        if let Some(metadata) = cell_update.updates.metadata {
            cell.metadata = metadata;
        }

        //update cell
        self.update_cell(&path, cell, project_id).await?;
        let key = ipynb_key(path.to_str().unwrap(), project_id);
        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        Ok(())
    }
}

impl CacheService {
    /// neighbor_cell_id: the relevant cell_id (up is the above, down is the following)
    /// move: swap the index
    /// TODO: Temporarily does not support cell dragging move.
    pub async fn move_cell<P: AsRef<Path>>(
        &self,
        path: P,
        neighbor_cell_id: String,
        cell_id: String,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let mut move_cell = self.read_cell(&path, cell_id, project_id).await?;
        let mut neighbor_cell = self.read_cell(&path, neighbor_cell_id, project_id).await?;

        if let Some(temp_index) = move_cell.index() {
            if let Some(neighbor_cell_index) = neighbor_cell.index() {
                move_cell.set_index(neighbor_cell_index);
                neighbor_cell.set_index(temp_index);
            } else {
                error!("{}", status::CELL_LACK_FIELD_ERROR_MSG);
                return Err(ErrorTrace::new(status::CELL_LACK_FIELD_ERROR_MSG)
                    .code(status::CELL_LACK_FIELD_ERROR_CODE));
            }
        } else {
            error!("{}", status::CELL_LACK_FIELD_ERROR_MSG);
            return Err(ErrorTrace::new(status::CELL_LACK_FIELD_ERROR_MSG)
                .code(status::CELL_LACK_FIELD_ERROR_CODE));
        }

        self.update_cell(&path, move_cell, project_id).await?;
        self.update_cell(&path, neighbor_cell, project_id).await?;
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        Ok(())
    }
    // TODO if cache not exist,maybe cell will be cleaned up?
    pub async fn delete_cell<P: AsRef<Path>>(
        &self,
        path: P,
        cell_id: String,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        self.synchronize_notebook_if_not_exists(&path, project_id)
            .await?;
        self.pool.get().await?.hdel(&key, cell_id).await?;
        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        Ok(())
    }

    ///insert_cell insert a new cell using the given index.
    /// should judge whether it is a notebook in cache to prevent being overwritten.
    ///insert_flag:
    /// 0: Insert in the middle, there have above_cell_index and under_cell_index. (above_cell_index+under_cell_index) /2
    /// 1: Insert at the top, only above_cell_index. cell_index == above_cell_index / 2;
    /// 2: Insert at the end, only under_cell_index. cell_index == under_cell_index + 1;
    /// return :this cell after changed.
    pub async fn insert_cell<P: AsRef<Path>>(
        &self,
        path: P,
        mut cell: Cell,
        // index is not available in jupyter. It is convenient to insert with float (Take the average of the two cells before and after as the index of insertion) to improve performance.
        above_cell_index: Option<f64>,
        under_cell_index: Option<f64>,
        insert_flag: usize,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        match insert_flag {
            0 => {
                let above_cell_index = above_cell_index.unwrap();
                let under_cell_index = under_cell_index.unwrap();
                let inserted_cell_index = (above_cell_index + under_cell_index) / 2.0;
                cell.set_index(inserted_cell_index);
            }
            1 => {
                let above_cell_index = above_cell_index.unwrap();
                let inserted_cell_index = above_cell_index / 2.0;
                cell.set_index(inserted_cell_index);
            }
            2 => {
                let under_cell_index = under_cell_index.unwrap();
                let inserted_cell_index = under_cell_index + 1.0;
                cell.set_index(inserted_cell_index);
            }
            _ => {
                error!("ILLEGAL INSERT FLAG!");
                return Err(ErrorTrace::new("ILLEGAL INSERT FLAG!"));
            }
        }

        self.update_cell(&path, cell.clone(), project_id).await?;

        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        Ok(cell)
    }

    /// add_cell using the specific cell structure
    pub async fn add_cell<P: AsRef<Path>>(
        &self,
        path: P,
        cell: Cell,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        debug!("add_cell key:{} cell:{:?}", key, cell);

        self.update_cell(&path, cell.clone(), project_id).await?;

        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        Ok(cell)
    }
}
