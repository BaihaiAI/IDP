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
use common_model::service::rsp::Rsp;
use common_tools::io_tool::file_writer::FileChunk;
use common_tools::io_tool::file_writer::FileSender;
use tracing::info;
use tracing::instrument;

use crate::api_model::project::ProjectDtoStr;
use crate::api_model::project::ProjectRet;
use crate::api_model::project::ProjectType;
use crate::common::error::ErrorTrace;
use crate::common::error::IdpGlobalError;
use crate::handler::git_service;
use crate::status_code::API_FAIL_CODE;
use crate::status_code::API_FAIL_MSG;
use crate::status_code::PROJECT_CREATE_FINAL_FAIL_CODE;
use crate::status_code::PROJECT_CREATE_FINAL_FAIL_MSG;
use crate::status_code::PROJECT_GET_PROJECT_ID_FAIL_CODE;
use crate::status_code::PROJECT_GET_PROJECT_ID_FAIL_MSG;
use crate::status_code::PROJECT_NAME_UNIQ_CREATE_FAIL_CODE;
use crate::status_code::PROJECT_NAME_UNIQ_CREATE_FAIL_CODE_RESOURCE_API;
use crate::status_code::PROJECT_NAME_UNIQ_CREATE_FAIL_MSG;
use crate::status_code::PROJECT_NOT_FOUND_FAIL_CODE;
use crate::status_code::PROJECT_NOT_FOUND_FAIL_MSG;
use crate::status_code::SUCCESS_CODE;

static FULL_PROJECT_NEW_URL: &str = "http://idp-resource-svc:10005/api/v1/project/new";
static FULL_PROJECT_DELETE_URL: &str = "http://idp-resource-svc:10005/api/v1/project/delete";

pub fn parse_return_success_code(retcode: u32) -> bool {
    if retcode == SUCCESS_CODE || retcode == 200u32 {
        return true;
    } else if retcode > 10000000 {
        let m = retcode / 10000000;
        if m == 2 {
            return true;
        }
    } else {
        return false;
    }
    false
}

// fn create_project_children_dir(project_root: String) {
//     cmd(sh /opt/idp-note/bin/cr_project_children  $project_root)
// }

pub async fn create_project_children_dir_one_by_one(
    team_id: u64,
    project_id: u64,
) -> Result<(), ErrorTrace> {
    let project_notebook_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    tracing::debug!(
        "project_notebook_root_path: {:?}",
        project_notebook_root_path
    );
    tokio::fs::create_dir(project_notebook_root_path).await?;
    let project_tmp_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::TMP,
    );
    tracing::debug!("project_tmp_root_path: {:?}", project_tmp_root_path);
    tokio::fs::create_dir(project_tmp_root_path).await?;

    let project_snapshot_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::SNAPSHOT,
    );
    tracing::debug!(
        "project_snapshot_root_path: {:?}",
        project_snapshot_root_path
    );
    tokio::fs::create_dir(project_snapshot_root_path).await?;

    let project_pipeline_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::PIPELINE,
    );
    tracing::debug!(
        "project_pipeline_root_path: {:?}",
        project_pipeline_root_path
    );
    tokio::fs::create_dir(&project_pipeline_root_path).await?;

    let project_job_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::JOB,
    );
    tracing::debug!("project_job_root_path: {:?}", project_job_root_path);

    tokio::fs::create_dir(&project_job_root_path).await?;

    let project_trash_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::TRASH,
    );
    tracing::debug!("project_trash_root_path: {:?}", project_trash_root_path);
    tokio::fs::create_dir(project_trash_root_path).await?;

    let project_miniconda3_root_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::MINICONDA3,
    );
    tracing::debug!(
        "project_miniconda3_root_path: {:?}",
        project_miniconda3_root_path
    );
    tokio::fs::create_dir(project_miniconda3_root_path.clone()).await?;

    std::fs::write(
        crate::business_::path_tool::project_conda_env_file_path(team_id, project_id),
        "python39",
    )?;

    let project_root = path_tool::project_root(team_id, project_id);
    tracing::debug!(
        "create_project_children_dir_one_by_one chown ray to project_root: {:?}",
        project_root
    );

    let mut cmd = std::process::Command::new("chown");
    cmd.arg("-R").arg("ray:users").arg(project_root);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

