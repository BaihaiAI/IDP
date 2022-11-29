use business::business_term::ProjectId;
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
use business::business_term::TeamId;

use super::control;
use super::control::get_dburl_by_db_file_name;
use super::control::get_pid_by_name;
use crate::api_model::hpopt::DatasourceResp;
use crate::common::error::IdpGlobalError;
use crate::status_code;

// Splicing db_file_name, start through dashboard, specify this file, will automatically create the corresponding sqlite database
pub async fn datasource_new(
    team_id: TeamId,
    project_id: ProjectId,
    datasource_name: String,
) -> Result<String, IdpGlobalError> {
    let db_file_name = format!("idp_{}.db", datasource_name);
    let datasource_list = get_datasource_list(team_id, project_id)?;
    // if exists the same name, return error
    if datasource_list.contains(&db_file_name) {
        //TODO change status code
        return Err(IdpGlobalError::ErrorCodeMsg(
            status_code::HPOPT_CREATE_DB_EXISTS_CODE,
            status_code::HPOPT_CREATE_DB_EXISTS_MSG.to_string(),
        ));
    }
    let db_file_fullpath =
        business::path_tool::get_hpopt_db_fullpath(team_id, project_id, &db_file_name);
    let db_url = control::get_dburl_by_db_file_name(team_id, project_id, &db_file_name);

    match control::start_hpopt_backend(db_url.clone(), team_id, project_id).await {
        Ok(_) => {
            // if start success, shutdown backend and return db_file_name(we just need create db schema via start backend).
            // need to wait some time, otherwise the backend will not create the database file successfully
            // sleep(std::time::Duration::from_secs(1));

            // wait db_file create success,after that shutdown backend(need set timeout 3 seconds)
            let full_path = std::path::Path::new(&db_file_fullpath);
            let mut count = 0;
            loop {
                if full_path.exists() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                count += 1;
                if count > 3 {
                    return Err(IdpGlobalError::ErrorCodeMsg(
                        status_code::HPOPT_CREATE_DB_TIMEOUT_CODE,
                        status_code::HPOPT_CREATE_DB_TIMEOUT_MSG.to_string(),
                    ));
                }
            }
            control::stop_hpopt_backend(db_url).await?;

            Ok(db_file_name)
        }
        Err(e) => Err(e),
    }
}

pub fn get_datasource_list(
    team_id: TeamId,
    project_id: ProjectId,
) -> Result<Vec<String>, std::io::Error> {
    // get datasource dir path
    let datasource_path = business::path_tool::get_hpopt_datasource_path(team_id, project_id);
    // create file struct by path and get all file name.
    let mut datasource_list = Vec::new();
    //TODO: list response add status.
    if let Ok(dir) = std::fs::read_dir(datasource_path.clone()) {
        dir.for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            datasource_list.push(file_name.to_string());
                        }
                    }
                }
            }
        });
    } else {
        // this dir not exist, create it.
        // print log on console. todo:need change to log crate.
        tracing::info!(
            "datasource dir not exist, create it. path: {}",
            datasource_path
        );
        // log::info!("datasource dir not exist, create it. path: {}",datasource_path);
        std::fs::create_dir_all(datasource_path)?;
    }
    Ok(datasource_list)
}
// pub async fn delete_datasource(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<(), IdpGlobalError> {
//     //todo!
//     Ok(())
// }
///
/// /store/{team_id}/projects/project_id/hp[opt_datasource]
pub async fn get_datasource_status_list(
    team_id: TeamId,
    project_id: ProjectId,
) -> Result<Vec<DatasourceResp>, std::io::Error> {
    let datasource_list = get_datasource_list(team_id, project_id)?;

    let resp = futures::future::join_all(
        datasource_list
            .iter()
            .map(|db_file_name| async move {
                let db_url = get_dburl_by_db_file_name(team_id, project_id, db_file_name);
                let pid = get_pid_by_name(&db_url).await;
                match pid {
                    Ok(pid) => DatasourceResp {
                        name: db_file_name.clone(),
                        status: "running".to_string(),
                        port: get_backend_port_by_pid(pid).await,
                    },
                    Err(_e) => DatasourceResp {
                        name: db_file_name.clone(),
                        status: "stopped".to_string(),
                        port: None,
                    },
                }
            })
            .collect::<Vec<_>>(),
    )
    .await;

    Ok(resp)
}
async fn get_backend_port_by_pid(pid: u32) -> Option<u16> {
    let output = tokio::process::Command::new("ps").arg("-ef").output().await;
    if let Err(e) = output {
        tracing::error!("get backend port by pid error: {}", e);
        return None;
    };

    let output_str = String::from_utf8(output.unwrap().stdout).unwrap();
    let lines = output_str.lines();
    for line in lines {
        if line.contains(pid.to_string().as_str()) {
            let split = line.split_whitespace();
            //port is last nth element
            let port = split.last().unwrap();

            return Some(port.parse::<u16>().unwrap());
        }
    }

    None
}
#[cfg(not)]
#[tokio::test]
async fn test_datasource_list() {
    let team_id = 19980923;
    let project_id = 1001;
    let datasource_list = get_datasource_list(team_id, project_id);
    println!("{:?}", datasource_list);
}
