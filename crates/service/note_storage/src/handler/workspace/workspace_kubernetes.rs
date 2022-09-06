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

use std::collections::HashMap;
use std::fs;
// use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use business::business_term::ProjectId;
use business::business_term::TeamId;
use business::path_tool;
use business::path_tool::get_nbconvert_by_team_id;
use cache_io::CacheService;
use chrono::prelude::Utc;
use common_model::api_model::ActiveDataRet;
use common_model::enums::mime::Mimetype;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;
// use reqwest::Body;
use serde::Deserialize;
use serde::Serialize;
use str_utils::StartsWithIgnoreAsciiCase;
// use tokio::fs::File;
// use tokio::fs::File as TokioFile;
// use tokio::io::AsyncReadExt;
// use tokio::io::AsyncSeekExt;
// use tokio_util::codec::BytesCodec;
// use tokio_util::codec::FramedRead;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::api::http::v2::pipeline::export_as_helper;
use crate::api_model::workspace::DataSourceObj;
use crate::api_model::workspace::DataSourceRet;
use crate::api_model::workspace::FileTreeNode;
use crate::api_model::workspace::FullFileTreeNode;
use crate::api_model::workspace::GlobalSearchResult;
use crate::api_model::workspace::IpynbFileJson;
use crate::api_model::workspace::PathBufPojo;
// use crate::api_model::workspace::PathPojo;
use crate::business_::path_tool::name_convert;
use crate::common::error::IdpGlobalError;
use crate::handler;
use crate::handler::content::cat::file_mime_magic::find_mimetype;
// use crate::handler::project_handler::parse_return_success_code;
use crate::status_code::{
    NB_RENAME_ERROR_CODE, NB_RENAME_ERROR_MSG, NB_STORE_ERROR_CODE, NB_STORE_ERROR_MSG,
};

// static full_datasource_url: String = "http://idp-commandservice-svc:8083/api/command/datasource/list".to_string();
// static active_datasource_url: String = "http://127.0.0.1:8083/api/command/database/active".to_string();
static FULL_DATASOURCE_URL: &str = "http://idp-commandservice-svc:8083/api/command/datasource/list";
static ACTIVE_DATASOURCE_URL: &str = "http://127.0.0.1:8083/api/command/database/active";

const CHUNK_SIZE: u64 = 1024 * 1024 * 2; // 2MB

#[instrument(skip(_cache_service))]
pub async fn file_dir_move(
    origin_path: String,
    target_path: String,
    team_id: TeamId,
    project_id: ProjectId,
    auto_close: Option<bool>,
    _cache_service: &mut CacheService,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("move file /dir..");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    let mut from_path = base_path.clone();
    let mut to_path = base_path.clone();
    if !origin_path.eq("/") {
        from_path.push(crate::business_::path_tool::get_relative_path(&origin_path));
    }
    to_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &target_path,
    )));

    let from_path_str = from_path.to_str().unwrap();
    let to_path_str = to_path.to_str().unwrap();

    if to_path.exists() {
        return Err(IdpGlobalError::NoteError("to_path exist".to_string()));
    }

    info!(
        "from_path: {:?},to_path_str: {:?}",
        from_path_str, to_path_str
    );
    //check_whether_file_can_change
    handler::kernel::shutdown_by_dir_path(project_id, origin_path).await?;

    tokio::fs::rename(from_path_str, to_path_str).await?;

    Ok(Rsp::success_without_data())
}

