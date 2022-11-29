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
use axum::Extension;
use axum::Json;
use common_model::Rsp;
use common_tools::cookies_tools;
use serde_json::json;
use serde_json::Value;

use crate::api_model::hpopt::DatasourceListReq;
use crate::api_model::hpopt::DatasourceNewReq;
use crate::api_model::hpopt::DatasourceResp;
use crate::api_model::hpopt::EditStudyCodeReq;
use crate::api_model::hpopt::OptRunReq;
use crate::api_model::hpopt::OptStateReq;
use crate::api_model::hpopt::StartHpOptReq;
use crate::api_model::hpopt::StopHpOptReq;
use crate::api_model::hpopt::StudyDetailReq;
use crate::api_model::hpopt::StudyNewReq;
use crate::api_model::hpopt::StudyObjectiveCodeReq;
use crate::api_model::hpopt::StudyObjectiveCodeResp;
use crate::app_context::AppContext;
use crate::common::error::IdpGlobalError;
use crate::handler::hpopt;
use crate::handler::hpopt::control::get_dburl_by_db_file_name;
pub type Cookies = axum::headers::Cookie;

/// -----------------------------------
/// datasource
/// -----------------------------------

pub async fn datasource_list(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(datasource_list): Query<DatasourceListReq>,
) -> Result<Rsp<Vec<DatasourceResp>>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    //TODO need change code, msg
    Ok(Rsp::success(
        hpopt::datasource::get_datasource_status_list(team_id, datasource_list.project_id).await?,
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
/// hpopt backend control
/// -----------------------------------
pub async fn start_hpopt_backend(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(start_hpopt): Query<StartHpOptReq>,
) -> Result<Rsp<u16>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    match start_hpopt.db_name {
        Some(db_file_name) => {
            let db_file_fullpath = business::path_tool::get_hpopt_db_fullpath(
                team_id,
                start_hpopt.project_id,
                &db_file_name,
            );
            //if exist,
            if std::path::Path::new(&db_file_fullpath).exists() {
                let db_url = hpopt::control::get_dburl_by_db_file_name(
                    team_id,
                    start_hpopt.project_id,
                    &db_file_name,
                );

                //start hpopt
                match hpopt::control::start_hpopt_backend(db_url, team_id, start_hpopt.project_id)
                    .await
                {
                    Ok(port) => {
                        // todo! change cookie hpoptPort value.
                        Ok(Rsp::success(port))
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(IdpGlobalError::NoteError("db file not exist".to_string()))
            }
        }
        None => {
            //generate db_url with random number.  deprecated

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

            match hpopt::control::start_hpopt_backend(db_url, team_id, start_hpopt.project_id).await
            {
                Ok(port) => Ok(Rsp::success(port)),
                Err(e) => Err(e),
            }
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
    // todo! clean cookie hpoptPort value.
    Ok(Rsp::success_without_data())
}

pub async fn backend_state(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
) -> Result<Rsp<String>, IdpGlobalError> {
    let port = get_hpopt_port_by_cookie(&cookies);

    let ip_addr = format!("http://127.0.0.1:{}", port);
    let url = format!("{}/api/studies", &ip_addr);

    if let Err(_e) = reqwest::get(&url).await {
        return Ok(Rsp::success("unready".to_string()));
    }
    Ok(Rsp::success("ready".to_string()))
}

///
/// about study
///
pub async fn list_study(
    // team_id: TeamId,
    // project_id: ProjectId,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    // datasource_name: String,
) -> Result<Json<Value>, IdpGlobalError> {
    let port = get_hpopt_port_by_cookie(&cookies);

    let ip_addr = format!("http://127.0.0.1:{}", port);
    let url = format!("{}/api/studies", &ip_addr);

    let resp = reqwest::get(&url).await?.json::<Value>().await?;

    Ok(Json(resp))
}
fn get_hpopt_port_by_cookie(cookies: &Cookies) -> u16 {
    let hpopt_port_str = cookies_tools::get_cookie_value_by_key(cookies, "hpoptPort");

    match hpopt_port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_e) => 0,
    }
}

// not contains objective fun content.
pub async fn study_detail(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(study_detail_req): Query<StudyDetailReq>,
) -> Result<Json<Value>, IdpGlobalError> {
    tracing::debug!("study_detail_req:{:?}", study_detail_req);
    let port = get_hpopt_port_by_cookie(&cookies);
    let ip_addr = format!("http://127.0.0.1:{}", port);
    let url = format!("{}/api/studies/{}", &ip_addr, study_detail_req.study_id);

    let resp = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    Ok(Json(resp))
}

pub async fn study_objective_code(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Query(study_obj_code_req): Query<StudyObjectiveCodeReq>,
) -> Result<Rsp<StudyObjectiveCodeResp>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);
    match hpopt::study::get_study_objective_code(
        team_id,
        study_obj_code_req.project_id,
        study_obj_code_req.study_id,
        study_obj_code_req.db_name,
    )
    .await
    {
        Ok((full_path, code)) => Ok(Rsp::success(StudyObjectiveCodeResp {
            full_file_path: full_path,
            objective_content: code,
        })),
        Err(e) => Err(IdpGlobalError::NoteError(e.to_string())),
    }
}

// todo: need optimize.
pub async fn study_new(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(study_new_req): Json<StudyNewReq>,
) -> Result<Json<Value>, IdpGlobalError> {
    let port = get_hpopt_port_by_cookie(&cookies);
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);
    // todo need change get port from redis(will save it when start_hpopt_backend).

    let db_name = study_new_req.db_name.clone();
    let ip_addr = format!("http://127.0.0.1:{}", port);
    let url = format!("{}/api/studies", &ip_addr);

    let new_study_req_json = json!(
        {
            "study_name": study_new_req.study_name,
            "directions": study_new_req.directions,

        }
    );
    tracing::debug!("new_study_req_json:{:?}", new_study_req_json);
    // 1. request to hpopt backend to create a new study.
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&new_study_req_json)
        .send()
        .await?
        .json::<Value>()
        .await?;
    tracing::debug!("study_new resp:{:?}", resp);
    // 2. get study_id from response.
    let study_id = match resp["study_summary"]["study_id"].as_i64() {
        Some(study_id) => study_id,
        None => return Ok(Json(resp)),
    };

    // 3. use teamid,project_id,db_name,study_id get file_path.
    let fun_file_path = business::path_tool::get_study_objective_fun_path(
        team_id,
        study_new_req.project_id,
        &db_name,
        study_id,
    );
    tracing::debug!("fun_file_path:{:?}", fun_file_path);

    let file_path = std::path::Path::new(&fun_file_path);
    // 3. use db_name and study_id create a objective function file.(if parent dir not exist,create it.)
    if !file_path.parent().unwrap().exists() {
        tracing::debug!("file_path.parent().unwrap().not exists()");
        std::fs::create_dir_all(file_path.parent().unwrap())?;
    }
    let mut file = tokio::fs::File::create(fun_file_path).await?;
    // 4. write objective function content to this file.
    tokio::io::AsyncWriteExt::write_all(&mut file, study_new_req.objective_content.as_bytes())
        .await?;
    // 5. return the response of hpopt backend to frontend.
    Ok(Json(resp))
    // Ok(Rsp::success_without_data())
}

