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

use bb8_redis::redis::AsyncCommands;
use common_model::entity::notebook::Notebook;
use common_tools::file_tool;
use err::ErrorTrace;
use err::Result;

use super::CacheService;
use crate::keys::ipynb_key;
use crate::RefreshDto;

impl CacheService {
    #[tracing::instrument]
    pub(crate) async fn synchronize_notebook<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        project_id: u64,
    ) -> Result<Notebook> {
        tracing::info!("--> synchronize_notebook");
        let path = path.as_ref().to_str().unwrap();
        let notebook_disk_data = file_tool::read_notebook_from_disk(path).await?;

        // Maybe the file's path length of has changed, so update to the path corresponding to the key and put it in the cache
        // notebook_disk_data.set_path(&path);
        let key = ipynb_key(path, project_id);
        // base information is put into the cache
        let mut conn = self.pool.get().await?;
        // whole of cells store to redis cache if exists(not empty)

        if notebook_disk_data.cells.is_empty() {
            return Err(ErrorTrace::new("notebook on fs cells is empty"));
        }

        let cells_tuple_vec = notebook_cells_to_redis_hvals(&notebook_disk_data);
        conn.hset_multiple(&key, &cells_tuple_vec).await?;
        //Set the expiration time. fixme: This place seems to sometimes fail to set the expiration time
        conn.expire(&key, crate::IPYNB_CACHE_TTL).await?;
        let cell_ids = cells_tuple_vec.into_iter().map(|x| x.0).collect::<Vec<_>>();
        tracing::info!("after hset {cell_ids:?}");

        // ??? redis->fs, why write back here?
        // self.send_persist_redis_to_fs_signal(&path, key, Mimetype::Notebook, project_id).await;

        Ok(notebook_disk_data)
    }

    #[tracing::instrument]
    pub async fn send_persist_redis_to_fs_signal<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        key: String,
        project_id: u64,
    ) {
        // debug_assert_eq!(file_type, Mimetype::Notebook);
        let refresh_dto =
            RefreshDto::new(key, path.as_ref().to_str().unwrap().to_string(), project_id);
        if self.refresh_sender.send(refresh_dto).await.is_err() {
            tracing::error!("refresh_sender send fail, refresh_receiver thread panicked?");
        }
    }

    #[tracing::instrument(skip(notebook))]
    pub async fn snapshot_restore<P: AsRef<Path> + std::fmt::Debug>(
        &self,
        path: P,
        notebook: Notebook,
        project_id: u64,
    ) -> Result<()> {
        if !path.as_ref().exists() {
            return Err(ErrorTrace::new(
                "panicked! cache inconsistent redis exist but fs not exist",
            ));
            // common_tools::file_tool::write_notebook_to_disk(&path, &notebook).await?;
            // is_send_signal = false;
        }

        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        //require delete cache data
        let mut conn = self.pool.get().await?;
        conn.del(&key).await?;

        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        // whole of cells store to redis cache if exists(not empty)
        if notebook.cells.is_empty() {
            return Err(ErrorTrace::new(&format!("cell is empty {key}")));
        }

        //The expiration time should be reset to the maximum time.
        let cells_tuple_vec = notebook_cells_to_redis_hvals(&notebook);
        conn.hset_multiple(&key, &cells_tuple_vec).await?;
        conn.expire(&key, crate::IPYNB_CACHE_TTL).await?;

        // current function only fs->redis, we don't write redis->fs back
        // self.send_persist_redis_to_fs_signal(&path, key, Mimetype::Notebook, project_id)
        //     .await;

        Ok(())
    }
}

/// See Also: read_notebook_from_disk()
fn notebook_cells_to_redis_hvals(notebook: &Notebook) -> Vec<(String, String)> {
    notebook
        .cells
        .iter()
        .map(|cell| {
            if let Some(cell_id) = cell.id() {
                (cell_id, serde_json::to_string(cell).unwrap())
            } else {
                // if cell have no id,new uuid as cell_id
                let cell_id = common_model::entity::cell::Uuid::new_v4();
                let mut new_cell = cell.clone();
                tracing::warn!(
                    "cell {:?}{:?} has no cell_id, new uuid str:{}",
                    cell.index(),
                    cell.source,
                    cell_id.to_string()
                );
                new_cell.set_id(cell_id);
                (
                    cell_id.to_string(),
                    serde_json::to_string(&new_cell).unwrap(),
                )
            }
        })
        .collect::<Vec<(String, String)>>()
}
