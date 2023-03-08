// Copyright 2023 BaihaiAI, Inc.
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
use axum::TypedHeader;
use chrono::NaiveDateTime;
use common_model::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_key;
use common_tools::cookies_tools::Cookies;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;

use crate::handler::deploy_prod_service::get_service_detail_by_id;

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/2/28
 * Time: 16:15
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct HistoryInfo {
    pub id: i32,
    pub service_id: i32,
    pub task_type: String,
    pub service_name: String,

    #[serde(serialize_with = "serde_helper::ser_i64_to_string")]
    pub team_id: i64,
    #[serde(serialize_with = "serde_helper::ser_i64_to_string")]
    pub user_id: i64,
    pub user_name: String,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryInfoListRsp {
    pub history_info: Vec<HistoryInfo>,
    pub pages: i32,
    pub total: i32,
    pub size: i32,
    pub current: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryListReq {
    pub page_size: i32,
    pub page_index: i32,
    pub service_id: i32,
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub sort_field: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryInsertReq {
    pub service_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryUpdateReq {
    pub id: i32,
    pub status: String,
}

pub async fn service_task_history_list_handler(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(params): Query<HistoryListReq>,
) -> Result<Rsp<HistoryInfoListRsp>, ErrorTrace> {
    tracing::debug!("run service_task_history_list ");

    let size = params.page_size;
    let current = params.page_index;
    let service_id = params.service_id;

    let keyword = if params.keyword.is_some() {
        params.keyword.unwrap_or("".to_string())
    } else {
        "".to_string()
    };

    let status = if params.status.is_some() {
        params.status.unwrap_or("".to_string())
    } else {
        "".to_string()
    };

    let sort_field = if params.sort_field.is_some() {
        params.sort_field.unwrap_or("create_time ".to_string())
    } else {
        "create_time ".to_string()
    };

    let sort_order = if params.sort.is_some() {
        params.sort.unwrap_or(" DESC".to_string())
    } else {
        " DESC".to_string()
    };

    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    tracing::info!("team_id={team_id}");

    let resp = service_task_history_list(
        &pg_pool, size, current, team_id, service_id, keyword, status, sort_field, sort_order,
    )
    .await?;

    Ok(Rsp::success(resp))
}

pub fn is_number(para_str: &str) -> bool {
    let mut flag = true;
    for c in para_str.chars() {
        if c.is_alphabetic() {
            flag = false
        } else if c.is_ascii_digit() {
        }
    }
    flag
}

pub async fn service_task_history_list(
    pg_pool: &sqlx::PgPool,
    size: i32,
    current: i32,
    team_id: i64,
    service_id: i32,
    keyword: String,
    status: String,
    sort_field: String,
    sort_order: String,
) -> Result<HistoryInfoListRsp, ErrorTrace> {
    let mut count_sql = "".to_string();
    let base_sql = r#"
            SELECT  count(history.id)
            FROM    idp_model_service_task_history as history
            INNER   JOIN idp_model_deploy_prod_service as service ON service.id = history.service_id
            WHERE   history.team_id = $1 and  history.service_id = $2
    "#;

    count_sql += base_sql;

    if !keyword.is_empty() {
        if is_number(keyword.as_str()) {
            count_sql += " and history.id = ";
            count_sql += &keyword.to_string()
        } else {
            count_sql += " and service.service_name like '%";
            count_sql += keyword.as_str();
            count_sql += "%'"
        }
    }

    if !status.is_empty() {
        count_sql += " and history.status ='";
        count_sql += status.as_str();
        count_sql += "'"
    }

    tracing::info!("##service_task_history_list count_sql={}", count_sql);

    let total = sqlx::query_as::<_, (i64,)>(count_sql.as_str())
        .bind(team_id)
        .bind(service_id)
        .fetch_one(pg_pool)
        .await?
        .0;
    let total = total as i32;
    let pages = if total % size == 0 {
        total / size
    } else {
        total / size + 1
    };

    let skip = size * (current - 1);

    let mut service_sql = "".to_string();
    let service_base_sql = "SELECT history.id, history.service_id, history.task_type, history.team_id, history.user_id, history.user_name,
    history.status, history.start_time, history.end_time, history.create_time,service.service_name
    FROM    idp_model_service_task_history history
    INNER JOIN idp_model_deploy_prod_service as service ON service.id = history.service_id
    WHERE   service.team_id = $1  and history.service_id = $2 ";

    service_sql += service_base_sql;
    if !keyword.is_empty() {
        if is_number(keyword.as_str()) {
            service_sql += " and history.id = ";
            service_sql += &keyword.to_string();
        } else {
            service_sql += " and service.service_name like '%";
            service_sql += keyword.as_str();
            service_sql += "%' "
        }
    }

    if !status.is_empty() {
        service_sql += " and history.status ='";
        service_sql += status.as_str();
        service_sql += "'"
    }

    service_sql += " ORDER BY service.";
    service_sql += sort_field.as_str();
    service_sql += " "; //it is important
    service_sql += sort_order.as_str();
    service_sql += "  OFFSET  $3  LIMIT  $4 ";

    tracing::info!("##service_task_history_list service_sql={}", service_sql);

    let history_info: Vec<HistoryInfo> = sqlx::query_as(&service_sql)
        .bind(team_id)
        .bind(service_id)
        .bind(skip)
        .bind(size)
        .fetch_all(pg_pool)
        .await?;

    // for item in service_info.iter_mut() {
    //     item.set_time(pg_pool, Some(team_id)).await;
    // }

    let history_list = HistoryInfoListRsp {
        history_info,
        size,
        current,
        total,
        pages,
    };

    Ok(history_list)
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: i64,
    pub username: String,
}

pub async fn get_user_info_by_user_id(
    pg_pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<UserInfo, ErrorTrace> {
    let service_sql = "select user_id,username from user_info where user_id = $1";
    let team_info: UserInfo = sqlx::query_as(service_sql)
        .bind(user_id)
        .fetch_one(pg_pool)
        .await?;
    Ok(team_info)
}

pub async fn insert_service_task_history(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<HistoryInsertReq>,
) -> Result<Rsp<i32>, ErrorTrace> {
    tracing::info!("run insert_service_task_history start");

    let task_type = "Schedule";
    let status = "Running";
    let service_id = req.service_id;

    let service_info = get_service_detail_by_id(&pg_pool, service_id).await?;
    tracing::info!("!!service_info={:?}", service_info);

    let team_id = service_info.team_id;
    let user_id = service_info.user_id;

    let user_info = get_user_info_by_user_id(&pg_pool, user_id).await?;
    let user_name = user_info.username;

    // insert into idp_model_service_task_history()
    // values(1000000005,'Schedule',1629169786696372224,1629169786696372224,'hr的名字','SUCCESS');

    let sql = r#"
        INSERT into idp_model_service_task_history
        (service_id,task_type,team_id,user_id,user_name,status)
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING id"#;
    let id = sqlx::query_as::<_, (i32,)>(sql)
        .bind(service_id)
        .bind(task_type)
        .bind(team_id)
        .bind(user_id)
        .bind(user_name)
        .bind(status)
        .fetch_one(&pg_pool)
        .await?
        .0;
    tracing::info!("run insert_service_task_history end");
    Ok(Rsp::success(id))
}

pub async fn update_service_task_history_handler(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<HistoryUpdateReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let history_task_id = req.id;
    let status = req.status;

    update_service_task_history(&pg_pool, history_task_id, status).await
}

pub async fn update_service_task_history(
    pg_pool: &sqlx::PgPool,
    history_task_id: i32,
    status: String,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("update_service_task_history start");
    sqlx::query(
        "
        UPDATE  idp_model_service_task_history
        SET     status=$1
        WHERE id = $2
        ",
    )
    .bind(status.to_string())
    .bind(history_task_id)
    .execute(pg_pool)
    .await?;
    tracing::info!("update_service_task_history end");
    Ok(Rsp::success_without_data())
}