#[instrument]
pub async fn delete(team_id: u64, project_id: u64) -> Result<Rsp<()>, err::ErrorTrace> {
    info!("access project delete function .......");
    // let ret = delete_project_from_db_by_api(project_id.to_string()).await;

    let project_path_ = path_tool::project_root(team_id, project_id);
    let project_base_path = std::path::Path::new(&project_path_);

    tracing::debug!("--> project_base_path={:?}", project_base_path);
    if !project_base_path.exists() {
        return Err(ErrorTrace::new(PROJECT_NOT_FOUND_FAIL_MSG).code(PROJECT_NOT_FOUND_FAIL_CODE));
    }

    delete_project_from_db_by_api(project_id.to_string()).await?;

    tracing::debug!("--> project_base_path={:?}", project_base_path);
    if project_base_path.exists() {
        // delete the project path and all files
        tokio::fs::remove_dir_all(project_path_).await?;
    }
    Ok(Rsp::success(()))
}

async fn delete_project_from_db_by_api(id: String) -> Result<bool, err::ErrorTrace> {
    let project_ret = reqwest::Client::new()
        .post(FULL_PROJECT_DELETE_URL)
        .json(&serde_json::json!({ "id": id }))
        .send()
        .await?
        .json::<ProjectRet>()
        .await?;
    tracing::debug!(
        "--> delete_project_from_db_by_api project_ret={:?}",
        project_ret
    );

    if parse_return_success_code(project_ret.code) {
        Ok(true)
    } else {
        tracing::debug!(
            "--> parse_return_success_code error ={:?} {:?}",
            project_ret.code,
            project_ret.clone().message.unwrap()
        );
        Err(err::ErrorTrace::new(&project_ret.message.unwrap()).code(project_ret.code))
    }
}

pub async fn get_new_project_id(
    project_name: String,
    creator: String,
    team_id: String,
) -> Result<String, IdpGlobalError> {
    let project_ret = reqwest::Client::new()
        .post(FULL_PROJECT_NEW_URL)
        .json(&serde_json::json!({
            "name": project_name,
            "creator": creator,
            "teamId": team_id
        }))
        .send()
        .await?
        .json::<ProjectRet>()
        .await?;

    tracing::debug!("--> get_new_project_id project_ret={:?}", project_ret);
    // let mut project_id: String = "".to_string();

    let code_1 = project_ret.code;
    let code_2 = project_ret.code;

    if parse_return_success_code(code_1) {
        tracing::debug!("--> get_new_project_id parse_return_success_code true");
        let project_id = project_ret.data.id.unwrap();
        Ok(project_id)
    } else {
        tracing::error!("--> get_new_project_id parse_return_success_code false");
        tracing::error!(
            "--> get_new_project_id project_ret.message.unwrap() = {:?} ",
            project_ret.clone().message.unwrap()
        );
        if code_1 == PROJECT_NAME_UNIQ_CREATE_FAIL_CODE_RESOURCE_API {
            Ok("-1".to_string())
        } else {
            Err(IdpGlobalError::ErrorCodeMsg(
                code_2,
                project_ret.message.unwrap(),
            ))
        }
    }
}

