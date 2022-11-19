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
    //read file content and return.
    let content = tokio::fs::read_to_string(&fun_file_path).await?;
    Ok((fun_file_path, content))
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
