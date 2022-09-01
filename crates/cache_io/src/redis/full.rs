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

use chrono::SecondsFormat;
use common_model::entity::notebook::Notebook;
use common_model::enums::mime::Mimetype;
use err::Result;
use redis::AsyncCommands;
use tracing::debug;

use super::refresh_disk::FileContent;
use super::CacheService;
use crate::keys::ipynb_key;
/// file r/w require absolute_path( composed of team_id,project_id,relative_path);
///
/// read: path(absolute_path) => get_inode => read_cache_by_inode => if exists: return end; if does
/// not exists => read_disk => if exists: return and update cache end; if does not exists: => return
/// error end;
///
/// write: path(absolute_path) + content(str/notebook) => judge the file whether exists => if
/// exists: get_inode and update_cache end if does not exists: => create file,write to
/// disk,get_inode and update_cache end;
///
/// partial_update_cell: path(absolute_path) + cell => get_inode and cell_id => read_cache => if
/// exists: update_cache ,if does not exists: => read_disk(not exists return error) => update_cache
/// end;
///
impl CacheService {
    pub async fn update_file_content<P: AsRef<Path>>(
        &self,
        path: P,
        content: String,
        file_type: Mimetype,
        project_id: u64,
    ) -> Result<()> {
        if !path.as_ref().exists() || file_type == Mimetype::Image || file_type == Mimetype::Text {
            common_tools::file_tool::write_large_to_nfs(&path, content.clone(), file_type).await?;
            // is_send_signal = false;
            return Ok(());
        }
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        let last_modified = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        let file_content = FileContent {
            length: content.len(),
            last_modified,
            content,
        };
        let val = serde_json::to_string(&file_content)?;
        let mut conn = self.pool.get().await?;
        conn.set_ex(&key, val, 2 * 60).await?;
        self.send_refresh_signal(&path, key, Mimetype::Text, project_id)
            .await;
        Ok(())
    }

    /// notebook cache design
    /// data_type:hash
    /// hkey:file_path key:cell id value:cell value (There is a default key as base which contains other information of the notebook file)
    /// is_from_disk: if true,will read data from disk and update cache,else priority read cache.
    /// TODO: Reuse functions that loaded from disk
    pub async fn read_notebook<P: AsRef<Path>>(
        &self,
        path: P,
        project_id: u64,
    ) -> Result<Notebook> {
        // get cache
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        let mut val_vec = self.pool.get().await?.hvals::<_, Vec<String>>(&key).await?;

        // Cache does not exist.Load from disk and put into cache
        if val_vec.is_empty() {
            debug!("cache is_empty!read notebook from disk");
            // Sync data from disk to cache
            self.synchronize_notebook(&path, project_id).await?;

            // Get the cache again copy it to empty val
            val_vec = self.pool.get().await?.hvals(&key).await?;
            debug_assert!(!val_vec.is_empty());
        }

        // debug!("redis cache...");
        let notebook = crate::vec_string_into_notebook(val_vec)?;

        self.pool.get().await?.expire(key, 60 * 5).await?;

        Ok(notebook)
    }
    pub(crate) async fn synchronize_notebook_if_not_exists<P: AsRef<Path>>(
        &self,
        path: P,
        project_id: u64,
    ) -> Result<()> {
        let key = ipynb_key(path.as_ref().to_str().unwrap(), project_id);
        let val_vec = self.pool.get().await?.hvals::<_, Vec<String>>(key).await?;

        if val_vec.is_empty() {
            debug!("cache is_empty!read notebook from disk");
            //Sync data from disk to cache
            self.synchronize_notebook(&path, project_id).await?;
        } else {
            debug!("cache exists.")
        }
        Ok(())
    }
}