/// upload_file_post
pub async fn new_project(
    mut multipart: Multipart,
    file_writer: FileSender,
) -> Result<String, ErrorTrace> {
    let mut datafile = None;
    let mut total = None;
    let mut index = None;
    // let mut file_name = None;
    let mut project_dto_str = None;

    let mut need_upload_file = false;

    // let mut err_msg = "";

    while let Some(file) = multipart.next_field().await? {
        let name = file.name().unwrap_or("").to_string();
        if name == "datafile" {
            datafile = Some(file.bytes().await?);
            if !datafile.as_ref().unwrap().is_empty() {
                tracing::debug!(
                    "datafile.as_ref().unwrap().len() = {:?}",
                    datafile.as_ref().unwrap().len()
                );
                if datafile.as_ref().unwrap().len() > 10 {
                    need_upload_file = true;
                }
            } else {
                tracing::debug!("datafile.as_ref().unwrap().is_empty() ............");
            }
        } else {
            let data = file.text().await?;
            match name.as_str() {
                "total" => total = Some(data),
                "index" => index = Some(data),
                // "name" => file_name = Some(data),
                "projectDtoStr" => project_dto_str = Some(data),
                _ => {}
            }
        };
    }
    // let a: String = 1 + 'a';

    //
    tracing::debug!("project_dto_str: {:?}", project_dto_str);
    let project_dto_str_obj: ProjectDtoStr = serde_json::from_str(&project_dto_str.unwrap())?;
    tracing::debug!("project_dto_str_obj: {:?}", project_dto_str_obj);

    let name_op = project_dto_str_obj.project_name.clone();
    let creator_op = project_dto_str_obj.creator.clone();
    let team_id_op = project_dto_str_obj.team_id.clone();

    //set upload file name
    let file_name = project_dto_str_obj.project_name.clone();

    tracing::debug!("file_name optione before : {:?}", file_name);
    let mut tmp_file_name = file_name.unwrap();
    tmp_file_name += ".zip";
    let file_name = Some(tmp_file_name);
    tracing::debug!("file_name optione after: {:?}", file_name);
    tracing::debug!(
        "set upload file_name by project name ,file_name = : {:?}",
        file_name
    );

    let result = get_new_project_id(
        name_op.unwrap(),
        creator_op.unwrap(),
        team_id_op.clone().unwrap(),
    )
    .await
    .map_err(|_| IdpGlobalError::ErrorCodeMsg(API_FAIL_CODE, API_FAIL_MSG.to_string()));

    // for get project_id
    let project_id_global = match result {
        Ok(project_id_str) => {
            if !project_id_str.contains('-') {
                tracing::debug!(
                    "Ok, get_new_project_id project_id_str=: {:?}",
                    project_id_str
                );
                project_id_str.clone()
            } else {
                tracing::debug!(
                    "No, get_new_project_id project_id_str=: {:?}",
                    project_id_str
                );
                return Err(ErrorTrace::new(PROJECT_NAME_UNIQ_CREATE_FAIL_MSG)
                    .code(PROJECT_NAME_UNIQ_CREATE_FAIL_CODE));
            }
        }
        Err(_err) => {
            return Err(ErrorTrace::new(PROJECT_GET_PROJECT_ID_FAIL_MSG)
                .code(PROJECT_GET_PROJECT_ID_FAIL_CODE));
        }
    };

    tracing::debug!(" need_upload_file=: {:?}", need_upload_file);

    let team_id = team_id_op.unwrap_or_default().parse::<u64>()?;

    //upload file by project_id
    if need_upload_file {
        //upload the zip file
        if datafile == None || total == None || index == None || file_name == None {
            let mut none_fields = vec![];

            if datafile == None {
                none_fields.push("datafile");
            }
            if total == None {
                none_fields.push("total");
            }
            if index == None {
                none_fields.push("index");
            }
            // if file_name == None {
            //     none_fields.push("name");
            // }

            let none_fields_str = none_fields.join(",");
            return Err(ErrorTrace::new(&format!(
                "Missing field {}",
                none_fields_str
            )));
        }

        // let file_path = file_path.unwrap_or("".to_string());
        let datafile = datafile.unwrap_or_default();
        let index = index.unwrap_or_default().parse::<u64>()? - 1;
        let total = total.unwrap_or_default().parse::<u64>()?;
        // let project_id = project_id_global.clone().unwrap_or_default().parse()?;

        let tmp_project_id = 0u64;

        // let team_id = team_id_op.unwrap_or_default().parse::<u64>()?;

        let base_path = path_tool::get_store_path(
            team_id,
            tmp_project_id,
            business::business_term::ProjectFolder::NOTEBOOKS,
        );
        let mut abs_list_path = base_path.clone();
        // abs_list_path.push(store_path::get_relative_path(Path::new(&file_path)));
        abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
            &file_name.clone().unwrap_or_else(|| "".to_string()),
        )));

        // let out_path = abs_list_path.clone();

        tracing::debug!("abs_list_path: {:?}", abs_list_path);
        tracing::debug!("index: {:?}", index);
        tracing::debug!("total: {:?}", total);
        tracing::debug!("tmp_project_id: {:?}", tmp_project_id);

        let (tx, rx) = tokio::sync::oneshot::channel();
        file_writer
            .send((
                FileChunk {
                    file_dir: abs_list_path.to_str().unwrap_or("").to_string(),
                    file_idx: index as u64,
                    total_chunk: total,
                    file_data: datafile.to_vec(),
                },
                tx,
            ))
            .await?;
        let n = rx.await?;
        tracing::debug!("Finished writing: {:?} / {:?}", n, total);
        if n == -1 {
            return Err(ErrorTrace::new("new_proj: upload file error"));
        }
    }

    //do project business by project_type

    let ret_code = do_project_business(
        team_id,
        project_id_global.clone(),
        file_name.unwrap_or_else(|| "".to_string()),
        project_dto_str_obj,
    )
    .await?
    .code;

    tracing::debug!("ret_business success ret_code:{:?}", ret_code);

    if parse_return_success_code(ret_code) {
        Ok("success".to_string())
    } else {
        Err(ErrorTrace::new(PROJECT_CREATE_FINAL_FAIL_MSG).code(PROJECT_CREATE_FINAL_FAIL_CODE))
    }
}

