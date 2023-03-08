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

use std::collections::HashMap;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Extension;
use axum::Json;
use axum::TypedHeader;
use common_model::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_key;
use common_tools::cookies_tools::Cookies;
use err::ErrorTrace;
use reqwest::header;
pub use uuid::Uuid;

use crate::handler::deploy_prod_service::K8S_SERVICE_API_V2_BASE_URL;
use crate::pojo::equipment::ActivateReq;
use crate::pojo::equipment::EquipDetailQto;
use crate::pojo::equipment::EquipInsertReq;
use crate::pojo::equipment::EquipmentDelReq;
use crate::pojo::equipment::EquipmentInfo;
use crate::pojo::equipment::EquipmentList;
use crate::pojo::equipment::EquipmentListQto;
use crate::pojo::equipment::StatusRsp;
use crate::pojo::equipment::TokenRsp;
use crate::pojo::equipment::UpdateReq;

/*
    if don't need to use status to filter data,
        paging before update,
        call k8s/status api,
        directly update status in database and struct,
    if we need to filter by status
        call k8s/status api,
        update status in database and struct,
        then perform paging operation
*/
pub async fn get_equipment_list(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<EquipmentListQto>,
) -> Result<Rsp<EquipmentList>, ErrorTrace> {
    tracing::info!("[get_equipment_list] run get_equipment_list func");
    //sort by id|createTime|updateTime

    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let sort_field = match req.sort_field {
        Some(data) => {
            if data == "createTime" {
                "create_time"
            } else if data == "updateTime" {
                "update_time"
            } else {
                "id"
            }
        }
        None => "id",
    };
    let sort = req.sort.unwrap_or_else(|| "desc".to_string());
    let current = req.current;
    let skip = req.size * (req.current - 1);
    let size = req.size;
    let search_info = req.search_info.unwrap_or_default();
    let status_flag = req.status.is_some();

    let mut equip_sql = format!(
        r#"
        SELECT 
            id as equipment_id , equipment_name,status,serial_number,memo,create_time,update_time
        FROM 
            idp_model_equipment
        WHERE 
            concat(serial_number,' ',equipment_name,' ',memo) LIKE '%{}%'
            and team_id = $1 
        ORDER BY {} {}
        "#,
        search_info, sort_field, sort
    );
    if !status_flag {
        equip_sql = format!("{equip_sql} OFFSET {skip} LIMIT {size}");
    }
    let equip_model = sqlx::query_as::<_, EquipmentInfo>(&equip_sql)
        .bind(team_id)
        .fetch_all(&pg_pool);
    tracing::debug!("[get_equipment_list]equip_sql->{}", equip_sql);

    let mut total = 0;
    let mut pages;
    let mut equip_info;
    //if we don't need to filter by status,then we get 'total' from database
    if !status_flag {
        let cnt_sql = format!(
            r#"
            SELECT count(*)
            FROM 
                idp_model_equipment 
            WHERE 
                concat(serial_number,' ',equipment_name,' ',memo) LIKE '%{}%' 
                and team_id = $1 
            "#,
            search_info.clone()
        );
        let total_model = sqlx::query_as::<_, (i64,)>(&cnt_sql)
            .bind(team_id)
            .fetch_one(&pg_pool);
        tracing::debug!("[get_equipment_list]cnt_sql->{}", cnt_sql);

        let (total_res, equip_model) = futures::future::join(total_model, equip_model).await;
        total = total_res?.0 as i32;

        equip_info = equip_model?;
    } else {
        //if we need to filter by status,we need to get 'total' after check & update status
        equip_info = equip_model.await?;
    }

    let status_map = get_all_equip_status(&equip_info).await;

    for item in equip_info.iter_mut() {
        if let Some(status) = status_map.get(&item.equipment_id) {
            let status = status.to_string();
            if item.status != status {
                item.status = status;
            }
        }
        item.set_time(&pg_pool, Some(team_id)).await;
    }

    tokio::spawn(update_all_status(status_map.clone(), pg_pool)).await?;

    if status_flag {
        //status format is status1,status2
        let status = req
            .status
            .unwrap()
            .split(',')
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        equip_info = equip_info
            .into_iter()
            .filter(|e| status.contains(&e.status))
            .collect::<Vec<EquipmentInfo>>();
        total = equip_info.len() as i32;
        let take_num = (size * current).try_into().unwrap();
        let skip = (size * (current - 1)).try_into().unwrap();
        equip_info = equip_info
            .into_iter()
            .take(take_num)
            .skip(skip)
            .collect::<Vec<EquipmentInfo>>();
    }

    pages = if total % size == 0 {
        total / size
    } else {
        total / size + 1
    };
    if pages == 0 {
        pages = 1
    }

    Ok(Rsp::success(EquipmentList {
        data: equip_info,
        size,
        current,
        total,
        pages,
    }))
}