#[cfg(not)]
pub async fn check_whether_file_or_dir_can_change<P: AsRef<Path>>(
    team_id: TeamId,
    project_id: ProjectId,
    file_path: P,
    auto_close: Option<bool>,
    redis_cache: &mut CacheService,
) -> Result<(), IdpGlobalError> {
    debug!("check_whether_file_or_dir_can_change..");
    let file_path = file_path.as_ref();
    // if file_path is file and extension is ".ipynb"
    if file_path.exists() {
        debug!("file exists.");
        if file_path.is_file() {
            let file_extension = file_path.extension().unwrap().to_str().unwrap();
            // get inode from "file_path"
            let inode = file_tool::get_inode_from_path(file_path).await?;
            if file_extension == "ipynb" {
                // get kernel_state from cache_service using project_id and inode
                let kernel_state = redis_cache.get_kernel_state(project_id, inode).await;

                match kernel_state {
                    Ok(kernel_state_opt) => {
                        if let Some(kernel_state) = kernel_state_opt {
                            if auto_close.is_some() && auto_close.unwrap() {
                                // call inner api to close this kernel by inode
                                debug!("exists running kernel,auto close it");
                                let kernel_state =
                                    serde_json::from_str::<KernelState>(&kernel_state).unwrap();
                                handler::kernel::close_kernel(
                                    project_id,
                                    inode,
                                    kernel_state.hostname,
                                )
                                .await?;
                            } else {
                                // if kernel_state is not None, then refuse to move file,return IllegalOperationError.
                                return Err(err::ErrorTrace::new("The corresponding kernel to the file is running, please stop the kernel first").into());
                            }
                        }
                    }
                    Err(err) => {
                        return Err(IdpGlobalError::NoteError(err.to_string()));
                    }
                }
            }
            redis_cache.del_file_cache_key(inode).await?;
        } else if file_path.is_dir() {
            debug!("file is dir.");
            //check this dir whether have ipynb file that is running.
            //get all kernel state object.
            let kernel_state_list_result = redis_cache.get_kernel_state_list(project_id).await;
            match kernel_state_list_result {
                Ok(kernel_state_list) => {
                    println!("{:?}", kernel_state_list);

                    for kernel_state in kernel_state_list {
                        let kernel_state_path = kernel_state.notebook_path;
                        if let Some(dir_path) =
                            crate::business_::path_tool::get_relative_path_from_notebooks_full_path(
                                team_id, project_id, file_path,
                            )
                        {
                            println!(
                                "kernel_state_path: {},dir_path:{}",
                                kernel_state_path, dir_path
                            );
                            if kernel_state_path.starts_with(&dir_path) {
                                if auto_close.is_some() && auto_close.unwrap() {
                                    handler::kernel::close_kernels(
                                        redis_cache,
                                        project_id,
                                        Some(dir_path),
                                    )
                                    .await?;
                                } else {
                                    return Err(err::ErrorTrace::new("The corresponding kernel to the file is running, please stop the kernel first").into());
                                }
                            }
                        } else {
                            return Err(IdpGlobalError::NoteError(
                                "get_relative_path occur error.".to_string(),
                            ));
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Get kernel state list error when check whether dir can change : {:?}",
                        err
                    );
                    return Err(IdpGlobalError::NoteError(err.to_string()));
                }
            }
        }
    } else {
        return Err(IdpGlobalError::NoteError(
            "File not found,perhaps the member of your team has deleted or moved it. ".to_string(),
        ));
    }
    Ok(())
}

#[instrument]
pub async fn file_dir_copy(
    origin_path: String,
    target_path: String,
    team_id: u64,
    project_id: u64,
) -> Result<Rsp<()>, ErrorTrace> {
    info!("copy file /dir..");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    let mut from_path = base_path.clone();
    let mut to_path = base_path;
    from_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &origin_path,
    )));
    to_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &target_path,
    )));

    let from_path_str = from_path.to_str().unwrap();
    let mut to_path_str = to_path.to_str().unwrap();

    if to_path.exists() {
        if to_path.is_dir() {
            let mut to_path_string = to_path_str.to_string();
            let s1 = "_".to_string();
            let s2 = get_unix_timestamp_ms().to_string();
            to_path_string += &s1;
            to_path_string += &s2;
            to_path_str = to_path_string.as_str();
            info!(
                "from_path: {:?},to_path_str: {:?}",
                from_path_str, &to_path_str
            );

            common_tools::command_tools::copy(from_path_str, to_path_str)?;
        } else {
            let to_path_string = to_path_str.to_string();
            let pos_ext = to_path_string.rfind('.');
            if pos_ext == None {
                let mut to_path_string = to_path_str.to_string();
                let s1 = "_".to_string();
                let s2 = get_unix_timestamp_ms().to_string();
                to_path_string += &s1;
                to_path_string += &s2;

                to_path_str = to_path_string.as_str();
                info!(
                    "from_path: {:?},to_path_str: {:?}",
                    from_path_str, &to_path_str
                );

                common_tools::command_tools::copy(from_path_str, to_path_str)?;
            } else {
                let to_path_string = to_path_str.to_string();
                let _pos_ext = pos_ext.unwrap();
                let (filename_pre, filename_ext) = to_path_string.split_at(_pos_ext);
                let ext = filename_ext.to_string();
                let mut filename_pre_string = filename_pre.to_string();

                let s1 = "_".to_string();
                let s2 = get_unix_timestamp_ms().to_string();
                filename_pre_string += &s1;
                filename_pre_string += &s2;
                filename_pre_string += &ext;

                to_path_str = filename_pre_string.as_str();

                info!(
                    "from_path: {:?},to_path_str: {:?}",
                    from_path_str, &to_path_str
                );
                common_tools::command_tools::copy(from_path_str, to_path_str)?;
            }
        }
    } else {
        info!(
            "from_path: {:?},to_path_str: {:?}",
            from_path_str, &to_path_str
        );
        common_tools::command_tools::copy(from_path_str, to_path_str)?;
    }

    Ok(Rsp::success_without_data())
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ModelId {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UploadFileDataRetData {
    pub code: u32,
    pub message: String,
    pub data: ModelId,
}

#[cfg(not)]
pub async fn model_upload(
    path: String,
    team_id: u64,
    user_id: u64,
    project_id: u64,
    model_name: String,
    version: String,
    intro: String,
) -> Result<Rsp<()>, IdpGlobalError> {
    tracing::debug!("access workspace_handler model_upload api");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    let mut from_path = base_path.clone();
    from_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));
    let source = from_path.into_os_string().into_string().unwrap();
    tracing::debug!("source: {}", &source);

    let url = "http://idp-admin-rs-svc:9092/api/v1/admin-rs/model-manage/upload".to_string();

    let http_client = reqwest::Client::new();
    let f = TokioFile::open(&source).await.unwrap();
    let fsize = f.metadata().await.unwrap().len();
    tracing::debug!("fsize: {}", fsize);

    let mut chunk_size = CHUNK_SIZE;

    tracing::debug!("file size: {}, chunk size: {}", fsize, chunk_size);

    let mut seek: u64 = 0;
    let mut parts: Vec<(u64, u64)> = Vec::new();
    let mut total = 0u32;
    while seek < fsize {
        if (fsize - seek) <= chunk_size {
            chunk_size = fsize % chunk_size;
        }
        parts.push((seek, chunk_size));
        seek += chunk_size;
        total += 1;
        tracing::debug!("file split total: {}", total);
    }

    tracing::debug!("----------------------------\n");

    let mut global_success_flag = true;

    let mut index = 0u32;
    for (_pos, part) in parts.iter().enumerate() {
        index += 1;

        // let http_client = reqwest::Client::new();

        let seek = part.0;
        let chunk_real_data_size = part.1;

        let mut file = File::open(&source).await?;
        file.seek(SeekFrom::Start(seek)).await?;
        let file = file.take(chunk_real_data_size);

        let stream =
            FramedRead::with_capacity(file, BytesCodec::new(), chunk_real_data_size as usize);

        tracing::debug!("##stream: {:?}", stream);
        tracing::debug!("##seek: {}, part.0:{}, part.1: {}", seek, part.0, part.1);

        let body = Body::wrap_stream(stream);
        let mut file_name = source.clone();
        if let Some(pos) = file_name.rfind('/') {
            file_name = file_name.clone()[pos + 1..].parse().unwrap();
        }

        tracing::debug!("##index:{},file_name: {}", index, &file_name);

        //chunk_real_data_size must be uniform in code
        let part = reqwest::multipart::Part::stream_with_length(body, chunk_real_data_size)
            .file_name(file_name.clone());
        let form = reqwest::multipart::Form::new()
            .text("name", file_name)
            .text("teamId", team_id.to_string())
            .text("userId", user_id.to_string())
            .text("projectId", project_id.to_string())
            .text("modelName", model_name.clone())
            .text("version", version.clone())
            .text("intro", intro.clone())
            .text("index", index.to_string())
            .text("total", total.to_string())
            .text("fileFrom", "copy".to_string())
            .part("datafile", part);

        tracing::debug!("form{:#?}", form);
        tracing::debug!(
            "index.to_string(): {:#?}, total.to_string(): {:#?} ",
            index.to_string(),
            total.to_string()
        );

        // tracing::debug!("form: {:#?}", form);

        let response = http_client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;

        // let response = http_client.post(&url).multipart(form).send().await.unwrap();
        tracing::debug!("model_upload upload job file: {:?}", response);

        let slice_ret: UploadFileDataRetData = serde_json::from_str(response.as_str()).unwrap();
        if !parse_return_success_code(slice_ret.code) {
            global_success_flag = false;
        }
    }

    if global_success_flag {
        Ok(Rsp::success_without_data())
    } else {
        Ok(Rsp::error_code_msg(
            UPLOAD_MODEL_ERROR_CODE,
            UPLOAD_MODEL_ERROR_MSG,
        ))
    }
}

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

