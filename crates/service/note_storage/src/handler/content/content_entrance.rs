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

use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;

use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use business::path_tool;
use business::path_tool::get_nbconvert_by_team_id;
use common_model::entity::cell::Cell;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;
use suppaftp::FtpStream;

use super::cat::CatReq;
use super::cat::CatRsp;
use crate::api::http::v2::pipeline::ipynb2html;
use crate::api_model::content::*;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler;

pub async fn ipynb_preview(Query(cat_req): Query<CatReq>) -> Result<Rsp<CatRsp>, ErrorTrace> {
    tracing::debug!("access ipynb preview api.");
    let team_id = cat_req.team_id;
    let browser_path = cat_req.path;
    let project_id = cat_req.project_id;
    let full_path =
        business::path_tool::get_store_full_path(team_id, project_id, browser_path.clone());

    let prefixed_file_name = format!(
        "preview_{}",
        path_tool::escape_path_as_string(browser_path.clone())
    );
    let html_file_name =
        crate::business_::path_tool::name_convert(prefixed_file_name, "html".to_string());
    let dst = path_tool::get_full_tmp_path(team_id, project_id, html_file_name);

    tracing::debug!("run ipynb2html on : {:?}", full_path);
    let nbconvert_path = get_nbconvert_by_team_id(format!("{}", team_id));
    ipynb2html(
        nbconvert_path,
        full_path.to_str().unwrap().to_string(),
        dst.to_str().unwrap().to_string(),
    )
    .await
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertCellReq {
    pub path: String,
    /**
    - 0: insert between two cell（above_cell_index+under_cell_index) /2
    - 1: insert above first cell cell_index == above_cell_index / 2;
    - 2: insert under last cell under_cell_index cell_index == under_cell_index + 1;
    */
    pub insert_flag: usize,
    pub cell_type: common_model::entity::cell::CellType,
    pub above_cell_index: Option<f64>,
    pub under_cell_index: Option<f64>,
    pub project_id: u64,
}

/// insert a new cell.
#[axum_macros::debug_handler]
pub async fn insert_cell(
    State(mut app_context): State<AppContext>,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(insert_cell_req): Json<InsertCellReq>,
) -> Result<Rsp<Cell>, IdpGlobalError> {
    tracing::info!("access insert_cell api.");

    if !(insert_cell_req.path.ends_with(".ipynb") || insert_cell_req.path.ends_with(".idpnb")) {
        return Err(IdpGlobalError::ErrorCodeMsg(
            crate::status_code::INVALID_FILETYPE_ERROR_CODE,
            crate::status_code::INVALID_FILETYPE_ERROR_MSG.to_string(),
        ));
    }

    let team_id = get_cookie_value_by_team_id(cookies);

    handler::content::insert_cell(
        insert_cell_req.path,
        insert_cell_req.insert_flag,
        insert_cell_req.cell_type,
        insert_cell_req.above_cell_index,
        insert_cell_req.under_cell_index,
        team_id,
        insert_cell_req.project_id,
        &mut app_context.redis_cache,
    )
    .await
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCellReq {
    pub path: String,
    pub project_id: u64,
    pub cell: Cell,
}

/// add_cell using the specific cell structure
pub async fn add_cell(
    State(mut app_context): State<AppContext>,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(add_cell_req): Json<AddCellReq>,
) -> Result<Rsp<Cell>, IdpGlobalError> {
    tracing::info!("access add_cell api.cell_id:{:?}", add_cell_req.cell.id());

    if !(add_cell_req.path.ends_with(".ipynb") || add_cell_req.path.ends_with(".idpnb")) {
        return Err(IdpGlobalError::ErrorCodeMsg(
            crate::status_code::INVALID_FILETYPE_ERROR_CODE,
            crate::status_code::INVALID_FILETYPE_ERROR_MSG.to_string(),
        ));
    }

    let team_id = get_cookie_value_by_team_id(cookies);

    handler::content::add_cell(
        team_id,
        add_cell_req.project_id,
        add_cell_req.path,
        add_cell_req.cell,
        &mut app_context.redis_cache,
    )
    .await
}

pub async fn move_cell(
    State(mut app_context): State<AppContext>,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(move_cell_req): Json<MoveCellReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    tracing::info!(
        "access move_cell api. id:{},another_id:{}",
        move_cell_req.id,
        move_cell_req.neighbor_cell_id
    );

    let team_id = get_cookie_value_by_team_id(cookies);

    handler::content::move_cell(
        move_cell_req.path,
        team_id,
        move_cell_req.project_id,
        move_cell_req.neighbor_cell_id,
        move_cell_req.id,
        &mut app_context.redis_cache,
    )
    .await
}

pub async fn delete_cell(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(delete_cell_req): Query<DeleteCellReq>,
    State(mut app_context): State<AppContext>,
) -> Result<Rsp<()>, IdpGlobalError> {
    tracing::info!("access delete_cell cell_id:{}", delete_cell_req.id);

    let team_id = get_cookie_value_by_team_id(cookies);

    handler::content::delete_cell(
        delete_cell_req.path,
        team_id,
        delete_cell_req.project_id,
        delete_cell_req.id,
        &mut app_context.redis_cache,
    )
    .await
}

// https://github.com/mattnenterprise/rust-ftp
pub fn ftp_stream(
    host: String,
    port: u16,
    user: String,
    password: String,
) -> Result<FtpStream, String> {
    if let Ok(mut ftp_stream) = FtpStream::connect(format!("{}:{}", host, port)) {
        match ftp_stream.login(&user, &password) {
            Ok(_) => Ok(ftp_stream.active_mode()),
            Err(err) => {
                tracing::error!("{err}");
                Err(err.to_string())
            }
        }
    } else {
        Err("connent to ftp server failed".to_string())
    }
}

pub fn env_or_default(key: &str, default_val: &str) -> String {
    if let Ok(v) = env::var(key) {
        v
    } else {
        String::from(default_val)
    }
}

pub fn idp_ftp_stream() -> Result<FtpStream, String> {
    ftp_stream(
        env_or_default("FTP_HOST", "idp-ftp-service"),
        21,
        env_or_default("FTP_USER", "user"),
        env_or_default("FTP_PASSWORD", "pass1234"),
    )
}

pub fn download_from_ftp(ftp_path: String, local_path: String) -> Result<(), String> {
    ensure_file_dir_exist(local_path.clone());
    if let Ok(mut ftp) = idp_ftp_stream() {
        match File::create(&local_path) {
            Ok(f) => {
                let mut writer = std::io::BufWriter::new(f);
                tracing::debug!("writer {} created", local_path);
                if let Some((dir_str, ftp_filename)) = dir_and_filename(ftp_path.clone()) {
                    tracing::debug!("dir_str: {}, filename: {}", dir_str, ftp_filename);
                    tracing::debug!("cd ftp dir {}", dir_str);
                    if let Err(err) = ftp.cwd(&dir_str) {
                        return Err(format!("cd {} failed on ftp: {}", dir_str, err));
                    }

                    if let Ok(content_cursor) = ftp.retr_as_buffer(&ftp_filename) {
                        tracing::debug!("{} content are {:?}", ftp_filename, content_cursor);
                        writer
                            .write_all(content_cursor.into_inner().as_slice())
                            .map_err(|e| {
                                format!("write to local file {} failed: {}", local_path, e)
                            })
                            .unwrap();
                        writer.flush().unwrap();
                        drop(writer);
                        tracing::debug!("write to local file {} success", local_path);
                        Ok(())
                    } else {
                        tracing::debug!("retr {} failed", ftp_filename);
                        Err(format!("retr {} failed", ftp_filename))
                    }
                } else {
                    Err(format!(
                        "split path {} to dir and filename failed",
                        ftp_path
                    ))
                }
            }
            Err(msg) => Err(format!(
                "open local file {} for write failed: {}",
                local_path, msg
            )),
        }
    } else {
        Err("connect to idp ftp failed".to_string())
    }
}

pub fn dir_and_filename(file_path: String) -> Option<(String, String)> {
    file_path.rfind('/').map(|pos| {
        (
            file_path[..pos].to_string(),
            file_path[pos + 1..].to_string(),
        )
    })
}

pub fn ensure_file_dir_exist(file_path: String) {
    if let Some((dir_str, _filename)) = dir_and_filename(file_path.clone()) {
        if let Err(msg) = create_dir_all(dir_str.clone()) {
            tracing::error!("create dir {} failed: {}", dir_str, msg);
        } else {
            tracing::debug!("create dir {} success", dir_str);
        }
    } else {
        tracing::debug!("{}'s dir already exists", file_path);
    }
}

pub async fn load_shared(
    Query(req): Query<SharedCellReq>,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
) -> Result<Rsp<String>, IdpGlobalError> {
    let team_id = common_tools::cookies_tools::get_cookie_value_by_team_id(cookies);
    let share_id = req.share_id;
    let project_id = req.project_id;
    handler::content::load_cell(team_id, project_id, share_id).await
}

#[test]
fn test_mkdir_dirs() {
    let fak = "/tmp/aa/bb/cc/dd/ee";
    ensure_file_dir_exist(fak.to_string());
    assert!(std::path::Path::new("/tmp/aa/bb/cc/dd").exists());
}

#[test]
fn test_mkdir() {
    // TODO: mkdir dir not support nested dir
    if let Ok(mut ftp) = idp_ftp_stream() {
        match ftp.mkdir("/test3/t1") {
            Ok(_) => tracing::debug!("mkdir success"),
            Err(msg) => tracing::debug!("mkdir failed: {}", msg),
        }
    }
}

#[test]
fn test_download() {
    match download_from_ftp("/up/test.txt".to_string(), "/tmp/new.txt".to_string()) {
        Ok(_) => tracing::debug!("download success"),
        Err(msg) => tracing::debug!("download failed: {}", msg),
    }
}