pub async fn do_project_business(
    team_id: u64,
    project_id_str: String,
    file_name: String,
    project_dto_str: ProjectDtoStr,
) -> Result<Rsp<()>, err::ErrorTrace> {
    let project_id: u64 = project_id_str.parse::<u64>()?;
    tracing::debug!("-->project_id={:?}", project_id);

    let git_url_op = project_dto_str.git_url;
    let git_info_op = project_dto_str.git_info;
    let project_type = project_dto_str.project_type;

    let project_path_ = path_tool::project_root(team_id, project_id);
    let project_root_path = std::path::Path::new(&project_path_);

    tracing::debug!("project_root_path: {:?}", project_root_path);

    // create project root Path if needed
    tokio::fs::create_dir(project_root_path.to_path_buf()).await?;

    // create all children directories.
    create_project_children_dir_one_by_one(team_id, project_id).await?;
    // tracing::debug!("create_project_children_dir_one_by_one ret={:?}", ret);

    let project_work_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );

    // project_type
    tracing::debug!("project_type: {:?}", project_type);
    match project_type {
        ProjectType::Git => {
            tracing::debug!("ProjectType git  ........");
            let git_url = git_url_op.unwrap();
            let git_info = git_info_op.unwrap();

            tracing::debug!("git_url: {:?}", git_url);
            tracing::debug!("project_work_path: {:?}", project_work_path);

            git_service::git_clone(
                git_url,
                git_info,
                project_work_path.display().to_string(),
                project_id,
            )
            .await?;
        }
        ProjectType::File => {
            // todo: upload dir from website
            let tmp_project_id = 0u64;
            let base_path = path_tool::get_store_path(
                team_id,
                tmp_project_id,
                business::business_term::ProjectFolder::NOTEBOOKS,
            );
            let mut abs_list_path = base_path.clone();
            abs_list_path.push(crate::business_::path_tool::get_relative_path(Path::new(
                &file_name,
            )));

            tracing::debug!("##base_path     = {:?}", base_path);
            tracing::debug!("##abs_list_path = {:?}", abs_list_path.clone());

            if abs_list_path.exists() {
                tracing::debug!("##abs_list_path.exists()!");
                tracing::debug!(
                    "##extract it {:?} to real project path start!",
                    abs_list_path.clone()
                );
                let target_base_path = path_tool::get_store_path(
                    team_id,
                    project_id,
                    business::business_term::ProjectFolder::NOTEBOOKS,
                );
                tracing::debug!("##project target_base_path= {:?}", target_base_path);

                //if web zip without fold , then use this command
                // if let Err(error) = Command::new(cd $base_path;unzip -o $file_name -d $target_base_path)

                let mut cmd = tokio::process::Command::new("unzip");
                cmd.arg("-o").arg(file_name).current_dir(base_path);
                tracing::info!("cmd = {cmd:?}");
                let output = cmd.output().await?;
                if !output.status.success() {
                    return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
                }

                let zip_path = abs_list_path.to_str().unwrap();
                let the_zip_path = &zip_path.trim_end_matches(".zip");
                // tracing::debug!("##the_zip_path= {:?}", the_zip_path.clone());
                // let source_files_dirs: String = format!("{}{}", the_zip_path, "/* ");
                // tracing::debug!("##!!!source_files_dirs={:?}", source_files_dirs);

                tracing::info!(
                    "tokio::fs::rename: the_zip_path={the_zip_path} target_base_path={target_base_path:?}"
                );
                tokio::fs::rename(the_zip_path, target_base_path).await?;
                // let source_files_dirs: String = format!("{}{}", the_zip_path, "/* ");
                // tracing::debug!("##!!!source_files_dirs={:?}", source_files_dirs);
                // let mut cmd = Command::new("bash");
                // cmd
                //     .arg("-c")
                //     .arg(format!(
                //         "mv {} {}",
                //         source_files_dirs,
                //         target_base_path.to_str().unwrap()
                //     ));
                // tracing::info!("cmd = {cmd:?}");
                // let output = cmd.output()?;
                // if !output.status.success() {
                //     return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
                // }

                tracing::debug!("##project extract it to real project path end!");
                tracing::info!("##delete the tmp zip file: {:?} start!", abs_list_path);
                if let Err(err) = tokio::fs::remove_file(&abs_list_path).await {
                    tracing::error!("{err}");
                }
                // because we use mv, so the origin temp zip path must be not found
                // if let Err(err) = tokio::fs::remove_dir(the_zip_path.to_string()).await {
                //     tracing::error!("{err}");
                // }
                tracing::info!("##delete the_zip_path is {:?}", the_zip_path);
                tracing::info!("##delete the tmp zip file: {:?} end  !", abs_list_path);

                if !project_root_path.join("notebooks").join(".git").exists() {
                    std::process::Command::new("git")
                        .arg("init")
                        .arg(".")
                        .current_dir(project_root_path.join("notebooks"))
                        .spawn()?
                        .wait()?;
                }
            }

            tracing::info!("create file project success!");
        }
        ProjectType::Default => {
            // git2::Repository::init(path)
            std::process::Command::new("git")
                .arg("init")
                .arg(".")
                .current_dir(project_root_path.join("notebooks"))
                .spawn()?
                .wait()?;
            tracing::info!("create default project success!");
        }
    };
    Ok(Rsp::success(()))
}