#[cfg(not)]
#[instrument]
pub async fn dir_export(
    export_path: String,
    team_id: u64,
    project_id: u64,
    project_name_opt: Option<String>,
) -> impl IntoResponse {
    info!("access dir_export_test function .......");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let tmp_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::TMP,
    );
    info!("tmp_path: {:?}", tmp_path);

    let mut abs_export_path = base_path.clone();
    abs_export_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &export_path,
    )));
    info!("export_path: {:?}", abs_export_path);

    let mut generate_filename;
    let download_file_name;
    if export_path.eq_ignore_ascii_case("/") {
        generate_filename = tmp_path.display().to_string() + "/notebook.zip";
        info!("root export, generate_filename: {:?}", generate_filename);
        let project_path = path_tool::project_root(team_id, project_id);
        info!("root export, project_path: {:?}", project_path);

        if let Some(project_name) = project_name_opt {
            download_file_name = project_name;
        } else {
            download_file_name = "notebook".to_string();
        }
        info!("root export, download_file_name:{}", download_file_name);

        let export_path_start = "notebooks".to_string();
        let exclude_export_path = "notebooks/storage-service/*".to_string();
        info!("root export, exclude_export_path:{}", exclude_export_path);

        //must be "$exclude_export_path" it works well. $exclude_export_path does not work
        let mut cmd = std::process::Command::new("zip");
        cmd.current_dir(project_path)
            .arg("-q")
            .arg("-r")
            .arg(&generate_filename)
            .arg(export_path_start)
            .arg("-x")
            .arg(exclude_export_path);
        tracing::info!("cmd = {cmd:?}");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        }
    } else {
        let is_dir = abs_export_path.is_dir();
        if !is_dir {
            return Err(ErrorTrace::new("dir not exist"));
        } else {
            let is_empty = abs_export_path.read_dir()?.next().is_none();
            if is_empty {
                return Err(ErrorTrace::new("dir is empty"));
            }
        }

        let pos = export_path.rfind('/').unwrap();
        let (_, file_name) = export_path.split_at(pos + 1);
        info!("file_name:{}", file_name);
        download_file_name = file_name.to_string();
        info!("download_file_name:{}", download_file_name);

        let (_, export_path_start) = export_path.split_at(1);
        info!("export_path_start:{}", export_path_start);

        generate_filename = tmp_path.display().to_string() + "/";
        let s2 = file_name.to_string();
        let ext = ".zip";
        generate_filename += &s2;
        generate_filename += ext;

        info!("generate_filename:{}", generate_filename);
        info!("file_name:{}", file_name);

        let org_export_path = abs_export_path.parent().unwrap();

        // (cd $org_export_path;zip -q -r $generate_filename $file_name) {
        let mut cmd = std::process::Command::new("zip");
        cmd.current_dir(org_export_path)
            .arg("-q")
            .arg("-r")
            .arg(&generate_filename)
            .arg(file_name);
        tracing::info!("cmd = {cmd:?}");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        }
    }
    let file = File::open(generate_filename).await?;

    let stream = ReaderStream::new(file);

    let att1 = String::from("attachment; filename=\"");
    let att2 = download_file_name;
    let att3 = String::from(".zip");
    let att4 = String::from("\"");
    let attachment_str = format!("{}{}{}{}", att1, att2, att3, att4);

    let body = StreamBody::new(stream);
    let headers = Headers(vec![
        (header::CONTENT_TYPE, "application/zip"),
        (
            header::CONTENT_DISPOSITION,
            string_to_static_str(attachment_str),
        ),
    ]);

    Ok((headers, body))
}

#[instrument]
pub async fn dir_zip(
    path: String,
    team_id: String,
    project_id: u64,
) -> Result<Rsp<()>, ErrorTrace> {
    info!("access workpasce_handler dir_zip function .......");
    let team_id = team_id.parse::<u64>()?;
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut abs_export_path = base_path.clone();
    abs_export_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));
    info!("export_path: {:?}", abs_export_path);

    let mut generate_filename;
    // let mut download_file_name ="".to_string();

    let pos = path.rfind('/').unwrap();
    let mut name: &str;
    (_, name) = path.split_at(pos + 1);
    info!("file_name:{}", name);
    let file_name = name;

    let is_dir = abs_export_path.is_dir();
    if !is_dir {
        let pos = name.rfind('.').unwrap();
        (name, _) = name.split_at(pos);
    } else {
        let is_empty = abs_export_path.read_dir()?.next().is_none();
        if is_empty {
            return Err(ErrorTrace::new("dir is empty"));
        }
    }

    let temp_name = name;
    // download_file_name = file_name.to_string();
    // info!("download_file_name:{}", download_file_name);

    let (_, export_path_start) = path.split_at(1);
    info!("export_path_start:{}", export_path_start);

    let abs_export_path_str = abs_export_path
        .to_str()
        .ok_or_else(|| IdpGlobalError::NoteError("cast exception!".to_string()));
    info!("abs_export_path_str: {:?}", abs_export_path_str);

    let parent_path = &abs_export_path.parent().unwrap();
    info!("parent_path:{:?}", parent_path);

    generate_filename = temp_name.to_string();
    let ext = ".zip";
    generate_filename += ext;

    info!("generate_filename:{}", generate_filename);
    info!("file_name:{}", file_name);

    let path = &parent_path.join(&generate_filename);
    info!("path:{:?}", path);
    if path.exists() {
        return Err(ErrorTrace::new("file already exists."));
    }

    info!("parent_path-->{:#?}", parent_path);
    info!("generate_filename-->{:#?}", generate_filename);
    info!("file_name-->{:#?}", file_name);

    info!("!!!!base_path: {:?}", base_path);
    // (cd $parent_path;zip -q -r $generate_filename $file_name)
    let mut cmd = std::process::Command::new("zip");
    cmd.current_dir(parent_path)
        .arg("-q")
        .arg("-r")
        .arg(&generate_filename)
        .arg(file_name);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }

    Ok(Rsp::success_without_data())
}