pub async fn get_all_equip_status(equip_info: &Vec<EquipmentInfo>) -> HashMap<i32, String> {
    let mut raw_futs = Vec::new();

    for item in equip_info {
        let serial_number = item.serial_number.clone();
        let status = item.status.clone();
        raw_futs
            .push(async move { get_equip_status(item.equipment_id, serial_number, status).await });
    }

    //Error handling for each future
    let unpin_futs: Vec<_> = raw_futs.into_iter().map(Box::pin).collect();
    let mut futs = unpin_futs;

    let mut map: HashMap<i32, String> = HashMap::new();
    while !futs.is_empty() {
        match futures::future::select_all(futs).await {
            (Ok(res), _index, remaining) => {
                map.insert(res.0, res.1);
                futs = remaining;
            }
            (Err(e), index, remaining) => {
                tracing::error!("index:{index}err!,error message: {e}");
                futs = remaining;
            }
        }
    }
    map
}

pub async fn get_equip_status(
    id: i32,
    serial_number: String,
    origin_status: String,
) -> Result<(i32, String), ErrorTrace> {
    //if err ,output error info

    let label = format!("nodeSn={serial_number}");
    let status_json = serde_json::json!({ "label": label });
    let status_url = format!("{K8S_SERVICE_API_V2_BASE_URL}/deviceStatus");
    let client = reqwest::Client::new();
    let rsp = client.post(&status_url).json(&status_json).send().await?;

    tracing::info!("[get_equip_status]rsp -> {:#?}", rsp);
    let content = rsp.text().await?;
    let status_vec = serde_json::from_str::<StatusRsp>(&content)?.data;
    let status = if status_vec.is_empty() {
        origin_status
    } else {
        let flag = &status_vec.get(0).unwrap().status;
        if flag == "True" {
            "online".to_string()
        } else {
            "offline".to_string()
        }
    };
    Ok((id, status))
}

pub async fn update_all_status(status_map: HashMap<i32, String>, pg_pool: sqlx::PgPool) {
    let mut futs = Vec::new();

    for (id, status) in status_map {
        futs.push(update_status(&pg_pool, id, status));
    }

    if let Err(error) = futures::future::try_join_all(futs).await {
        tracing::error!("err! error message : {:#?}", error);
    };
}

pub async fn update_status(
    pg_pool: &sqlx::PgPool,
    id: i32,
    status: String,
) -> Result<(), ErrorTrace> {
    let update_sql = r#"
        UPDATE idp_model_equipment SET status = $1 where id = $2"#
        .to_string();
    if let Err(err) = sqlx::query(&update_sql)
        .bind(status)
        .bind(id)
        .execute(pg_pool)
        .await
    {
        tracing::error!("err! error message : {:#?}", err);
    };
    Ok(())
}

/*
    change status to withdraw,call deviceDelete API
    TODO :delete
*/
pub async fn delete_equipment(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<EquipmentDelReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::debug!("[delete_equipment] run delete_equipment func");

    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let sql = r#"
        UPDATE idp_model_equipment
        SET status = 'withdraw'
        WHERE id = $1 and team_id = $2
        RETURNING serial_number"#;
    let serial_number = sqlx::query_as::<_, (String,)>(sql)
        .bind(req.equipment_id)
        .bind(team_id)
        .fetch_one(&pg_pool)
        .await?
        .0;

    let label = format!("nodeSn={serial_number}");
    let status_json = serde_json::json!({ "label": label });
    let cancel_url = format!("{K8S_SERVICE_API_V2_BASE_URL}/deviceDelete");
    let client = reqwest::Client::new();
    let rsp = client.post(&cancel_url).json(&status_json).send().await?;
    tracing::info!("[delete_equipment]rsp -> {:#?}", rsp);

    Ok(Rsp::success_without_data())
}

/*
        initial status : unactivated
        unactivated|online|offline|withdraw

        serial_number is gengerated by uuid
*/
pub async fn insert_equipment(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<EquipInsertReq>,
) -> Result<Rsp<i32>, ErrorTrace> {
    tracing::debug!("[insert_equipment] run insert_equipment func");

    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();

    let memo = req.memo.unwrap_or_default();
    let serial_number = Uuid::new_v4().to_string();
    let status = "unactivated";

    let sql = r#"
        INSERT into idp_model_equipment
        (team_id,creator_id,equipment_name,status,serial_number,memo)
        VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING id"#;
    let id = sqlx::query_as::<_, (i32,)>(sql)
        .bind(team_id)
        .bind(user_id)
        .bind(req.equipment_name)
        .bind(status)
        .bind(serial_number)
        .bind(memo)
        .fetch_one(&pg_pool)
        .await?
        .0;

    Ok(Rsp::success(id))
}