// pub async fn delete_study(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }

// ///
// /// about optimize
// ///
pub async fn objective_example_names() -> Result<Rsp<Vec<String>>, IdpGlobalError> {
    //TODO need change code, msg
    Ok(Rsp::success(
        hpopt::optimize::get_optimize_objective_example_names().await?,
    ))
}
pub async fn objective_code_content(
    Query(req): Query<crate::api_model::hpopt::ObjectiveContentReq>,
) -> Result<Rsp<String>, IdpGlobalError> {
    Ok(Rsp::success(
        hpopt::optimize::get_optimize_objective_code_content(req.name).await?,
    ))
}
pub async fn edit_study_objective_code(
    Json(edit_study_code_req): Json<EditStudyCodeReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    hpopt::study::edit_study_objective_code(edit_study_code_req.path, edit_study_code_req.content)
        .await?;
    Ok(Rsp::success_without_data())
}

pub async fn study_optimize_run(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Extension(app_context): Extension<AppContext>,
    Json(opt_run_req): Json<OptRunReq>,
) -> Result<Rsp<String>, IdpGlobalError> {
    let team_id = cookies_tools::get_cookie_value_by_team_id(cookies);

    let opt_state_key = hpopt::optimize::study_optimize_run(
        team_id,
        opt_run_req.project_id,
        opt_run_req.study_id,
        opt_run_req.study_name,
        opt_run_req.db_name,
        opt_run_req.n_trials,
        &app_context.redis_cache,
    )
    .await?;
    Ok(Rsp::success(opt_state_key))
}
// pub async fn study_optimize_stop(
//     team_id: TeamId,
//     project_id: ProjectId,
//     datasource_name: String,
// ) -> Result<String, IdpGlobalError> {

//     //todo!
// }
pub async fn optimize_state(
    Query(opt_state_req): Query<OptStateReq>,
    Extension(app_context): Extension<AppContext>,
) -> Result<Rsp<Option<String>>, IdpGlobalError> {
    tracing::info!("query opt state...");
    //get clone state form redis
    Ok(Rsp::success(
        app_context
            .redis_cache
            .get_optimize_state(&opt_state_req.opt_state_key)
            .await?,
    ))
}

// backend-status
//
