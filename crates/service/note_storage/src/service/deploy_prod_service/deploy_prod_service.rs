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

pub async fn deploy(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<DeployServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("run deploy service");
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;

    let project_id = get_cookie_value_by_key(&cookies, "projectId");
    let project_id = project_id.parse::<i32>()?;

    tracing::info!("user_id={user_id},team_id={team_id},project_id={project_id}");

    let service_id = insert_prod_service(&pg_pool, team_id, project_id, user_id, &req).await?;

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;

    let mut location = None;
    let mut project_id = None;
    let image = if let Some(edition_id) = req.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id);
        build_image(
            &edition,
            service_id,
            &team_region,
            &pg_pool,
            req.service_type,
            None,
        )
        .await?
    } else {
        req.image
    };

    sqlx::query(
        "
        UPDATE  idp_model_deploy_prod_service
        SET     image=$1
        WHERE id = $2
        ",
    )
    .bind(&image)
    .bind(service_id)
    .execute(&pg_pool)
    .await?;

    let image_clone = image.clone();
    let pg_pool_clone = pg_pool.clone();
    if req.service_type == 2 {
        tokio::spawn(async move {
            let client = reqwest::ClientBuilder::new().build().unwrap();

            let (status, status_message) =
                match deploy_model::get_image_tag(&client, &image_clone).await {
                    true => (ServiceStatus::PreSchedule, None),
                    false => (ServiceStatus::Stop, Some("Deploy failed")),
                };

            update_service_status(&pg_pool_clone, team_id, service_id, status, status_message)
                .await
                .unwrap();
        });
    } else {
        tokio::spawn(deploy_model(
            pg_pool.clone(),
            team_id,
            user_id,
            service_id,
            image,
            req.pod_memory,
            req.pod_cpu,
            req.pod_gpu,
            req.service_type,
            team_region,
            req.instance_count,
            req.cron_config,
            location,
            project_id,
            req.equipments,
            req.package_id,
            req.edition_id,
        ));
    }

    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Deploy,
        service_id,
    )
    .await?;

    Ok(Rsp::success_without_data())
}

pub async fn start(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("run start service");
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    tracing::info!("user_id={user_id},team_id={team_id}");

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;

    let service_info = get_service_detail(&pg_pool, team_id, None, req.service_id).await?;
    let mut location = None;
    let mut project_id = None;
    if let Some(edition_id) = service_info.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id);
    }

    if service_info.service_type == 2 {
        update_service_status(
            &pg_pool,
            team_id,
            req.service_id,
            ServiceStatus::PreSchedule,
            None,
        )
        .await?;
    } else {
        tokio::spawn(deploy_model(
            pg_pool.clone(),
            team_id,
            user_id,
            req.service_id,
            service_info.image,
            service_info.pod_memory,
            service_info.pod_cpu,
            service_info.pod_gpu,
            service_info.service_type,
            team_region,
            service_info.instance_count,
            service_info.cron_config,
            location,
            project_id,
            service_info.equipments,
            service_info.package_id,
            service_info.edition_id,
        ));

        add_aperation_log(
            &pg_pool,
            team_id,
            user_id,
            ServiceOperation::Start,
            req.service_id,
        )
        .await?;
    }

    Ok(Rsp::success_without_data())
}

pub async fn delete(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    destroy_model(&pg_pool, team_id, req.service_id, &team_region).await?;
    delete_prod_service(&pg_pool, team_id, req.service_id).await?;
    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Delete,
        req.service_id,
    )
    .await?;
    Ok(Rsp::success_without_data())
}

pub async fn stop(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    destroy_model(&pg_pool, team_id, req.service_id, &team_region).await?;
    stop_prod_service(&pg_pool, team_id, req.service_id).await?;
    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Stop,
        req.service_id,
    )
    .await?;
    Ok(Rsp::success_without_data())
}

pub async fn run_once(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("run_once service");
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    tracing::info!("user_id={user_id},team_id={team_id}");

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;

    let service_info = get_service_detail(&pg_pool, team_id, None, req.service_id).await?;
    let mut location = None;
    let mut project_id = None;
    if let Some(edition_id) = service_info.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id);
    }

    if service_info.service_type == 2 {
        update_service_status(
            &pg_pool,
            team_id,
            req.service_id,
            ServiceStatus::PreSchedule,
            None,
        )
        .await?;
    } else {
        tokio::spawn(deploy_model(
            pg_pool.clone(),
            team_id,
            user_id,
            req.service_id,
            service_info.image,
            service_info.pod_memory,
            service_info.pod_cpu,
            service_info.pod_gpu,
            service_info.service_type,
            team_region,
            service_info.instance_count,
            service_info.cron_config,
            location,
            project_id,
            service_info.equipments,
            service_info.package_id,
            service_info.edition_id,
        ));

        add_aperation_log(
            &pg_pool,
            team_id,
            user_id,
            ServiceOperation::Start,
            req.service_id,
        )
        .await?;
    }

    Ok(Rsp::success_without_data())
}

pub async fn list(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(params): Query<ServiceListReq>,
) -> Result<Rsp<ServiceListRsp>, ErrorTrace> {
    tracing::debug!("run get_message_list service");

    let size = params.page_size;
    let current = params.page_index;

    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    tracing::info!("team_id={team_id}");
    let resp = get_service_list(&pg_pool, size, current, team_id, params.service_type).await?;

    Ok(Rsp::success(resp))
}

pub async fn service_detail(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<ServiceReq>,
) -> Result<Rsp<ServiceInfo>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;

    let cookie_opt = cookies.get("id_token");
    let id_token = if let Some(val) = cookie_opt {
        val.to_string()
    } else {
        String::default()
    };

    let resp = get_service_detail(&pg_pool, team_id, Some(id_token), req.service_id).await?;

    Ok(Rsp::success(resp))
}

pub async fn operation_list(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<ServiceReq>,
) -> Result<Rsp<OperationRsp>, ErrorTrace> {
    tracing::debug!("run operation_list service");
    let resp = log_list(&pg_pool, req.service_id).await?;
    Ok(Rsp::success(resp))
}

pub async fn get_service_equipment(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(params): Query<GetEquipmentReq>,
) -> Result<Rsp<EquipmentListRsp>, ErrorTrace> {
    tracing::debug!("run get_service_equipment service");

    let size = params.page_size;
    let current = params.page_index;

    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>()?;
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    tracing::info!("user_id={user_id},team_id={team_id}");
    let resp = get_equipment_list(&pg_pool, size, current, team_id, params.service_id).await?;

    Ok(Rsp::success(resp))
}

pub async fn get_resource(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<ResourceRsp>, ErrorTrace> {
    tracing::info!("run get_resource");
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>()?;
    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    let resp = get_resource_handler(team_id, &team_region).await?;

    Ok(Rsp::success(resp))
}

pub async fn get_edition(
    pg_pool: &sqlx::PgPool,
    edition_id: i32,
) -> Result<EditionInfo, ErrorTrace> {
    let edition: EditionInfo = match sqlx::query_as(
        "
    SELECT  location, runtime_env, team_id, project_id, image
    FROM    idp_model_edition
    WHERE   id = $1
        ",
    )
    .bind(edition_id)
    .fetch_optional(pg_pool)
    .await
    {
        Err(msg) => {
            tracing::error!("Failed to select edition from DB: {msg:?}");
            return Err(ErrorTrace::new("Failed to build image"));
        }
        Ok(edition_opt) => match edition_opt {
            None => {
                tracing::error!("edition is none");
                return Err(ErrorTrace::new("Failed to build image"));
            }
            Some(edition_info) => edition_info,
        },
    };
    Ok(edition)
}