pub async fn activate_equipment(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ActivateReq>,
) -> Result<Rsp<String>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let sql = r#"
        SELECT 
            serial_number 
        FROM 
            idp_model_equipment
        WHERE
            id = $1 and team_id = $2 and status = 'unactivated'
    "#;
    let _serial_number = match sqlx::query_as::<_, (String,)>(sql)
        .bind(req.equipment_id)
        .bind(team_id)
        .fetch_one(&pg_pool)
        .await
    {
        Ok(data) => data.0,
        Err(_) => return Err(ErrorTrace::new("no permission")),
    };

    let token_url = format!("{K8S_SERVICE_API_V2_BASE_URL}/getToken");
    let content = serde_json::json!({ "namespace": "kubeedge" });
    let client = reqwest::Client::new();
    let rsp = client.post(&token_url).json(&content).send().await?;
    tracing::info!("[activate_equipment]rsp -> {:#?}", rsp);

    let value = rsp.text().await?;
    let token_model = serde_json::from_str::<TokenRsp>(&value)?;
    let token = token_model.data.into_iter().next().unwrap_or_default();

    Ok(Rsp::success(token))
}

pub async fn equipment_detail(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<EquipDetailQto>,
) -> Result<Rsp<EquipmentInfo>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let select_sql = r#"
    SELECT
        id as equipment_id , equipment_name,status,serial_number,memo,create_time,update_time
    FROM 
        idp_model_equipment
    WHERE 
        id = $1 and team_id = $2
        "#;
    let mut equipment_info = sqlx::query_as::<_, EquipmentInfo>(select_sql)
        .bind(req.equip_id)
        .bind(team_id)
        .fetch_one(&pg_pool)
        .await?;
    equipment_info.set_time(&pg_pool, Some(team_id)).await;

    Ok(Rsp::success(equipment_info))
}

pub async fn equipment_update(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<UpdateReq>,
) -> Result<Rsp<i32>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    if let Some(equip_name) = req.equipment_name {
        sqlx::query(
            r#"
        UPDATE idp_model_equipment SET equipment_name = $1 WHERE id = $2 and team_id = $3"#,
        )
        .bind(equip_name)
        .bind(req.equipment_id)
        .bind(team_id)
        .execute(&pg_pool)
        .await?;
    }

    if let Some(memo) = req.memo {
        sqlx::query(
            r#"
        UPDATE idp_model_equipment SET memo = $1 WHERE id = $2 and team_id = $3"#,
        )
        .bind(memo)
        .bind(req.equipment_id)
        .bind(team_id)
        .execute(&pg_pool)
        .await?;
    }

    Ok(Rsp::success(req.equipment_id))
}

pub async fn equipment_csv(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<EquipmentListQto>,
) -> Result<impl IntoResponse, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let sort_field = match req.sort_field {
        Some(data) => {
            if data == "createTime" {
                "create_time"
            } else if data == "updateTime" {
                "update_time"
            } else {
                "id"
            }
        }
        None => "id",
    };
    let sort = req.sort.unwrap_or_else(|| "desc".to_string());
    let search_info = req.search_info.unwrap_or_default();
    let status_flag = req.status.is_some();

    let equip_sql = format!(
        r#"
        SELECT 
            id as equipment_id , equipment_name,status,serial_number,memo,create_time,update_time
        FROM 
            idp_model_equipment
        WHERE 
            concat(serial_number,' ',equipment_name,' ',memo) LIKE '%{}%'
            and team_id = $1 
        ORDER BY {} {}
        "#,
        search_info, sort_field, sort
    );

    let equip_model = sqlx::query_as::<_, EquipmentInfo>(&equip_sql)
        .bind(team_id)
        .fetch_all(&pg_pool);
    tracing::debug!("[equipment_csv]equip_sql->{}", equip_sql);

    let mut equip_info = equip_model.await?;

    let status_map = get_all_equip_status(&equip_info).await;

    for item in equip_info.iter_mut() {
        if let Some(status) = status_map.get(&item.equipment_id) {
            let status = status.to_string();
            if item.status != status {
                item.status = status;
            }
        }
        item.set_time(&pg_pool, Some(team_id)).await;
    }

    tokio::spawn(update_all_status(status_map.clone(), pg_pool)).await?;

    if status_flag {
        //status format is status1,status2
        let status = req
            .status
            .unwrap()
            .split(',')
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        equip_info = equip_info
            .into_iter()
            .filter(|e| status.contains(&e.status))
            .collect::<Vec<EquipmentInfo>>();
    }

    let tmp_file = format!("/tmp/{}_{}.csv", team_id, chrono::Utc::now().timestamp());
    let mut wtr = csv::Writer::from_path(&tmp_file).unwrap();

    equip_info.into_iter().for_each(|item| {
        wtr.serialize(item).unwrap();
    });

    wtr.flush().unwrap();

    let file = tokio::fs::File::open(&tmp_file).await?;

    let stream = tokio_util::io::ReaderStream::new(file);

    let attachment_str = "attachment; filename=\"equipment.csv\"".to_string();
    let body = axum::body::StreamBody::new(stream);
    let mut resp = axum::response::IntoResponse::into_response(body);
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream;charset=UTF-8"),
    );
    resp.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment_str).unwrap(),
    );

    Ok(resp)
}
