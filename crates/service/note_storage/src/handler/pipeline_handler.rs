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
use axum::Json;
use business::path_tool;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;
use tracing::info;
use tracing::instrument;

use crate::api_model::pipeline::PipelineCatResultReq;
use crate::api_model::pipeline::PipelineResultDto;
use crate::status_code::NB_NO_MORE_CONTENT_ERROR_CODE;
use crate::status_code::NB_NO_MORE_CONTENT_ERROR_MSG;

pub async fn cat_result(
    // team_id: u64,
    // project_id: u64,
    // path: String,
    // job_id: String,
    // job_instance_id: String,
    // task_instance_id: String,
    // start: i32,
    // mut limit: u32,
    Query(req): Query<PipelineCatResultReq>,
) -> Result<Rsp<PipelineResultDto>, ErrorTrace> {
    let PipelineCatResultReq {
        start,
        limit,
        job_id,
        job_instance_id,
        task_instance_id,
        path,
        team_id,
        project_id,
    } = req;
    if start == 0 {
        return Err(ErrorTrace::new("start must >= 1").code(ErrorTrace::CODE_WARNING));
    }
    let start = start - 1;

    let output_path = path_tool::get_pipeline_output_path(
        team_id,
        project_id,
        &path,
        job_id,
        job_instance_id,
        task_instance_id,
        false,
    )
    .map_err(|err| ErrorTrace::new(&err))?;
    let output_content = match std::fs::read_to_string(&output_path) {
        Ok(buf) => buf,
        Err(error) => {
            let error = format!("check output file line error: {output_path:?} {error}",);
            return Err(ErrorTrace::new(&error));
        }
    };
    info!("output_path: {:?}", output_path);
    let total_line = output_content.lines().count();
    if total_line == 0 {
        return Ok(Rsp::success(PipelineResultDto {
            content: "".to_string(),
            total_line: 0,
        }));
    }

    if start > total_line {
        return Err(
            ErrorTrace::new(NB_NO_MORE_CONTENT_ERROR_MSG).code(NB_NO_MORE_CONTENT_ERROR_CODE)
        );
    }

    let output = output_content
        .lines()
        // .split_inclusive('\n')
        .skip(start)
        .take(limit)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(Rsp::success(PipelineResultDto {
        content: output,
        total_line,
    }))
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
