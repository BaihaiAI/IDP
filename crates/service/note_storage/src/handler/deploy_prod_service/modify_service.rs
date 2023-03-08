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

pub async fn insert_prod_service(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    project_id: i32,
    user_id: i64,
    req: &DeployServiceReq,
) -> Result<i32, ErrorTrace> {
    tracing::info!("insert idp_model_deploy_prod_service");

    if !ServiceType::legal(req.service_type) {
        return Err(ErrorTrace::new("Wrong serviceType"));
    }

    let status = ServiceStatus::ContainerCreating.to_string();

    let area = &crate::app_context::CONFIG.net_domain;

    let service_id = sqlx::query_as::<_, (i32,)>(
        "
    INSERT  INTO    idp_model_deploy_prod_service 
            (service_name, intro, team_id, project_id, user_id, package_id, edition_id,
            pod_cpu, pod_gpu, pod_memory, service_type, instance_count, equipments,
            status, image, cron_config, support_chip, support_system, area)
    VALUES  ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
    RETURNING id
        ",
    )
    .bind(&req.service_name)
    .bind(&req.service_intro)
    .bind(team_id)
    .bind(project_id)
    .bind(user_id)
    .bind(req.package_id)
    .bind(req.edition_id)
    .bind(req.pod_cpu)
    .bind(req.pod_gpu)
    .bind(req.pod_memory)
    .bind(req.service_type)
    .bind(req.instance_count)
    .bind(&req.equipments)
    .bind(&status)
    .bind(&req.image)
    .bind(&req.cron_config)
    .bind(&req.support_chip)
    .bind(&req.support_system)
    .bind(area)
    .fetch_one(pg_pool)
    .await?
    .0;

    Ok(service_id)
}

pub async fn delete_prod_service(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
) -> Result<(), ErrorTrace> {
    tracing::info!("delete idp_model_deploy_prod_service");

    let service_type = sqlx::query_as::<_, (i32,)>(
        "
    SELECT  service_type
    FROM    idp_model_deploy_prod_service
    WHERE   id =$1",
    )
    .bind(service_id)
    .fetch_one(pg_pool)
    .await?
    .0;

    let status = ServiceStatus::Stop.to_string();
    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     is_deleted=true, status = $1
    WHERE   id = $2 and team_id = $3
        ",
    )
    .bind(&status)
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    if service_type == 3 {
        sqlx::query(
            "
        UPDATE  idp_model_service_pod
        SET     is_deleted=true, status = $1
        WHERE   service_id = $2
            ",
        )
        .bind(status)
        .bind(service_id)
        .execute(pg_pool)
        .await?;
    }

    Ok(())
}

pub async fn stop_prod_service(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
) -> Result<(), ErrorTrace> {
    tracing::info!("stop idp_model_deploy_prod_service");

    let service_type = sqlx::query_as::<_, (i32,)>(
        "
    SELECT  service_type
    FROM    idp_model_deploy_prod_service
    WHERE   id =$1",
    )
    .bind(service_id)
    .fetch_one(pg_pool)
    .await?
    .0;

    let status = ServiceStatus::Stop.to_string();
    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     status = $1
    WHERE   id = $2 and team_id = $3
        ",
    )
    .bind(&status)
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    if service_type == 3 {
        sqlx::query(
            "
        UPDATE  idp_model_service_pod
        SET     status = $1
        WHERE   service_id = $2
            ",
        )
        .bind(status)
        .bind(service_id)
        .execute(pg_pool)
        .await?;
    }

    Ok(())
}

pub async fn update_service_resource(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    resource: &ResourceReq,
) -> Result<(), ErrorTrace> {
    tracing::info!("delete idp_model_deploy_prod_service");

    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     pod_cpu=$1, pod_gpu=$2, pod_memory=$3, instance_count=$4
    WHERE id = $4 and team_id = $5
        ",
    )
    .bind(resource.pod_cpu)
    .bind(resource.pod_gpu)
    .bind(resource.pod_memory)
    .bind(resource.instance_count)
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn update_service_cron(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    cron_config: Option<serde_json::Value>,
) -> Result<(), ErrorTrace> {
    tracing::info!("delete idp_model_deploy_prod_service");

    sqlx::query(
        "
        UPDATE  idp_model_deploy_prod_service
        SET     cron_config=$1, status = $2
        WHERE id = $3 and team_id = $4
        ",
    )
    .bind(cron_config)
    .bind(ServiceStatus::PreSchedule.to_string())
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn update_service_status(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    status: ServiceStatus,
    status_message: Option<&str>,
) -> Result<(), ErrorTrace> {
    tracing::info!("update_service_status");

    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     status=$1, status_message = $2
    WHERE id = $3 and team_id = $4
        ",
    )
    .bind(status.to_string())
    .bind(status_message)
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn update_service_pod_status(
    pg_pool: &sqlx::PgPool,
    pod_id: i32,
    status: ServiceStatus,
    status_message: Option<&str>,
) -> Result<(), ErrorTrace> {
    tracing::info!("update_service_pod_status");

    sqlx::query(
        "
    UPDATE  idp_model_service_pod
    SET     status=$1, status_message = $2
    WHERE id = $3
        ",
    )
    .bind(status.to_string())
    .bind(status_message)
    .bind(pod_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn update_service_image(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    req: &BuildImageReq,
) -> Result<(), ErrorTrace> {
    tracing::info!("delete idp_model_deploy_prod_service");

    sqlx::query(
        "
        UPDATE  idp_model_deploy_prod_service
        SET     image=$1, edition_id = $2, package_id = $3, env = $4
        WHERE id = $5 and team_id = $6
        ",
    )
    .bind(&req.image)
    .bind(req.edition_id)
    .bind(req.package_id)
    .bind(&req.env)
    .bind(service_id)
    .bind(team_id)
    .execute(pg_pool)
    .await?;

    Ok(())
}
