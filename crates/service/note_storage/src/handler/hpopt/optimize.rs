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
use cache_io::CacheService;
use cache_io::OptimizeState;
use common_model::Rsp;
use tokio::process::Child;
use tokio::process::Command;
use tracing::error;
use tracing::info;

use crate::common::error::IdpGlobalError;
use crate::status_code;

pub async fn get_optimize_objective_example_names() -> Result<Vec<String>, std::io::Error> {
    // get datasource dir path
    let optimize_objective_example_dir_path =
        business::path_tool::get_optimize_objective_example_path();
    // create file struct by path and get all file name.
    let mut example_name_list = Vec::new();
    if let Ok(dir) = std::fs::read_dir(optimize_objective_example_dir_path) {
        dir.for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            example_name_list.push(file_name.to_string());
                        }
                    }
                }
            }
        });
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no exists example file",
        ));
    }
    Ok(example_name_list)
}

pub async fn get_optimize_objective_code_content(
    objective_example_name: String,
) -> Result<String, IdpGlobalError> {
    let file_path = format!(
        "{}/{}",
        business::path_tool::get_optimize_objective_example_path(),
        objective_example_name
    );
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub const HPOPT_PYTHON_HEADER: &str = r#"
import optuna
import sys
"#;
pub const HPOPT_PYTHON_LOAD_FUN: &str = r#"
study = optuna.load_study"#;
pub const HPOPT_PYTHON_FOOTER: &str = r#"
study.optimize(objective, n_trials=int(sys.argv[1]))
"#;
//TODO: support parallel.
pub async fn study_optimize_run(
    team_id: TeamId,
    project_id: ProjectId,
    study_id: i64,
    study_name: String,
    db_name: String,
    n_trials: i64,
    redis_cache: &CacheService,
) -> Result<String, IdpGlobalError> {
    //todo!
    let (_fun_file_path, content) = crate::handler::hpopt::study::get_study_objective_code(
        team_id,
        project_id,
        study_id,
        db_name.clone(),
    )
    .await?;
    // get db_url
    let db_url = super::control::get_dburl_by_db_file_name(team_id, project_id, &db_name);
    let opt_run_path = create_opt_python_file(
        content,
        db_name.clone(),
        db_url,
        study_name,
        study_id,
        team_id,
        project_id,
    )
    .await?;

    // get env python path
    let python_path = business::path_tool::get_conda_python_path(team_id, project_id);

    let mut cmd = Command::new("bash");
    //get log path
    let log_path = format!("{}.out", opt_run_path,);
    //redirect stdout and stderror to log file.
    let command = format!(
        "{} {} {} 1>{} 2>&1",
        python_path, opt_run_path, n_trials, log_path
    );
    cmd.arg("-c").arg(command);

    tracing::info!("cmd: {:?}", cmd);
    let child = cmd.spawn()?;

    if child.id().is_none() {
        return Err(IdpGlobalError::ErrorCodeMsg(
            status_code::HPOPT_RUN_START_FAIL_CODE,
            status_code::HPOPT_RUN_START_FAIL_MSG.to_string(),
        ));
    }
    let opt_run_key = opt_run_key(project_id, db_name, study_id, child.id().unwrap());
    tokio::spawn(opt_state_monitor(
        child,
        redis_cache.clone(),
        opt_run_key.clone(),
    ));
    //

    Ok(opt_run_key)
}

#[inline]
fn opt_run_key(project_id: ProjectId, db_name: String, study_id: i64, pid: u32) -> String {
    // format : project_id:db_name:study_id:pid
    format!(
        "{}{}:{}:{}:{}",
        cache_io::keys::OPTIMIZE_STATE_PREFIX,
        project_id,
        db_name,
        study_id,
        pid
    )
}
async fn create_opt_python_file(
    fun_content: String,
    db_file_name: String,
    db_url: String,
    study_name: String,
    study_id: i64,
    team_id: TeamId,
    project_id: ProjectId,
) -> Result<String, IdpGlobalError> {
    // create file content
    let load_fun_content = format!(
        "{}(study_name = '{}',storage='{}')",
        HPOPT_PYTHON_LOAD_FUN, study_name, db_url
    );
    let full_file_content = format!(
        "{}\n{}\n{}\n{}\n",
        HPOPT_PYTHON_HEADER, fun_content, load_fun_content, HPOPT_PYTHON_FOOTER
    );
    let opt_run_path =
        business::path_tool::optimize_run_path(team_id, project_id, db_file_name, study_id);

    // create and write content
    let file_path = std::path::Path::new(&opt_run_path);
    // 3. use db_name and study_id create a objective function file.(if parent dir not exist,create it.)
    if !file_path.parent().unwrap().exists() {
        tracing::debug!("file_path.parent().unwrap().not exists()");
        std::fs::create_dir_all(file_path.parent().unwrap())?;
    }
    let mut file = tokio::fs::File::create(&opt_run_path).await?;
    // 4. write objective function content to this file.
    tokio::io::AsyncWriteExt::write_all(&mut file, full_file_content.as_bytes()).await?;
    Ok(opt_run_path)
}
async fn opt_state_monitor(mut child: Child, cache_service: CacheService, opt_state_key: String) {
    info!("fork child process finished,pid:{:#?}", child.id());

    // firstly set clone state as cloning.
    if let Err(err) = cache_service
        .set_optimize_state(&opt_state_key, OptimizeState::Running)
        .await
    {
        error!("{err}");
    }

    match child.wait().await {
        Ok(status) => {
            if status.success() {
                info!("run optimize success");
                //set clone state as success.
                if let Err(err) = cache_service
                    .set_optimize_state(&opt_state_key, OptimizeState::Success)
                    .await
                {
                    error!("{err}");
                }
            } else {
                //set clone state as failed.
                error!("run optimize exit with status:{:?}", status);
                if let Err(err) = cache_service
                    .set_optimize_state(&opt_state_key, OptimizeState::Failed)
                    .await
                {
                    error!("{err}");
                }
            }
        }
        Err(err) => {
            error!("wait run optimize error {:?}", err);
            //set clone state as failed.
            if let Err(err) = cache_service
                .set_optimize_state(&opt_state_key, OptimizeState::Failed)
                .await
            {
                error!("{err}");
            }
        }
    }
}

pub async fn optimize_log(
    team_id: TeamId,
    project_id: ProjectId,
    study_id: i64,
    db_name: String,
) -> Result<Rsp<String>, IdpGlobalError> {
    let opt_run_path =
        business::path_tool::optimize_run_path(team_id, project_id, db_name, study_id);
    let log_path = format!("{}.out", opt_run_path,);
    let log_content = tokio::fs::read_to_string(log_path).await?;
    Ok(Rsp::success(log_content))
}

#[tokio::test]
async fn test_optimize_run_redirect() {
    let python_path = "/Users/huangjin/miniconda3/envs/python39/bin/python";
    let opt_run_path = "/store/19980923/projects/1001/hpopt/run/111.db/8.py";
    let mut cmd = Command::new("bash");
    //get log path
    let log_path = format!("{}.out", opt_run_path,);
    //redirect stdout and stderror to log file.
    let command = format!(
        "{} {} {} 1>{} 2>&1",
        python_path, opt_run_path, "10", log_path
    );
    cmd.arg("-c").arg(command);
    println!("cmd: {:?}", cmd);
    let mut child = cmd.spawn().unwrap();
    let status = child.wait().await.unwrap();
    println!("status: {:?}", status);
}
