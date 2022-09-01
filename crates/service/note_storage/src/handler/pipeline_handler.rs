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

use axum::Json;
use business::path_tool;
use common_model::service::rsp::Rsp;
use tracing::error;
use tracing::info;
use tracing::instrument;

use crate::api_model::pipeline::PipelineResultDto;
use crate::common::error::IdpGlobalError;
use crate::status_code::NB_NO_MORE_CONTENT_ERROR_CODE;
use crate::status_code::NB_NO_MORE_CONTENT_ERROR_MSG;

#[instrument]
pub async fn cat_result(
    team_id: u64,
    project_id: u64,
    path: String,
    job_id: String,
    job_instance_id: String,
    task_instance_id: String,
    start: i32,
    mut limit: u32,
) -> Result<Rsp<PipelineResultDto>, IdpGlobalError> {
    // get output path

    let mut job_id_u64 = 0u64;
    if let Ok(p) = job_id.parse::<u64>() {
        job_id_u64 = p;
    }
    let mut job_instance_id_u64 = 0u64;
    if let Ok(p) = job_instance_id.parse::<u64>() {
        job_instance_id_u64 = p;
    }
    let mut task_instance_id_u64 = 0u64;
    if let Ok(p) = task_instance_id.parse::<u64>() {
        task_instance_id_u64 = p;
    }

    let output_path = path_tool::get_pipeline_output_path(
        team_id,
        project_id,
        &path,
        job_id_u64,
        job_instance_id_u64,
        task_instance_id_u64,
        false,
    );
    let buf = match std::fs::read(&output_path) {
        Ok(buf) => buf,
        Err(error) => {
            error!("check output file line error:{}", error.to_string());
            return Err(IdpGlobalError::NoteError(error.to_string()));
        }
    };
    let total_line = bytecount::count(&buf, b'\n');
    if total_line == 0 {
        return Ok(Rsp::success(PipelineResultDto {
            content: "".to_string(),
            total_line: 0,
        }));
    }

    if start > total_line.try_into().unwrap() {
        return Err(IdpGlobalError::ErrorCodeMsg(
            NB_NO_MORE_CONTENT_ERROR_CODE,
            NB_NO_MORE_CONTENT_ERROR_MSG.to_string(),
        ));
    }
    info!("output_path: {:?}", output_path);
    let mut reader =
        super::workspace::my_reader::BufReader::open(output_path).expect("read file error!");

    let mut result_builder = String::new();
    let mut line_count = 0;
    while line_count < start - 1 {
        line_count += 1;
        reader.next();
    }
    while limit > 0 {
        if let Some(Ok(value)) = reader.next() {
            result_builder.push_str(&value);
        }
        limit -= 1;
    }
    Ok(Rsp::success(PipelineResultDto {
        content: result_builder,
        total_line: total_line.try_into().unwrap(),
    }))

    // let mut cmd = std::process::Command::new("wc");
    // cmd.arg("-l").arg(&output_path);
    // tracing::info!("cmd = {cmd:?}");
    // let output = cmd.output()?;
    // let res = if !output.status.success() {
    //     Err(std::io::Error::new(
    //         std::io::ErrorKind::Other,
    //         String::from_utf8_lossy(&output.stderr),
    //     ))
    // } else {
    //     Ok(String::from_utf8_lossy(&output.stdout))
    // };

    // match res {
    //     Err(error) => {
    //         error!("check output file line error:{}", error.to_string());
    //         Err(IdpGlobalError::NoteError(error.to_string()))
    //     }
    //     Ok(total_line) => {
    //         if let Some(total_line) = total_line.split_whitespace().next() {
    //             let total_line = total_line.parse::<i32>().expect("number format error");

    //             // {"data":{"content":"jjjjj\n","totalLine":2},"code":21000000,"message":"success"}
    //             if total_line == 0 {
    //                 return Ok(Rsp::success(PipelineResultDto {
    //                     content: "".to_string(),
    //                     total_line: 0,
    //                 }));
    //             }

    //             if start > total_line {
    //                 return Err(IdpGlobalError::ErrorCodeMsg(
    //                     NB_NO_MORE_CONTENT_ERROR_CODE,
    //                     NB_NO_MORE_CONTENT_ERROR_MSG.to_string(),
    //                 ));
    //             }
    //             info!("output_path: {:?}", output_path);
    //             let mut reader = super::workspace::my_reader::BufReader::open(output_path)
    //                 .expect("read file error!");

    //             let mut result_builder = String::new();
    //             let mut line_count = 0;
    //             while line_count < start - 1 {
    //                 line_count += 1;
    //                 reader.next();
    //             }
    //             while limit > 0 {
    //                 if let Some(Ok(value)) = reader.next() {
    //                     result_builder.push_str(&value);
    //                 }
    //                 limit -= 1;
    //             }

    //             Ok(Rsp::success(PipelineResultDto {
    //                 content: result_builder,
    //                 total_line: total_line.try_into().unwrap(),
    //             }))
    //         } else {
    //             Err(IdpGlobalError::NoteError(
    //                 "cannot check file total_line error.".to_string(),
    //             ))
    //         }
    //     }
    // }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyDto {
    pub path: String,
    pub copy_type: String,
    pub team_id: String,
    pub project_id: String,
    pub job_id: String,
    pub instance_id: String,
    pub task_id: String,
}

