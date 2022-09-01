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

use business::business_term::ProjectId;
use business::business_term::TeamId;
use business::path_tool;
use business::path_tool::session_file_path;
use cache_io::CacheService;
use common_model::entity::cell::Cell;
use common_model::entity::cell::CellType;
use common_model::service::rsp::Rsp;
use uuid::Uuid;

use crate::common::error::IdpGlobalError;
use crate::handler::content::content_entrance::download_from_ftp;
pub mod cat;
pub mod partial_update_cell;
pub mod share_cell;
pub use cat::cat;
pub use cat::full_path_cat;
pub use partial_update_cell::put_cell;
pub use share_cell::share_cell;
pub mod save_cell;
pub use save_cell::save;
pub mod content_entrance;

/// insert_cell insert a new cell using the given index.
/// to prevent overwrite by disk file must judge notebook exist in cache
/// return :this cell after changed.
pub async fn insert_cell(
    path: String,
    insert_flag: usize,
    cell_type: CellType,
    above_cell_index: Option<f64>,
    under_cell_index: Option<f64>,
    team_id: TeamId,
    project_id: ProjectId,
    redis_cache: &mut CacheService,
) -> Result<Rsp<Cell>, IdpGlobalError> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    let cell = Cell::new(cell_type);
    let return_cell = redis_cache
        .insert_cell(
            &path,
            cell,
            above_cell_index,
            under_cell_index,
            insert_flag,
            project_id,
        )
        .await?;
    Ok(Rsp::success(return_cell))
}

/// add_cell using the specific cell structure
pub async fn add_cell(
    team_id: TeamId,
    project_id: ProjectId,
    path: String,
    mut cell: Cell,
    redis_cache: &mut CacheService,
) -> Result<Rsp<Cell>, IdpGlobalError> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, &path);
    if cell.id().is_none() {
        cell.set_id(Uuid::new_v4());
    }
    let return_cell = redis_cache.add_cell(path, cell, project_id).await?;
    Ok(Rsp::success(return_cell))
}

/// swap neighbor_cell_id and cell_id
pub async fn move_cell(
    path: String,
    team_id: u64,
    project_id: u64,
    neighbor_cell_id: String,
    cell_id: String,
    redis_cache: &mut CacheService,
) -> Result<Rsp<()>, IdpGlobalError> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    redis_cache
        .move_cell(&path, neighbor_cell_id, cell_id, project_id)
        .await?;

    Ok(Rsp::success(()))
}

pub async fn delete_cell(
    path: String,
    team_id: u64,
    project_id: u64,
    cell_id: String,
    redis_cache: &mut CacheService,
) -> Result<Rsp<()>, IdpGlobalError> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    redis_cache.delete_cell(&path, cell_id, project_id).await?;

    Ok(Rsp::success(()))
}

pub async fn load_cell(
    team_id: u64,
    project_id: u64,
    share_id: String,
) -> Result<Rsp<String>, IdpGlobalError> {
    let ftp_path = format!("/{}/share.ipynb", share_id);
    let ftp_session_path = format!("/{}/share.session", share_id);
    let path = format!("/shared/{}/share.ipynb", share_id);
    let abs_path = path_tool::get_store_full_path(team_id, project_id as u64, &path);

    let abs_path = abs_path.to_str().unwrap();
    if let Err(msg) = download_from_ftp(ftp_path, abs_path.to_string()) {
        return Err(IdpGlobalError::NoteError(msg));
    }

    // let inode = file_tool::get_inode_from_path(&local_path_str).await?;
    let session_file = session_file_path(team_id, project_id, &path);

    if let Err(msg) = download_from_ftp(ftp_session_path, session_file) {
        return Err(IdpGlobalError::NoteError(msg));
    }
    Ok(Rsp::success(abs_path.to_string()))
}
