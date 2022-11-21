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

use std::io::Error;

use business::business_term::ProjectId;
use business::business_term::TeamId;
use business::path_tool;

pub async fn get_study_objective_code(
    team_id: TeamId,
    project_id: ProjectId,
    study_id: i64,
    db_name: String,
) -> Result<(String, String), Error> {
    let fun_file_path =
        path_tool::get_study_objective_fun_path(team_id, project_id, &db_name, study_id);
    tracing::debug!("fun_file_path: {}", fun_file_path);
    //read file content and return.
    let content = tokio::fs::read_to_string(&fun_file_path).await?;
    Ok((fun_file_path, content))
}
pub async fn edit_study_objective_code(file_path: String, content: String) -> Result<(), Error> {
    tracing::debug!("fun_file_path: {}", file_path);
    //rewrun file content and return.
    tokio::fs::write(&file_path, content).await?;
    Ok(())
}

// pub async fn list_study(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }
// pub async fn new_study(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }
// pub async fn delete_study(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }

// pub async fn study_detail(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }

// ///
// /// about optimize
// ///
// pub async fn study_optimize_run(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }
// pub async fn study_optimize_status(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }
