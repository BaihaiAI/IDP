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
use std::path::PathBuf;

use bb8_redis::redis;
use common_model::entity::cell::Cell;
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
    #[tracing::instrument]
    pub async fn read_cell<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        cell_id: &str,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let path = path.as_ref().to_str().unwrap();
        let key = ipynb_key(path, project_id);
        let mut conn = self.pool.get().await?;
        let cell_option = conn.hget::<_, _, Option<String>>(key, cell_id).await?;

        // If cell's cache is empty,try to load it into the cache.
        match cell_option {
            None => {
                tracing::info!("cache not found cell, sync fs->redis");
                self.synchronize_notebook(&path, project_id).await?;
                let key = ipynb_key(path, project_id);
                let cell_opt: Option<String> = conn.hget(key, cell_id).await?;

                match cell_opt {
                    None => Err(ErrorTrace::new(&format!(
                        "cell_id {cell_id} not found in {path}(fs and redis)"
                    ))),
                    Some(cell_str) => Ok(serde_json::from_str::<Cell>(&cell_str)?),
                }
            }
            Some(cell_str) => Ok(serde_json::from_str::<Cell>(&cell_str).map_err(|err| {
                ErrorTrace::new(&format!("{path} {cell_id} cell_str={cell_str} {err}"))
            })?),
        }
    }

    /*
    ## dirty read
    req2 has update data1 to data2
    req1 write old data1
    ```
    (req1)1666253861.661278 [0 172.16.29.162:51822] "HGET" "ipynb_114_/store/1581166227778703360/projects/114/notebooks/train_model.idpnb" "36c5dbad-e69b-4506-a98c-a59ac9edbdb8"
    1666253861.661339 [0 172.16.29.162:51834] "HGET" "ipynb_114_/store/1581166227778703360/projects/114/notebooks/train_model.idpnb" "be2f1755-3df2-4a62-814d-6db187c2b236"
    (req2)1666253861.662708 [0 172.16.29.162:51850] "HSET" "ipynb_114_/store/1581166227778703360/projects/114/notebooks/train_model.idpnb" "36c5dbad-e69b-4506-a98c-a59ac9edbdb8" "{\"cell_type\":\"code\",\"outputs\":[],\"source\":[\"# cat train_model.idpnb | jq .cells[2]\\n\",\"data['day'] = data.imp_time.apply(lambda a : pd.to_datetime(a).day)\"],\"execution_time\":\"1\",\"execution_count\":15,\"metadata\":{\"id\":\"36c5dbad-e69b-4506-a98c-a59ac9edbdb8\",\"index\":4.0}}"
    (req1)1666253861.663339 [0 172.16.29.162:39836] "HSET" "ipynb_114_/store/1581166227778703360/projects/114/notebooks/train_model.idpnb" "36c5dbad-e69b-4506-a98c-a59ac9edbdb8" "{\"cell_type\":\"code\",\"outputs\":[{\"ename\":\"KeyboardInterrupt
    ```
    since redis has no transaction, so we use note_storage process memory to lock the key for update
    */
    #[tracing::instrument]
    pub async fn update_cell<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        mut cell: Cell,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        // The front-end will not call the cat api when the tab page tab file is not closed,so it must be judged to ensure that cell is loaded into cache
        self.synchronize_notebook_if_not_exists(&path, project_id)
            .await?;
        let path = path.as_ref().to_str().unwrap();

        /*
        let key = ipynb_key(path, project_id);
        if let Some(base_info) = base_info {
            let mut base_info_obj = match serde_json::from_str::<BaseInfo>(&base_info) {
                Ok(obj) => obj,
                Err(err) => {
                    tracing::error!("wrong base_info json_str {base_info} in {path}");
                    return Err(ErrorTrace::from(err));
                }
            };
            let key = ipynb_key(path, project_id);
            self.pool
                .get()
                .await?
                .hset(key, "base", serde_json::to_string(&base_info_obj).unwrap())
                .await?;
        }
        */

        let cell_id = if let Some(cell_id) = cell.id() {
            cell_id
        } else {
            let cell_id = common_model::entity::cell::Uuid::new_v4();
            tracing::warn!(
                "cell {:?}{:?} has no cell_id, new uuid str:{}",
                cell.index(),
                cell.source,
                cell_id.to_string()
            );
            cell.set_id(cell_id);
            cell_id.to_string()
        };
        debug!("update_cell cache:{}", cell_id);
        let key = ipynb_key(path, project_id);
        self.pool
            .get()
            .await?
            .hset(key, cell_id, serde_json::to_string::<Cell>(&cell)?)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(cell_update))]
    pub async fn partial_update_cell(
        &self,
        path: &std::path::PathBuf,
        cell_update: common_model::entity::cell::CellUpdate,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let cell_id = &cell_update.id;

        let lock_key = (path.clone(), cell_id.clone());
        let mut lock_for_update = self.lock_for_update.write().await;
        let lock = lock_for_update
            .entry(lock_key.clone())
            .or_insert_with(|| std::sync::Arc::new(tokio::sync::Mutex::new(())))
            .clone();
        drop(lock_for_update);
        let lock_guard = lock.lock().await;

        let mut cell = self.read_cell(&path, &cell_update.id, project_id).await?;
        tracing::info!(
            "partial_update_cell: read_cell_before_write cell_id={}, output.len()={}",
            cell_update.id,
            cell.outputs.len()
        );
        let mut has_change = false;
        if let Some(cell_type) = cell_update.updates.cell_type {
            if cell.cell_type != cell_type {
                cell.cell_type = cell_type;
                has_change = true;
            }
        }
        if let Some(outputs) = cell_update.updates.outputs {
            if cell.outputs != outputs {
                cell.outputs = outputs;
                has_change = true;
            }
        }
        if let Some(source) = cell_update.updates.source {
            if cell.source != source {
                cell.source = source;
                has_change = true;
            }
        }
        if let Some(execution_time) = cell_update.updates.execution_time {
            if cell.execution_time != Some(execution_time.clone()) {
                cell.execution_time = Some(execution_time);
                has_change = true;
            }
        }
        if let Some(execution_count) = cell_update.updates.execution_count {
            if cell.execution_count != Some(execution_count) {
                cell.execution_count = Some(execution_count);
                has_change = true;
            }
        }
        if let Some(metadata) = cell_update.updates.metadata {
            // e.g. visual cell change
            if cell.metadata != metadata {
                cell.metadata = metadata;
                has_change = true;
            }
        }

        if !has_change {
            tracing::info!("cell_id={cell_id} no change, skip update to redis");
            return Ok(());
        }

        self.update_cell(&path, cell, project_id).await?;
        drop(lock_guard);
        let mut lock_for_update2 = self.lock_for_update.write().await;
        lock_for_update2.remove(&lock_key);
        drop(lock_for_update2);

        let key = ipynb_key(path.to_str().unwrap(), project_id);
        self.send_persist_redis_to_fs_signal(&path, key, project_id)
            .await;

        Ok(())
    }
}

impl CacheService {
    /// neighbor_cell_id: the relevant cell_id (up is the above, down is the following)
    /// move: swap the index
    /// TODO: Temporarily does not support cell dragging move.
    #[tracing::instrument]
    pub async fn move_cell<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        neighbor_cell_id: String,
        cell_id: String,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let mut move_cell = self.read_cell(&path, &cell_id, project_id).await?;
        let mut neighbor_cell = self.read_cell(&path, &neighbor_cell_id, project_id).await?;

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
        self.send_persist_redis_to_fs_signal(&path, key, project_id)
            .await;

        Ok(())
    }
    // TODO if cache not exist,maybe cell will be cleaned up?
    #[tracing::instrument]
    pub async fn delete_cell(
        &self,
        path: PathBuf,
        cell_id: String,
        project_id: u64,
    ) -> Result<(), ErrorTrace> {
        let key = ipynb_key(path.to_str().unwrap(), project_id);
        self.synchronize_notebook_if_not_exists(&path, project_id)
            .await?;
        self.pool.get().await?.hdel(&key, cell_id).await?;
        self.send_persist_redis_to_fs_signal(&path, key, project_id)
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
    #[tracing::instrument]
    pub async fn insert_cell(
        &self,
        path: PathBuf,
        mut cell: Cell,
        // index is not available in jupyter. It is convenient to insert with float (Take the average of the two cells before and after as the index of insertion) to improve performance.
        above_cell_index: Option<f64>,
        under_cell_index: Option<f64>,
        insert_flag: usize,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let key = ipynb_key(path.to_str().unwrap(), project_id);
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

        self.send_persist_redis_to_fs_signal(&path, key, project_id)
            .await;

        Ok(cell)
    }

    /// add_cell using the specific cell structure
    #[tracing::instrument]
    pub async fn add_cell(
        &self,
        path: PathBuf,
        cell: Cell,
        project_id: u64,
    ) -> Result<Cell, ErrorTrace> {
        let key = ipynb_key(path.to_str().unwrap(), project_id);
        debug!("add_cell key:{} cell:{:?}", key, cell);

        self.update_cell(&path, cell.clone(), project_id).await?;

        self.send_persist_redis_to_fs_signal(&path, key, project_id)
            .await;

        Ok(cell)
    }
}
