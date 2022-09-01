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

use axum::extract::Query;
use axum::Extension;
use business::path_tool::session_file_path;
use business::path_tool::{self};
use cache_io::CacheService;
use common_model::entity::notebook::Notebook;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;

use crate::app_context::AppContext;
use crate::handler::content::content_entrance::dir_and_filename;
use crate::handler::content::content_entrance::idp_ftp_stream;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareCellReq {
    pub path: String,
    pub team_id: u64,
    pub project_id: u64,
    pub cell_id: String,
}

pub async fn share_cell(
    Query(req): Query<ShareCellReq>,
    Extension(app_context): Extension<AppContext>,
) -> Result<Rsp<String>, ErrorTrace> {
    let team_id = req.team_id;
    let browser_path = req.path;
    let cell_id = req.cell_id;

    share_cell_(
        team_id,
        req.project_id,
        browser_path,
        cell_id,
        &app_context.redis_cache,
    )
    .await
}

pub async fn share_cell_(
    team_id: u64,
    project_id: u64,
    browser_path: String,
    cell_id: String,
    redis_cache: &CacheService,
) -> Result<Rsp<String>, ErrorTrace> {
    tracing::error!(
        "share_cell {} {} {} {}",
        team_id,
        project_id,
        browser_path,
        cell_id
    );
    let share_id = uuid::Uuid::new_v4().to_string();
    let notebook_full_path =
        path_tool::get_store_full_path(team_id, project_id, browser_path.clone());
    if let Ok(mut notebook) = redis_cache
        .read_notebook(&notebook_full_path, project_id)
        .await
    {
        if let Some(aim_cell) = notebook.get_cell_by_id(cell_id.clone()) {
            let dir = business::path_tool::store_parent_dir();
            let to_upload = format!("{}/store/tmp/share.ipynb", dir.to_str().unwrap());
            let mut tmp_notebook = Notebook::new(&to_upload);
            tmp_notebook.cells.clear();
            tmp_notebook.cells.push(aim_cell);
            if let Err(err) =
                common_tools::file_tool::write_notebook_to_disk(to_upload.clone(), &tmp_notebook)
                    .await
            {
                tracing::error!("{err}");
                return Err(ErrorTrace::new(&format!(
                    "write new notebook to disk failed {}",
                    browser_path
                )));
            }
            if let Err(msg) = upload_to_ftp(to_upload.clone(), format!("/{}/share.ipynb", share_id))
            {
                return Err(ErrorTrace::new(&msg));
            }
            // let inode = file_tool::get_inode_from_path(&notebook_full_path).await?;
            let session_file = session_file_path(team_id, project_id, &browser_path);
            if let Err(msg) = upload_to_ftp(session_file, format!("/{}/share.session", share_id)) {
                return Err(ErrorTrace::new(&msg));
            }
            Ok(Rsp::success(share_id))
        } else {
            Err(ErrorTrace::new(&format!("no such cell id {}", cell_id)))
        }
    } else {
        Err(ErrorTrace::new(&format!(
            "can not read notebook {} -> {:?}",
            browser_path, notebook_full_path
        )))
    }
}

pub fn upload_to_ftp(local_path: String, aim_path: String) -> Result<(), String> {
    if let Ok(mut ftp) = idp_ftp_stream() {
        if let Ok(f) = std::fs::File::open(local_path.clone()) {
            if let Some((dir_str, _filename)) = dir_and_filename(aim_path.clone()) {
                if let Err(err) = ftp.cwd(&dir_str) {
                    tracing::warn!("{err}");
                    if let Err(msg) = ftp.mkdir(&dir_str) {
                        return Err(format!("mkdir {} failed on ftp {}", dir_str, msg));
                    }
                    if let Err(msg) = ftp.cwd(&dir_str) {
                        return Err(format!("cd {} failed on ftp {}", dir_str, msg));
                    }
                }
                // change working directory success
                let mut reader = std::io::BufReader::new(f);
                if let Err(msg) = ftp.put_file(&aim_path, &mut reader) {
                    Err(format!(
                        "put failed: {} -> {}, {}",
                        local_path, aim_path, msg
                    ))
                } else {
                    Ok(())
                }
            } else {
                Err(format!("aim_path {} 's diris not a directory", aim_path))
            }
        } else {
            Err(format!("open local file {} for read failed", local_path))
        }
    } else {
        Err("connect to idp ftp server failed".to_string())
    }
}
