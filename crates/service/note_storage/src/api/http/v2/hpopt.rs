use axum::extract::Query;
use business::path_tool;
use common_model::Rsp;
use common_tools::cookies_tools;

use crate::api_model::hpopt::StartHpOpt;
use crate::common::error::IdpGlobalError;
use crate::handler::hpopt;

pub async fn start_hpopt_backend(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(start_hpopt): Query<StartHpOpt>,
) -> Result<Rsp<()>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    match start_hpopt.db_name {
        Some(db_file_name) => {
            let db_file_fullpath =
            business::path_tool::get_hpopt_db_fullpath(team_id, start_hpopt.project_id, &db_file_name);
            //if exsits,
            if std::path::Path::new(&db_file_fullpath).exists() {
                let db_url = hpopt::control::get_dburl_by_db_file_name(team_id, start_hpopt.project_id, &db_file_name);

                //start hpopt
                hpopt::control::start_hpopt_backend(db_url, team_id, start_hpopt.project_id).await
            } else {
                Err(IdpGlobalError::NoteError("db file not exist".to_string()))
            }
        }
        None => {
            //generate db_url with random number.

            //create a random number use timestamp.
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // dbfile name is "idp_hpopt_{timestamp}.db"
            let db_file_name = format!("idp_hpopt_{}.db", timestamp);

            let db_url = hpopt::control::get_dburl_by_db_file_name(team_id, start_hpopt.project_id, &db_file_name);

            hpopt::control::start_hpopt_backend(db_url, team_id, start_hpopt.project_id).await
        }
    }
}


