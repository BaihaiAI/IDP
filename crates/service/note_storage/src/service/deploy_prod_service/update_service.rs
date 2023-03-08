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

pub async fn update_all(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<UpdateServiceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();
    let service_id = req.service_id;

    let service_info = get_service_detail(&pg_pool, team_id, None, req.service_id).await?;

    let mut deploy_flag = false;

    if service_info.service_name != req.service_name
        || service_info.service_intro != req.service_intro
    {
        sqlx::query(
            "
        UPDATE  idp_model_deploy_prod_service
        SET     service_name=$1, intro=$2
        WHERE id = $3 and team_id = $4
            ",
        )
        .bind(req.service_name)
        .bind(req.service_intro)
        .bind(req.service_id)
        .bind(team_id)
        .execute(&pg_pool)
        .await?;
    }

    if service_info.pod_cpu != req.pod_cpu
        || service_info.pod_gpu != req.pod_gpu
        || service_info.pod_memory != req.pod_memory
        || service_info.instance_count != req.instance_count
    {
        deploy_flag = true;
        let resource_req = ResourceReq {
            service_id: req.service_id,
            pod_cpu: req.pod_cpu,
            pod_gpu: req.pod_gpu,
            pod_memory: req.pod_memory,
            instance_count: req.instance_count,
        };
        update_service_resource(&pg_pool, team_id, service_id, &resource_req).await?;
    }

    if service_info.cron_config != req.cron_config {
        deploy_flag = true;
        update_service_cron(&pg_pool, team_id, service_id, req.cron_config).await?;
    }

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    let mut image = req.image.clone();
    if service_info.package_id != req.package_id
        || service_info.edition_id != req.edition_id
        || service_info.image != req.image
        || service_info.env != req.env
    {
        deploy_flag = true;

        image = if let Some(edition_id) = req.edition_id {
            let edition = get_edition(&pg_pool, edition_id).await?;
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
            req.image.clone()
        };

        let image_req = BuildImageReq {
            service_id,
            image: image.clone(),
            edition_id: req.edition_id,
            package_id: req.package_id,
            env: req.env,
        };
        update_service_image(&pg_pool, team_id, service_id, &image_req).await?;
    }

    if service_info.equipments != req.equipments {
        //todo 更新边缘服务
        return Ok(Rsp::success_without_data());
    }

    if !deploy_flag {
        add_aperation_log(
            &pg_pool,
            team_id,
            user_id,
            ServiceOperation::Renew,
            service_id,
        )
        .await?;
        return Ok(Rsp::success_without_data());
    }

    destroy_model(&pg_pool, team_id, service_id, &team_region).await?;

    let service_info = get_service_detail(&pg_pool, team_id, None, service_id).await?;

    if service_info.service_type == 2 {
        update_service_status(
            &pg_pool,
            team_id,
            service_id,
            ServiceStatus::PreSchedule,
            None,
        )
        .await?;
        add_aperation_log(
            &pg_pool,
            team_id,
            user_id,
            ServiceOperation::Renew,
            service_id,
        )
        .await?;
        return Ok(Rsp::success_without_data());
    }

    let mut location = None;
    let mut project_id = None;
    if let Some(edition_id) = service_info.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id);
    }
    tokio::spawn(deploy_model(
        pg_pool.clone(),
        team_id,
        user_id,
        service_id,
        image,
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
    update_service_status(
        &pg_pool,
        team_id,
        service_id,
        ServiceStatus::ContainerCreating,
        None,
    )
    .await?;
    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Renew,
        service_id,
    )
    .await?;

    Ok(Rsp::success_without_data())
}

pub async fn update_resource(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<ResourceReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();
    let service_id = req.service_id;

    update_service_resource(&pg_pool, team_id, service_id, &req).await?;

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    destroy_model(&pg_pool, team_id, service_id, &team_region).await?;

    let service_info = get_service_detail(&pg_pool, team_id, None, service_id).await?;

    if service_info.service_type == 2 {
        update_service_status(
            &pg_pool,
            team_id,
            service_id,
            ServiceStatus::PreSchedule,
            None,
        )
        .await?;
        add_aperation_log(
            &pg_pool,
            team_id,
            user_id,
            ServiceOperation::Renew,
            service_id,
        )
        .await?;
        return Ok(Rsp::success_without_data());
    }
    let mut location = None;
    let mut project_id = None;
    if let Some(edition_id) = service_info.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id);
    }
    tokio::spawn(deploy_model(
        pg_pool.clone(),
        team_id,
        user_id,
        service_id,
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
        ServiceOperation::Renew,
        service_id,
    )
    .await?;

    Ok(Rsp::success_without_data())
}

pub async fn update_cron(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<CronReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();
    let service_id = req.service_id;

    update_service_cron(&pg_pool, team_id, service_id, req.cron_config).await?;
    update_service_status(
        &pg_pool,
        team_id,
        service_id,
        ServiceStatus::PreSchedule,
        None,
    )
    .await?;
    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    destroy_model(&pg_pool, team_id, service_id, &team_region).await?;

    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Renew,
        service_id,
    )
    .await?;

    Ok(Rsp::success_without_data())
}

pub async fn update_basic_info(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<BasicInfoReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();
    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     service_name=$1, intro=$2
    WHERE id = $3 and team_id = $4
        ",
    )
    .bind(req.service_name)
    .bind(req.intro)
    .bind(req.service_id)
    .bind(team_id)
    .execute(&pg_pool)
    .await?;
    add_aperation_log(
        &pg_pool,
        team_id,
        user_id,
        ServiceOperation::Renew,
        req.service_id,
    )
    .await?;
    Ok(Rsp::success_without_data())
}