#[cfg(not)]
pub async fn dir_lazy_load(
    team_id: u64,
    project_id: u64,
    _region: String,
    path_array: Vec<String>,
    only_pipeline_support: bool,
) -> Result<Rsp<FileTreeNode>, IdpGlobalError> {
    info!("access dir_lazy_load api");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut file_dir_map = HashMap::new();
    let pipeline_ext = ["ipynb", "idpnb", "py", "sh"];

    let path_start: String;
    let show_one_layer: bool;
    if path_array.clone().len() == 1 {
        path_start = path_array.get(0).unwrap().to_string();
        show_one_layer = true;
    } else {
        path_start = "/".to_string();
        show_one_layer = false;
    }

    let mut data_source_hashmap: HashMap<String, DataSourceObj> = HashMap::new();
    let mut has_storage_service = false;
    for path in path_array.clone().iter() {
        if path.contains("/storage-service") {
            has_storage_service = true;
        }
    }
    tracing::debug!("has_storage_service:----- {}", has_storage_service);
    if has_storage_service {
        // call api/v1/command/datasource/list
        if let Ok(ret) = get_full_datasource_data(project_id.to_string(), team_id).await {
            let full_array = ret.data.record;
            for data_source_obj in full_array.iter() {
                data_source_hashmap
                    .insert(data_source_obj.alias.to_string(), data_source_obj.clone());
            }
        }

        let mut data_source_hashmap_develop: HashMap<String, DataSourceObj> = HashMap::new();
        // call api/v1/command/database/active api
        if let Ok(ret) = get_develop_active_connect_data(project_id.to_string()).await {
            let active_array = ret.data.clone();
            tracing::debug!("##get_develop_active_connect_data ret.data={:?}", ret.data);
            // let mut active_hashmap: HashMap<String, ActiveDataObj> = HashMap::new();
            for active_data_obj in active_array.iter() {
                let data_source_obj = DataSourceObj::new(
                    active_data_obj.alias.clone(),
                    active_data_obj.type_.clone(),
                    active_data_obj.data_source_type.clone(),
                    "".to_string(),
                    "".to_string(),
                    true,
                );
                data_source_hashmap_develop.insert(data_source_obj.alias.clone(), data_source_obj);
            }
            for (key, value) in data_source_hashmap_develop.clone() {
                tracing::debug!(
                    "##get_develop_active_connect_data - {} / {} ",
                    key,
                    value.active,
                );
            }
        }
        for (key, value) in data_source_hashmap_develop.clone() {
            data_source_hashmap.insert(key, value);
        }
    }

    for path in path_array.iter() {
        let mut abs_list_path = base_path.clone();
        abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
            &path,
        )));
        info!("abs_list_path: {:?}", abs_list_path);

        let mut sub_paths = Vec::new();
        let mut path_pojos = Vec::new();

        if abs_list_path.is_dir() {
            for entry in fs::read_dir(abs_list_path.clone())
                .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()))?
            {
                let entry =
                    entry.map_err(|_| IdpGlobalError::NoteError("get error".to_string()))?;
                let path = entry.path();
                let short_path = path
                    .display()
                    .to_string()
                    .replace(&base_path.clone().display().to_string(), "");

                let path_clone = path.clone();
                let fullpath = path_clone.display().to_string();
                let pos = fullpath.rfind('/').unwrap();
                let (_, filename) = fullpath.split_at(pos + 1);
                // println!("id:{}",filename);

                if !filename.starts_with_ignore_ascii_case(".") {
                    // println!("in recursiveChild    short_path: {:?}", short_path);
                    // println!("in recursiveChild          path: {}", path.display().to_string());
                    let mut sort_path;
                    if path.is_dir() {
                        let cloud_id = "storage-service";
                        let _slice: Vec<&str> = short_path.split('/').collect();
                        let first_path = _slice[1];
                        if first_path.eq_ignore_ascii_case(cloud_id) {
                            sort_path = "a-".to_string();
                        } else {
                            sort_path = "d-".to_string();
                        }
                    } else {
                        sort_path = "f-".to_string();
                    }
                    sort_path += &short_path;

                    let pos_ext = filename.rfind('.');
                    if pos_ext == None {
                        // sub_paths.push(short_path.to_string());
                        let _path_pojo =
                            PathPojo::new(short_path.to_string(), sort_path.to_string());
                        path_pojos.push(_path_pojo);
                    } else {
                        let _pos_ext = pos_ext.unwrap();
                        let (_, filename_ext) = filename.split_at(_pos_ext + 1);
                        let ext = filename_ext.to_ascii_lowercase();
                        if only_pipeline_support {
                            if pipeline_ext.contains(&ext.as_str()) {
                                // sub_paths.push(short_path.to_string());
                                let _path_pojo =
                                    PathPojo::new(short_path.to_string(), sort_path.to_string());
                                path_pojos.push(_path_pojo);
                            }
                        } else {
                            // sub_paths.push(short_path.to_string());
                            let _path_pojo =
                                PathPojo::new(short_path.to_string(), sort_path.to_string());
                            path_pojos.push(_path_pojo);
                        }
                    }
                }
            }
        }

        path_pojos.sort_unstable_by(|a, b| a.sort_path.cmp(&b.sort_path));
        for x in &path_pojos {
            sub_paths.push(x.clone().path);
        }
        file_dir_map.insert(path.to_string(), sub_paths);
    }

    // println!("file_dir_map = {:#?}", file_dir_map);

    let mut tree_node = FileTreeNode {
        absolute_path: base_path.display().to_string(),
        browser_path: "".to_string(),
        project_id: project_id.to_string(),
        file_name: String::from("notebooks"), //"notebooks",
        file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
        source_type: "".to_string(),          // xxxxx
        bucket: "".to_string(),
        end_point: "".to_string(),
        active: true,
        has_children: true,
        children: vec![],
        contains_keywords: false,
    };

    // let key_path = "/".to_string();
    recursive_child(
        &mut tree_node,
        path_start,
        base_path,
        file_dir_map,
        team_id,
        project_id,
        show_one_layer,
        data_source_hashmap,
    );
    Ok(Rsp::success(tree_node))
}

