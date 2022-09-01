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

use common_model::entity::notebook::Notebook;
use common_model::enums::mime::Mimetype;
use common_tools::file_tool;
use err::Result;
use redis::AsyncCommands;

use super::CacheService;
use crate::keys::ipynb_key;
use crate::RefreshDto;

impl CacheService {
    pub(crate) async fn synchronize_notebook<P: AsRef<Path>>(
        &self,
        path: P,
        project_id: u64,
    ) -> Result<Notebook> {
        tracing::debug!("synchronize_notebook-->,path:{:?}", path.as_ref());
        let mut notebook_disk_data = file_tool::read_notebook_from_disk(&path).await?;

        // Maybe the file's path length of has changed, so update to the path corresponding to the key and put it in the cache
        notebook_disk_data.set_path(&path);
        notebook_disk_data.update_last_modified_time();
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        // base information is put into the cache
        self.pool
            .get()
            .await?
            .hset(
                key,
                "base",
                serde_json::to_string(&(notebook_disk_data.base))?,
            )
            .await?;
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        // whole of cells store to redis cache if exists(not empty)
        if !notebook_disk_data.cells.is_empty() {
            let cells_tuple_vec = notebook_cells_to_tuple(&notebook_disk_data);
            self.pool
                .get()
                .await?
                .hset_multiple(&key, &cells_tuple_vec)
                .await?;
            //Set the expiration time. fixme: This place seems to sometimes fail to set the expiration time
            self.pool.get().await?.expire(&key, 60 * 5).await?;
        }

        self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
            .await;

        tracing::debug!(
            "<-- synchronize_notebook!path:{:?}",
            path.as_ref().as_os_str(),
        );

        Ok(notebook_disk_data)
    }

    pub async fn send_refresh_signal<P: AsRef<Path>>(
        &self,
        path: P,
        key: String,
        file_type: Mimetype,
        project_id: u64,
    ) {
        let refresh_dto = RefreshDto::new(
            key,
            path.as_ref().to_str().unwrap().to_string(),
            file_type,
            project_id,
        );
        if let Err(err) = self.refresh_sender.send(refresh_dto).await {
            tracing::error!("{err}");
        }
    }

    pub async fn update_notebook<P: AsRef<Path>>(
        &self,
        path: P,
        mut notebook: Notebook,
        project_id: u64,
    ) -> Result<()> {
        let mut is_send_signal = true;

        notebook.update_last_modified_time();

        if !path.as_ref().exists() {
            common_tools::file_tool::write_notebook_to_disk(&path, &notebook).await?;
            is_send_signal = false;
        }

        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        //require delete cache data
        self.pool.get().await?.del(&key).await?;

        self.pool
            .get()
            .await?
            .hset(
                key,
                "base",
                serde_json::to_string(&(notebook.base)).unwrap(),
            )
            .await?;
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        // whole of cells store to redis cache if exists(not empty)
        if !notebook.cells.is_empty() {
            let cells_tuple_vec = notebook_cells_to_tuple(&notebook);
            self.pool
                .get()
                .await?
                .hset_multiple(&key, &cells_tuple_vec)
                .await?;
        }

        //The expiration time should be reset to the maximum time.
        self.pool.get().await?.expire(&key, 60 * 5).await?;

        if is_send_signal {
            self.send_refresh_signal(&path, key, Mimetype::Notebook, project_id)
                .await;
        }

        Ok(())
    }
}

fn notebook_cells_to_tuple(notebook: &Notebook) -> Vec<(String, String)> {
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
                    "this cell has no cell_id:{:?}, new uuid str:{}",
                    new_cell,
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