pub async fn copy(Json(payload): Json<CopyDto>) -> Result<Rsp<()>, err::ErrorTrace> {
    info!("access pipeline/copy api");

    // let team_id = get_cookie_value_by_team_id(cookies); //If only the svc call can't get the cookie, so it can't be used

    // Handling request parameters
    let (path, team_id, project_id, job_id, copy_type, instance_id, task_id) = (
        payload.path,
        payload.team_id,
        payload.project_id,
        payload.job_id,
        payload.copy_type,
        payload.instance_id,
        payload.task_id,
    );

    copy_(
        team_id,
        path,
        project_id,
        job_id,
        instance_id,
        task_id,
        copy_type,
    )
    .await?;
    Ok(Rsp::success(()))

    // if ret {
    //     Ok(Rsp::success(()))
    // } else {
    //     Err(IdpGlobalError::ErrorCodeMsg(
    //         COPY_NOTEBOOK_ERROR_CODE,
    //         COPY_NOTEBOOK_ERROR_MSG.to_string(),
    //     ))
    // }
}

#[instrument]
pub async fn copy_(
    team_id_str: String,
    path: String,
    project_id: String,
    job_id: String,
    instance_id: String,
    task_id: String,
    copy_type: String,
) -> Result<(), err::ErrorTrace> {
    const FILE_SEPARATOR: &str = "/";
    let origin_path;
    let target_path;

    let mut origin_path_string = "".to_string();
    let mut target_path_string = "".to_string();

    let team_id = team_id_str.parse::<u64>().unwrap_or_default();
    tracing::debug!("--> copy={:?}", team_id);

    let mut copy_mirror = false;

    match copy_type.as_str() {
        "save" => {
            origin_path = path_tool::get_store_path(
                team_id,
                project_id.parse().unwrap(),
                business::business_term::ProjectFolder::NOTEBOOKS,
            );
            origin_path_string += origin_path.to_str().unwrap();
            origin_path_string += path.as_str();

            target_path = path_tool::get_store_path(
                team_id,
                project_id.parse().unwrap(),
                business::business_term::ProjectFolder::PIPELINE,
            );
            target_path_string += target_path.to_str().unwrap();
            target_path_string += FILE_SEPARATOR;
            target_path_string += job_id.as_str();
            target_path_string += path.as_str();
            copy_mirror = true;
        }
        "run" => {
            origin_path = path_tool::get_store_path(
                team_id,
                project_id.parse().unwrap(),
                business::business_term::ProjectFolder::PIPELINE,
            );
            origin_path_string += origin_path.to_str().unwrap();
            origin_path_string += FILE_SEPARATOR;
            origin_path_string += job_id.as_str();
            origin_path_string += path.as_str();

            target_path = path_tool::get_store_path(
                team_id,
                project_id.parse().unwrap(),
                business::business_term::ProjectFolder::PIPELINE,
            );
            target_path_string += target_path.to_str().unwrap();
            target_path_string += FILE_SEPARATOR;
            target_path_string += &job_id.clone();
            target_path_string += "-";
            target_path_string += instance_id.clone().as_str();
            target_path_string += "-";
            target_path_string += task_id.clone().as_str();
            target_path_string += FILE_SEPARATOR;
            target_path_string += path.as_str()
        }
        _ => {}
    }

    tracing::debug!("--> origin_path_string={:?}", origin_path_string);
    tracing::debug!("--> target_path_string={:?}", target_path_string);

    let origin_path = std::path::Path::new(&origin_path_string);
    let target_path = std::path::Path::new(&target_path_string);
    tracing::info!("pipeline/cp: {origin_path:?} -> {target_path:?}");

    let target_dir = target_path.parent().unwrap();
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir)?;
    }
    std::fs::copy(origin_path, target_path)?;
    if copy_mirror {
        let file_name = target_path_string.to_lowercase();
        if file_name.ends_with(".ipynb") || file_name.ends_with("idpnb") {
            tracing::debug!(
                "--> notebook_job_file_clear_output target_path_string ={:?}",
                target_path_string
            );
            let _ret = notebook_job_file_clear_output(&target_path_string).await;
            tracing::debug!("--> notebook_job_file_clear_output _ret ={:?}", _ret);
        } else {
            tracing::debug!(
                "--> need not notebook_job_file_clear_output target_path_string ={:?}",
                target_path_string
            );
        }
    }
    #[cfg(unix)]
    if target_path.ends_with(".sh") {
        std::fs::set_permissions(
            target_path,
            <std::fs::Permissions as std::os::unix::prelude::PermissionsExt>::from_mode(0o755),
        )?;
    }
    Ok(())
}

async fn notebook_job_file_clear_output(file_path: &str) -> Result<(), err::ErrorTrace> {
    let mut notebook_job_file = common_tools::file_tool::read_notebook_from_disk(file_path).await?;
    // clear cell outputs
    for cell in notebook_job_file.cells.iter_mut() {
        cell.outputs.clear();
    }
    let file_path_buf = std::path::PathBuf::from(file_path);
    //update the notebook_job_file
    common_tools::file_tool::write_notebook_to_disk(&file_path_buf, &notebook_job_file).await?;

    Ok(())
}
