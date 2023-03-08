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

use axum::extract::Multipart;
use business::path_tool;
use common_tools::io_tool::file_writer::FileChunk;
use common_tools::io_tool::file_writer::FileSender;

// use common_model::enums::store::PathType;
use crate::common::error::ErrorTrace;

/// NOTE file would lost execute permission after upload
pub async fn upload_file_handler(
    mut multipart: Multipart,
    file_writer: FileSender, // ) -> Result<(HeaderMap, String)> {
) -> Result<String, ErrorTrace> {
    let mut datafile = None;
    let mut total = None;
    let mut index = None;
    let mut file_name = None;
    let mut file_path = None;
    let mut team_id = None;
    let mut project_id = None;
    let mut uuid = None;

    let mut _filename = "".to_string();
    let mut _ext = "";

    while let Some(file) = multipart.next_field().await? {
        let name = file.name().unwrap_or("").to_string();
        if file.file_name().is_some() {
            _filename = file.file_name().unwrap().to_string();
            _ext = _filename.rsplit('.').next().unwrap();
            tracing::info!("ext = {:?}", _ext);
        }

        if name == "datafile" {
            datafile = Some(file.bytes().await?)
        } else {
            let data = file.text().await?;
            match name.as_str() {
                "total" => total = Some(data),
                "index" => index = Some(data),
                "name" => file_name = Some(data),
                "filePath" => file_path = Some(data),
                "teamId" => team_id = Some(data),
                "projectId" => project_id = Some(data),
                "uuid" => uuid = Some(data),
                _ => {}
            }
        };
    }

    if datafile.is_none()
        || total.is_none()
        || index.is_none()
        || file_path.is_none()
        || project_id.is_none()
        || file_name.is_none()
        || uuid.is_none()
    {
        let mut none_fields = vec![];

        if datafile.is_none() {
            none_fields.push("datafile");
        }
        if total.is_none() {
            none_fields.push("total");
        }
        if index.is_none() {
            none_fields.push("index");
        }
        if file_name.is_none() {
            none_fields.push("name");
        }
        if file_path.is_none() {
            none_fields.push("file_path");
        }
        if team_id.is_none() {
            none_fields.push("team_id");
        }
        if project_id.is_none() {
            none_fields.push("project_id");
        }
        if project_id.is_none() {
            none_fields.push("uuid");
        }
        let none_fields_str = none_fields.join(",");
        return Err(ErrorTrace::new(&format!(
            "Missing field {}",
            none_fields_str
        )));
    }

    let file_path = file_path.unwrap_or_default();
    let uuid = uuid.unwrap_or_default();
    let datafile = datafile.unwrap_or_default();
    let index = index.unwrap_or_default().parse::<u64>()? - 1;
    let total = total.unwrap_or_default().parse::<u64>()?;
    let team_id = team_id.unwrap_or_default().parse::<u64>()?;
    let project_id = project_id.unwrap_or_default().parse::<u64>()?;
    tracing::info!("file_path: {:?}", file_path);

    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    let mut abs_list_path = base_path;
    abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &file_path,
    )));
    let file_name = file_name.unwrap();
    abs_list_path.push(Path::new(&file_name));
    let store_parent_dir = business::path_tool::store_parent_dir();
    let tmp_path = format!(
        "{}/store/tmp/{}_{}_{}",
        store_parent_dir.to_str().unwrap(),
        uuid,
        &file_path.replace('/', "_"),
        &file_name
    );

    tracing::info!("abs_list_path: {:?}", abs_list_path);
    tracing::info!("index: {index}, total: {total}, project_id: {project_id}");

    let (tx, rx) = tokio::sync::oneshot::channel();
    file_writer
        .send((
            FileChunk {
                file_dir: tmp_path.clone(),
                file_idx: index,
                total_chunk: total,
                file_data: datafile.to_vec(),
            },
            tx,
        ))
        .await?;
    let n = rx.await?;
    tracing::info!("Finished writing: {:?} / {:?}", n, total);
    if n == -1 {
        return Err(ErrorTrace::new("upload file error"));
    }
    Ok(if n as u64 == total {
        tokio::fs::create_dir_all(
            &abs_list_path
                .parent()
                .unwrap_or(&std::path::PathBuf::from("/")),
        )
        .await?;
        tokio::fs::rename(tmp_path, abs_list_path).await?;
        "over".to_string()
    } else {
        format!("path: {}, index: {}", abs_list_path.to_str().unwrap(), n)
    })
}