#[cfg(not)]
pub async fn ray_chown_fix_one_time(pg_pool: sqlx::PgPool) -> Result<Rsp<()>, ErrorTrace> {
    #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
    #[serde(rename_all = "camelCase")]
    pub struct TeamProject {
        pub team_id: i64,
        pub id: i32,
    }
    info!("access  ray_chown_fix_one_time function .......");
    let record_list: Vec<TeamProject> = sqlx::query_as("SELECT team_id,id from project")
        .fetch_all(&pg_pool)
        .await
        .unwrap();

    for (_index, item) in record_list.iter().enumerate() {
        println!("teamid={:?},projectid={:?}", item.team_id, item.id);

        let project_root = path_tool::project_root(item.team_id as u64, item.id as u64);

        let mut cmd = std::process::Command::new("chown");
        cmd.arg("-R").arg("ray:users").arg(project_root);
        tracing::info!("cmd = {cmd:?}");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        }

        // let project_job_root_path = path_tool::get_store_path(
        //     item.team_id as u64,
        //     item.id as u64,
        //     business::business_term::ProjectFolder::JOB,
        // );
        // let job_dir = project_job_root_path
        //     .into_os_string()
        //     .into_string()
        //     .unwrap();
        // //set the operation permission to ray user on project_job_root_path path
        //
        // let mut cmd = std::process::Command::new("chown");
        // cmd.arg("-R").arg("ray:users").arg(job_dir);
        // tracing::info!("cmd = {cmd:?}");
        // let output = cmd.output()?;
        // if !output.status.success() {
        //     return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        // }
        //
        // let project_pipeline_root_path = path_tool::get_store_path(
        //     item.team_id as u64,
        //     item.id as u64,
        //     business::business_term::ProjectFolder::PIPELINE,
        // );
        // let pipeline_dir = project_pipeline_root_path
        //     .into_os_string()
        //     .into_string()
        //     .unwrap();
        // //set the operation permission to ray user on project_job_root_path path
        //
        // let mut cmd = std::process::Command::new("chown");
        // cmd.arg("-R").arg("ray:users").arg(pipeline_dir);
        // tracing::info!("cmd = {cmd:?}");
        // let output = cmd.output()?;
        // if !output.status.success() {
        //     return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        // }
    }

    Ok(Rsp::success(()))
}
