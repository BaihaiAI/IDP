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

use super::*;

const BASE_SQL: &str = "
SELECT  service.create_time, service.cron_config, service.edition_id,edition.testing_input,
        edition.version as edition_version, service.id, service.instance_count,
        null as last_run_time, service.package_id, service.team_id, service.user_id,
        package.intro as package_intro, package.model_name as package_name,
        service.pod_cpu, service.pod_gpu, service.pod_memory,
        service.intro as service_intro, service.service_name,
        service.status, service.status_message, service.update_time, service.equipments,
        service.image, service.url,null as id_token, service.env, service.service_type,
        service.support_system, service.support_chip,
        edition.input_types, edition.output_types
FROM    idp_model_deploy_prod_service as service
INNER JOIN  idp_model_edition as edition 
ON      service.edition_id = edition.id
INNER JOIN  idp_model_package as package 
ON      service.package_id = package.id";

pub async fn get_service_list(
    pg_pool: &sqlx::PgPool,
    size: i32,
    current: i32,
    team_id: i64,
    service_type: i32,
) -> Result<ServiceListRsp, ErrorTrace> {
    let count_sql = r#"
    SELECT  count(*) 
    FROM    idp_model_deploy_prod_service as service
    WHERE   team_id = $1 and service_type = $2
        "#;
    let total = sqlx::query_as::<_, (i64,)>(count_sql)
        .bind(team_id)
        .bind(service_type)
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
    //get service_list
    //todo last_run_time
    let service_sql = BASE_SQL.to_string()
        + "
    WHERE   service.team_id = $1 and service.service_type = $2 and service.is_deleted = false
    ORDER BY service.create_time DESC
    OFFSET  $3
    LIMIT   $4
    ";
    let mut service_info: Vec<ServiceInfo> = sqlx::query_as(&service_sql)
        .bind(team_id)
        .bind(service_type)
        .bind(skip)
        .bind(size)
        .fetch_all(pg_pool)
        .await?;

    for item in service_info.iter_mut() {
        item.set_time(pg_pool, Some(team_id)).await;
    }

    let service_list = ServiceListRsp {
        service_info,
        size,
        current,
        total,
        pages,
    };
    Ok(service_list)
}

pub async fn get_service_detail(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    id_token: Option<String>,
    service_id: i32,
) -> Result<ServiceInfo, ErrorTrace> {
    let service_sql = BASE_SQL.to_string()
        + "
    WHERE   service.id = $1 and service.team_id = $2
    ";
    let mut service_info: ServiceInfo = sqlx::query_as(&service_sql)
        .bind(service_id)
        .bind(team_id)
        .fetch_one(pg_pool)
        .await?;
    service_info.set_time(pg_pool, Some(team_id)).await;
    service_info.set_some_field(id_token).await;

    Ok(service_info)
}

pub async fn get_service_detail_by_id(
    pg_pool: &sqlx::PgPool,
    service_id: i32,
) -> Result<ServiceInfo, ErrorTrace> {
    let service_sql = BASE_SQL.to_string()
        + "
            WHERE   service.id = $1
    ";
    let service_info: ServiceInfo = sqlx::query_as(&service_sql)
        .bind(service_id)
        .fetch_one(pg_pool)
        .await?;

    Ok(service_info)
}