#[allow(clippy::single_match)]
pub fn recursive_child(
    node: &mut FileTreeNode,
    key_path: String,
    base_path: PathBuf,
    map: HashMap<String, Vec<String>>,
    team_id: u64,
    project_id: u64,
    show_one_layer: bool,
    data_source_hashmap: HashMap<String, DataSourceObj>,
) {
    let map_live = map.clone();
    let k = key_path;
    let v = map_live.get(&k);

    match v {
        Some(value) => {
            for sub_key_path in value.clone().iter() {
                // info!("sub_key: {}", sub_key_path);

                let mut abs_list_path = base_path.clone();
                abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
                    &sub_key_path,
                )));
                // info!("abs_list_path: {:?}", abs_list_path);

                let fullpath = abs_list_path.clone().display().to_string();
                let pos = fullpath.rfind('/').unwrap();
                let (_, filename) = fullpath.split_at(pos + 1);

                let mut child_node = FileTreeNode {
                    absolute_path: abs_list_path.display().to_string(),
                    browser_path: sub_key_path.to_string(),
                    project_id: project_id.to_string(),
                    file_name: filename.to_string(),
                    file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
                    source_type: "".to_string(),
                    bucket: "".to_string(),
                    end_point: "".to_string(),
                    active: true,
                    has_children: false,
                    children: vec![],
                    contains_keywords: false,
                };

                if abs_list_path.is_dir() {
                    let cloud_id = "storage-service";
                    let _slice: Vec<&str> = sub_key_path.split('/').collect();
                    let first_path = _slice[1];

                    if first_path.eq_ignore_ascii_case(cloud_id) {
                        if data_source_hashmap.contains_key(&filename.to_string()) {
                            let value = data_source_hashmap
                                .get(&filename.to_string())
                                .unwrap()
                                .clone();
                            child_node.source_type = value.datasource;
                            if value.active {
                                child_node.active = true;
                            } else {
                                child_node.active = false;
                            }
                            child_node.bucket = value.dbname; //bucket
                            child_node.end_point = value.path; //end_point
                        } else {
                            child_node.active = true; //storage-service self
                        }
                    } else {
                        child_node.active = true;
                    }
                }

                if abs_list_path.is_file() {
                    child_node.file_type = String::from("FILE");
                } else {
                    child_node.file_type = String::from("DIRECTORY");
                    child_node.has_children = true;
                    if !show_one_layer {
                        recursive_child(
                            &mut child_node,
                            sub_key_path.to_string(),
                            base_path.clone(),
                            map.clone(),
                            team_id,
                            project_id,
                            show_one_layer,
                            data_source_hashmap.clone(),
                        );
                    }
                }
                node.children.push(child_node.clone());
            }
        }
        _ => {}
    }
}

async fn get_full_datasource_data(
    project: String,
    team_id: u64,
) -> Result<DataSourceRet, reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post(FULL_DATASOURCE_URL)
        .json(&serde_json::json!({
           "project": project,
            "teamId": team_id
        }))
        .send()
        .await?
        .json()
        .await?;

    // println!("get_full_datasource_data={:#?}", echo_json);

    if let Ok(ret) = serde_json::from_value(echo_json) {
        Ok(ret)
    } else {
        println!("BBQ .........................");
        let null_ret:DataSourceRet = serde_json::from_value("{\"code\":200,\"message\":\"SUCCESS\",\"data\":[{\"path\":\"\",\"dbname\":\"\",\"datasource\":\"\",\"alias\":\"\",\"type\":\"\"}]}".parse().unwrap()).unwrap();
        Ok(null_ret)
    }
}

async fn get_develop_active_connect_data(project: String) -> Result<ActiveDataRet, reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post(ACTIVE_DATASOURCE_URL)
        .json(&serde_json::json!({ "project": project }))
        .send()
        .await?
        .json()
        .await?;
    tracing::debug!(
        "##get_develop_active_connect_data ACTIVE_DATASOURCE_URL={}",
        ACTIVE_DATASOURCE_URL
    );

    let ret: ActiveDataRet = serde_json::from_value(echo_json).unwrap();
    Ok(ret)
}

pub async fn dir_search(
    team_id: u64,
    project_id: u64,
    keyword: String,
    only_pipeline_support: bool,
) -> Result<Rsp<FileTreeNode>, IdpGlobalError> {
    info!("access dir_search function .......");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut tree_node = FileTreeNode {
        absolute_path: base_path.display().to_string(),
        browser_path: "".to_string(),
        project_id: project_id.to_string(),
        file_name: String::from("notebooks"), //"notebooks",
        file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
        source_type: "".to_string(),
        bucket: "".to_string(),
        end_point: "".to_string(),
        active: true,
        has_children: true,
        children: vec![],
        contains_keywords: false,
    };

    let path = "/".to_string();
    let _r = recursive_search_child(
        &mut tree_node,
        path,
        base_path,
        keyword,
        team_id,
        project_id,
        only_pipeline_support,
    );
    Ok(Rsp::success(tree_node))

    // let json_str = serde_json::to_string_pretty(&tree_node);
    // println!("result = {}", json_str.unwrap());
}

pub fn recursive_search_child(
    node: &mut FileTreeNode,
    path: String,
    base_path: PathBuf,
    keywords: String,
    team_id: u64,
    project_id: u64,
    only_pipeline_support: bool,
) -> Result<Rsp<String>, IdpGlobalError> {
    let mut abs_list_path = base_path.clone();
    abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));
    info!("recursive_search_child abs_list_path: {:?}", abs_list_path);

    let mut pathbuf_vec = Vec::new();

    let pipeline_ext = ["ipynb", "idpnb", "py", "sh"];

    if abs_list_path.is_dir() {
        for entry in fs::read_dir(abs_list_path.clone())
            .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()))?
        {
            let entry = entry.map_err(|_| IdpGlobalError::NoteError("get error".to_string()))?;
            let path = entry.path();

            let short_path = path
                .display()
                .to_string()
                .replace(&base_path.clone().display().to_string(), "");

            let fullpath = path.clone().display().to_string();
            let pos = fullpath.rfind('/').unwrap();
            let (_, filename) = fullpath.split_at(pos + 1);

            let mut sort_path;
            if path.is_dir() {
                sort_path = "d-".to_string();
            } else {
                sort_path = "f-".to_string();
            }
            sort_path += &short_path;

            if !filename.starts_with_ignore_ascii_case(".") {
                // info!("in recursiveChild    short_path: {:?}", short_path);
                // info!("in recursiveChild          path: {}", path.display().to_string());

                // let _pathbuf_pojo = PathBufPojo::new(path, short_path, filename.to_string(),
                // sort_path.to_string()); pathbuf_vec.push(_pathbuf_pojo);

                let pos_ext = filename.rfind('.');
                if pos_ext == None {
                    let _pathbuf_pojo = PathBufPojo::new(
                        path,
                        short_path,
                        filename.to_string(),
                        sort_path.to_string(),
                    );
                    pathbuf_vec.push(_pathbuf_pojo);
                } else {
                    let _pos_ext = pos_ext.unwrap();
                    let (_, filename_ext) = filename.split_at(_pos_ext + 1);
                    let ext = filename_ext.to_ascii_lowercase();
                    if only_pipeline_support {
                        if pipeline_ext.contains(&ext.as_str()) {
                            let _pathbuf_pojo = PathBufPojo::new(
                                path,
                                short_path,
                                filename.to_string(),
                                sort_path.to_string(),
                            );
                            pathbuf_vec.push(_pathbuf_pojo);
                        }
                    } else {
                        let _pathbuf_pojo = PathBufPojo::new(
                            path,
                            short_path,
                            filename.to_string(),
                            sort_path.to_string(),
                        );
                        pathbuf_vec.push(_pathbuf_pojo);
                    }
                }
            }
        }

        pathbuf_vec.sort_unstable_by(|a, b| a.sort_path.cmp(&b.sort_path));
        // info!("pathbuf_vec = {:#?}", pathbuf_vec);

        for x in &pathbuf_vec {
            // println!("x = {:#?}", x);
            // "json": {
            // "a": 123,
            //
            // }
            let mut child_node = FileTreeNode {
                absolute_path: x.clone().path.display().to_string(),
                browser_path: x.clone().short_path,
                project_id: project_id.to_string(),
                file_name: x.clone().filename,
                file_type: "".to_string(),
                source_type: "".to_string(),
                bucket: "".to_string(),
                end_point: "".to_string(),
                active: true,
                has_children: false,
                children: vec![],
                contains_keywords: false,
            };
            if abs_list_path.is_file() {
                child_node.file_type = String::from("FILE");
            } else {
                child_node.file_type = String::from("DIRECTORY");
                child_node.has_children = true;
                let _r = recursive_search_child(
                    &mut child_node,
                    x.clone().short_path,
                    base_path.clone(),
                    keywords.clone(),
                    team_id,
                    project_id,
                    only_pipeline_support,
                )
                .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()));
            }

            if x.clone().filename.to_string().contains(&keywords.clone()) {
                child_node.contains_keywords = true;
                node.children.push(child_node.clone());
                node.contains_keywords = true; //very important
            }

            if child_node.contains_keywords {
                let exist_this_child_node = node.children.contains(&child_node);
                if !exist_this_child_node {
                    node.children.push(child_node.clone());
                    node.contains_keywords = true; //very important
                }
            }

            // node.children.push( child_node.clone()); //if list the all files and folders
        }
    }

    Ok(Rsp::success("mydata".to_string()))
    //
}

