use axum::extract::Query;
use axum::Json;
use common_model::Rsp;
use common_tools::cookies_tools;

use crate::api_model::hpopt::DatasourceListReq;
use crate::api_model::hpopt::DatasourceNewReq;
use crate::api_model::hpopt::StartHpOptReq;
use crate::api_model::hpopt::StopHpOptReq;
use crate::common::error::IdpGlobalError;
use crate::handler::hpopt;
use crate::handler::hpopt::control::get_dburl_by_db_file_name;

/// -----------------------------------
/// datasource
/// -----------------------------------

pub async fn datasource_list(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(datasource_list): Query<DatasourceListReq>,
) -> Result<Rsp<Vec<String>>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    //TODO need change code, msg
    Ok(Rsp::success(
        hpopt::datasource::get_datasource_list(team_id, datasource_list.project_id).await?,
    ))
}

pub async fn datasource_new(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(datasource_new): Json<DatasourceNewReq>,
) -> Result<Rsp<String>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    match hpopt::datasource::datasource_new(
        team_id,
        datasource_new.project_id,
        datasource_new.db_name,
    )
    .await
    {
        Ok(db_file_name) => Ok(Rsp::success(db_file_name)),
        Err(e) => Err(e),
    }
    //TODO need change code, msg
}

/// -----------------------------------
/// hpopt bacaned control
/// -----------------------------------
pub async fn start_hpopt_backend(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(start_hpopt): Query<StartHpOptReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    match start_hpopt.db_name {
        Some(db_file_name) => {
            let db_file_fullpath = business::path_tool::get_hpopt_db_fullpath(
                team_id,
                start_hpopt.project_id,
                &db_file_name,
            );
            //if exsits,
            if std::path::Path::new(&db_file_fullpath).exists() {
                let db_url = hpopt::control::get_dburl_by_db_file_name(
                    team_id,
                    start_hpopt.project_id,
                    &db_file_name,
                );

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

            let db_url = hpopt::control::get_dburl_by_db_file_name(
                team_id,
                start_hpopt.project_id,
                &db_file_name,
            );

            hpopt::control::start_hpopt_backend(db_url, team_id, start_hpopt.project_id).await
        }
    }
}

pub async fn stop_hpopt_backend(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(stop_hpopt_req): Query<StopHpOptReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);
    let db_url =
        get_dburl_by_db_file_name(team_id, stop_hpopt_req.project_id, &stop_hpopt_req.db_name);
    hpopt::control::stop_hpopt_backend(db_url).await?;
    Ok(Rsp::success_without_data())
}