pub async fn global_keyword_search(
    team_id: u64,
    project_id: u64,
    keyword: String,
) -> Result<Rsp<Vec<GlobalSearchResult>>, IdpGlobalError> {
    info!("access global_keyword_search function .......");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut global_search_result_vec = Vec::new();

    let path = "/".to_string();
    let _r = recursive_global_keyword_search(
        &mut global_search_result_vec,
        path,
        base_path,
        keyword,
        team_id,
        project_id,
    );

    Ok(Rsp::success(global_search_result_vec))

    // let json_str = serde_json::to_string_pretty(&tree_node);
    // println!("result = {}", json_str.unwrap());
}

pub fn recursive_global_keyword_search(
    vec: &mut Vec<GlobalSearchResult>,
    path: String,
    base_path: PathBuf,
    keyword: String,
    team_id: u64,
    project_id: u64,
) -> Result<Rsp<String>, IdpGlobalError> {
    let mut abs_list_path = base_path.clone();
    abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));
    info!(
        "recursive_global_keyword_search abs_list_path: {:?}",
        abs_list_path
    );

    if abs_list_path.is_dir() {
        for entry in fs::read_dir(abs_list_path.clone())
            .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()))?
        {
            let entry = entry.map_err(|_| IdpGlobalError::NoteError("get error".to_string()))?;
            let path = entry.path();

            let short_path = path
                .display()
                .to_string()
                .replace(&base_path.clone().display().to_string(), "");

            let fullpath = path.clone().display().to_string();
            let pos = fullpath.rfind('/').unwrap();
            let (_, filename) = fullpath.split_at(pos + 1);

            if !filename.starts_with_ignore_ascii_case(".") {
                // info!("in recursiveChild    short_path: {:?}", short_path);
                // info!("in recursiveChild          path: {}", path.display().to_string());
                if path.is_file() {
                    let pos_ext = filename.rfind('.');
                    if pos_ext == None {
                        // info!("in recursiveChild  no ext.");
                        put_result_to_vec(
                            vec,
                            path,
                            keyword.clone(),
                            short_path.clone(),
                            team_id,
                            project_id,
                            filename.to_string(),
                        );
                    } else {
                        let _pos_ext = pos_ext.unwrap();
                        let (_, filename_ext) = filename.split_at(_pos_ext + 1);
                        let ext = filename_ext.to_ascii_lowercase();

                        if "ipynb".eq_ignore_ascii_case(ext.as_str()) {
                            // ipynb search
                            if let Err(_error) = put_ipynb_result_to_vec(
                                vec,
                                path,
                                keyword.clone(),
                                short_path.clone(),
                                team_id,
                                project_id,
                                filename.to_string(),
                            ) {
                                continue;
                            }
                        } else {
                            // info!("in recursiveChild  not ipynb search");
                            // read the file content and if contains keyword
                            put_result_to_vec(
                                vec,
                                path,
                                keyword.clone(),
                                short_path.clone(),
                                team_id,
                                project_id,
                                filename.to_string(),
                            );
                        }
                    }
                } else {
                    let _r = recursive_global_keyword_search(
                        vec,
                        short_path.clone(),
                        base_path.clone(),
                        keyword.clone(),
                        team_id,
                        project_id,
                    )
                    .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()));
                }
            }
        }
    }
    Ok(Rsp::success("mydata".to_string()))
}

pub fn put_result_to_vec(
    vec: &mut Vec<GlobalSearchResult>,
    path: PathBuf,
    keyword: String,
    short_path: String,
    _team_id: u64,
    project_id: u64,

    filename: String,
) {
    // filename match?
    if filename
        .to_ascii_lowercase()
        .contains(&keyword.to_ascii_lowercase())
    {
        let gsr_file = GlobalSearchResult {
            absolute_path: path.display().to_string(), // x.clone().path.display().to_string(),
            browser_path: short_path.clone(),
            project_id: project_id.to_string(),
            file_name: filename.to_string(),
            cell_id: "".to_string(),
            text: filename.to_string(),
            line: 0,
        };
        vec.push(gsr_file);
    }

    // file content match?
    if let Ok(file_reader) = my_reader::BufReader::open(path.display().to_string()) {
        let mut line_count = 0;
        for line in file_reader {
            line_count += 1;
            if let Ok(line_text) = line {
                if line_text
                    .to_ascii_lowercase()
                    .contains(&keyword.clone().to_ascii_lowercase())
                {
                    let gsr = GlobalSearchResult {
                        absolute_path: path.display().to_string(), /* x.clone().path.display().
                                                                    * to_string(), */
                        browser_path: short_path.clone(),
                        project_id: project_id.to_string().clone(),
                        file_name: filename.to_string(),
                        cell_id: "".to_string(),
                        text: line_text.to_string().trim().to_string(),
                        line: line_count,
                    };
                    vec.push(gsr);
                }
            }
        }
    }
    // let file_reader =
    // io_tool::my_reader::BufReader::open(path.display().to_string()).expect("read file error!");
}

pub fn put_ipynb_result_to_vec(
    vec: &mut Vec<GlobalSearchResult>,
    path: PathBuf,
    keyword: String,
    short_path: String,
    _team_id: u64,
    project_id: u64,
    filename: String,
) -> Result<(), IdpGlobalError> {
    // filename match?
    if filename
        .to_ascii_lowercase()
        .contains(&keyword.to_ascii_lowercase())
    {
        let gsr_file = GlobalSearchResult {
            absolute_path: path.display().to_string(), // x.clone().path.display().to_string(),
            browser_path: short_path.clone(),
            project_id: project_id.to_string(),
            file_name: filename.to_string(),
            cell_id: "".to_string(),
            text: filename.to_string(),
            line: 0,
        };
        vec.push(gsr_file);
    }

    // file content match?
    if let Ok(file_reader) = my_reader::BufReader::open(path.display().to_string()) {
        let mut result_builder = String::new();
        for line in file_reader.flatten() {
            // println!("{}", line.map_err(|_|IdpGlobalError::NoteError("dir open
            // error".to_string()))?);
            // if let Ok(line_value) = line {
            result_builder.push_str(&line);
            // }
        }
        if let Ok(ipynb) = serde_json::from_str::<IpynbFileJson>(&result_builder.to_string()) {
            let cells = ipynb.cells;
            for x in &cells {
                let source_vec = x.clone().source;
                let cell_id = x.clone().metadata.id;
                let mut line_count = 1;
                for line_txt in &source_vec {
                    if line_txt.contains(&keyword.clone()) {
                        let gsr = GlobalSearchResult {
                            absolute_path: path.display().to_string(), /* x.clone().path.display().to_string(), */
                            browser_path: short_path.clone(),
                            project_id: project_id.to_string().clone(),
                            file_name: filename.to_string(),
                            cell_id: cell_id.to_string(),
                            text: line_txt.to_string().trim().to_string(),
                            line: line_count,
                        };
                        vec.push(gsr);
                    }
                    line_count += 1;
                }
            }
        } else {
            error!("{}", &result_builder);
            return Err(IdpGlobalError::NoteError(
                "not json content error!".to_string(),
            ));
        }
    }
    Ok(())

    // {
    //     "cells": [
    //         {
    //             "cell_type": "code",
    //             "source": [
    //                 "import time\n",
    //                 "time.sleep(300)\n",
    //                 "print(\"first finished!\")"
    //             ],
    //             "metadata": {
    //                 "id": "e5979f4c-5c58-42d5-b66d-64f25e75033e",
    //                 "index": 0
    //             }
    //         }
    //     ]
    // }

    // println!("{}", result_builder.to_string());
}

pub async fn new_file(
    relative_path: &str,
    team_id: u64,
    project_id: u64,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access new_file function .......");

    let path = business::path_tool::get_store_full_path(team_id, project_id, relative_path);

    if path.exists() {
        return Err(IdpGlobalError::NoteError(
            "file already exists.".to_string(),
        ));
    }

    info!("new_file_path: {:?}", path);

    if let Some(extension) = path.extension() {
        if extension.eq("ipynb") || extension.eq("idpnb") {
            let notebook = common_model::entity::notebook::Notebook::new(path.to_str().unwrap());
            common_tools::file_tool::write_notebook_to_disk(&path, &notebook).await?;
        } else {
            tokio::fs::File::create(&path).await?;
        }

        return Ok(Rsp::success(()));
    }
    Err(IdpGlobalError::NoteError(
        "InvalidFileTypeTypeError".to_string(),
    ))
}

#[instrument]
pub async fn file_rename(
    path: String,
    source: String,
    desc: String,
    team_id: TeamId,
    project_id: ProjectId,
    auto_close: Option<bool>,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access file_rename function .......");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut abs_path = base_path.clone();
    if !path.eq("/") {
        abs_path.push(crate::business_::path_tool::get_relative_path(&path));
    }

    let mut source_path_str = abs_path.to_str().unwrap().to_string();
    source_path_str += "/";
    source_path_str += &source;

    let mut dest_path_str = abs_path.to_str().unwrap().to_string();
    dest_path_str += "/";
    dest_path_str += &desc;

    tracing::debug!("--> source_path_str={:?}", source_path_str);
    tracing::debug!("--> dest_path_str={:?}", dest_path_str);

    if Path::new(&dest_path_str).exists() {
        return Ok(Rsp::success_without_data()
            .code(NB_RENAME_ERROR_CODE)
            .message(NB_RENAME_ERROR_MSG));
    }

    handler::kernel::shutdown_by_dir_path(project_id, source).await?;

    tokio::fs::rename(source_path_str, dest_path_str).await?;
    Ok(Rsp::success_without_data())
}

#[instrument()]
pub async fn delete_file_or_dir(
    path: String,
    team_id: u64,
    project_id: u64,
    auto_close: Option<bool>,
) -> Result<Rsp<FullFileTreeNode>, IdpGlobalError> {
    info!("access delete_file_or_dir function .......");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut abs_path = base_path.clone();
    if !path.eq("/") {
        abs_path.push(crate::business_::path_tool::get_relative_path(&path));
    }

    tracing::debug!("--> abs_path={:?}", abs_path);
    if !abs_path.exists() {
        let none_node = FullFileTreeNode {
            absolute_path: abs_path.display().to_string(),
            browser_path: path.to_string(),
            project_id: project_id.to_string(),
            file_name: path.to_string(),
            file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
            has_children: false,
            children: vec![],
        };
        Ok(Rsp::success(none_node))
    } else {
        let only_pipeline_support = false;
        let res =
            dir_recursive_load(team_id, project_id, path.clone(), only_pipeline_support).await;
        handler::kernel::shutdown_by_dir_path(project_id, path).await?;

        tokio::fs::remove_dir_all(abs_path).await?;
        res
    }
}

#[instrument]
pub async fn dir_new(
    path: String,
    team_id: u64,
    project_id: u64,
) -> Result<Rsp<()>, IdpGlobalError> {
    info!("access dir_new function .......");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut abs_path = base_path;
    abs_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));

    tracing::debug!("--> abs_path={:?}", abs_path);
    if abs_path.exists() {
        return Ok(Rsp::success_without_data()
            .code(NB_STORE_ERROR_CODE)
            .message(NB_STORE_ERROR_MSG));
    }

    tokio::fs::create_dir_all(abs_path).await?;
    Ok(Rsp::success_without_data())
}

pub async fn export_new_path(
    team_id: String,
    real_path: String,
    output_type: String,
) -> Option<String> {
    if let Ok((mime, _mimetype_str)) = find_mimetype(real_path.clone()) {
        if mime == Mimetype::Notebook && (output_type == "html" || output_type == "pdf") {
            let aim = name_convert(real_path.clone(), output_type.clone());
            let nbconvert_path = get_nbconvert_by_team_id(team_id);
            if let Err(err) =
                export_as_helper(nbconvert_path, real_path.clone(), output_type, aim.clone()).await
            {
                tracing::error!("{err}");
                None
            } else {
                Some(aim)
            }
        } else {
            Some(real_path)
        }
    } else {
        None
    }
}

#[cfg(not)]
#[instrument]
pub async fn export_as(
    path: String,
    output_type: String,
    team_id: u64,
    project_id: u64,
) -> impl IntoResponse {
    info!("access export_as function .......");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let file_name = path.clone();

    let mut abs_path = base_path.clone();
    abs_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));

    tracing::debug!("--> abs_path={:?}", abs_path);
    if !abs_path.exists() {
        return Err(IdpGlobalError::NoteError("path not exist".to_string()));
    }

    if let Some(abs_path) = export_new_path(
        team_id.to_string(),
        abs_path.to_str().unwrap().to_string(),
        output_type.clone(),
    )
    .await
    {
        let file = File::open(abs_path)
            .await
            .map_err(|_| IdpGlobalError::NoteError("file open error".to_string()))?;

        let stream = ReaderStream::new(file);

        let att1 = String::from("attachment; filename=\"");
        let att2 = file_name;
        let att4 = String::from("\"");
        let attachment_str = format!("{}{}{}", att1, att2, att4);

        let body = StreamBody::new(stream);
        let headers = Headers(vec![
            (
                header::CONTENT_TYPE,
                "application/octet-stream;charset=UTF-8",
            ),
            (
                header::CONTENT_DISPOSITION,
                string_to_static_str(attachment_str),
            ),
        ]);

        Ok((headers, body))
    } else {
        Err(IdpGlobalError::NoteError(
            "don't know how to export".to_string(),
        ))
    }
}

pub async fn dir_recursive_load(
    team_id: u64,
    project_id: u64,
    path: String,
    only_pipeline_support: bool,
) -> Result<Rsp<FullFileTreeNode>, IdpGlobalError> {
    info!("access dir_recursive_load api");

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );

    // println!("base_path: {:?}", base_path);
    let mut tree_node = FullFileTreeNode {
        absolute_path: base_path.display().to_string(),
        browser_path: "".to_string(),
        project_id: project_id.to_string(),
        file_name: String::from("notebooks"), //"notebooks",
        file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
        has_children: true,
        children: vec![],
    };

    // let _r = full_recursive_child(
    //     &mut tree_node,
    //     path,
    //     base_path,
    //     team_id,
    //     project_id,
    //     only_pipeline_support,
    // );
    //
    // Ok(Res::success(tree_node)))

    full_recursive_child(
        &mut tree_node,
        path,
        base_path,
        team_id,
        project_id,
        only_pipeline_support,
    )
}

pub fn full_recursive_child(
    node: &mut FullFileTreeNode,
    key_path: String,
    base_path: PathBuf,
    team_id: u64,
    project_id: u64,
    only_pipeline_support: bool,
) -> Result<Rsp<FullFileTreeNode>, IdpGlobalError> {
    let mut abs_list_path = base_path.clone();
    abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &key_path,
    )));

    // println!(
    //     "------- abs_list_path.is_dir recursive_child it : {:?}",
    //     abs_list_path
    // );

    let fullpath = abs_list_path.clone().display().to_string();
    let pos = fullpath.rfind('/').unwrap();
    let (_, filename) = fullpath.split_at(pos + 1);

    let mut the_node = FullFileTreeNode {
        absolute_path: abs_list_path.display().to_string(),
        browser_path: key_path,
        project_id: project_id.to_string(),
        file_name: filename.to_string(),
        file_type: String::from("DIRECTORY"), //"DIRECTORY", or "FILE"
        has_children: false,
        children: vec![],
    };

    if abs_list_path.is_file() {
        the_node.file_type = String::from("FILE");
    } else {
        the_node.file_type = String::from("DIRECTORY");
        the_node.has_children = true;

        if abs_list_path.is_dir() {
            for entry in fs::read_dir(abs_list_path.clone())
                .map_err(|_| IdpGlobalError::NoteError("dir open error".to_string()))?
            {
                let entry =
                    entry.map_err(|_| IdpGlobalError::NoteError("get error".to_string()))?;
                let path = entry.path();

                let mut dir_flag = false;
                if path.clone().is_dir() {
                    dir_flag = true;
                }

                let short_path = path
                    .display()
                    .to_string()
                    .replace(&base_path.clone().display().to_string(), "");

                let fullpath = path.clone().display().to_string();
                let pos = fullpath.rfind('/').unwrap();
                let (_, filename) = fullpath.split_at(pos + 1);
                // println!("id:{}",filename);

                if !filename.starts_with_ignore_ascii_case(".") {
                    let pipeline_ext = ["ipynb", "idpnb", "py", "sh"];
                    let pos_ext = filename.rfind('.');
                    let mut add_to_node_flag = true;
                    if pos_ext == None {
                        if only_pipeline_support {
                            add_to_node_flag = false;
                        }
                    } else {
                        let _pos_ext = pos_ext.unwrap();
                        let (_, filename_ext) = filename.split_at(_pos_ext + 1);
                        let ext = filename_ext.to_ascii_lowercase();
                        if only_pipeline_support {
                            if pipeline_ext.contains(&ext.as_str()) {
                                add_to_node_flag = true;
                            } else {
                                add_to_node_flag = false;
                            }
                        }
                    }

                    if add_to_node_flag || dir_flag {
                        //dir_flag is important, when pipeline is true
                        let _r = full_recursive_child(
                            &mut the_node,
                            short_path.to_string(),
                            base_path.clone(),
                            team_id,
                            project_id,
                            only_pipeline_support,
                        );
                    }
                }
            }
        }
    }
    node.children.push(the_node.clone());

    Ok(Rsp::success(the_node))
}

pub mod my_reader {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;

    // #[deprecated]
    pub struct BufReader {
        reader: io::Lines<io::BufReader<File>>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file).lines();
            Ok(Self { reader })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<String>;

        fn next(&mut self) -> Option<Self::Item> {
            self.reader.next()
        }
    }
}
