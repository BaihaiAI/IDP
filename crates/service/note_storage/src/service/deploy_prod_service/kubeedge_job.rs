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

#[derive(Debug, sqlx::FromRow)]
pub struct Equipment {
    pub image: String,
    pub edition_id: Option<i32>,
    pub equipment_sn: String,
}

pub async fn rollback_kubeedge_job(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<RollBackJobReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();

    if sqlx::query_as::<_, (String,)>(
        "
        SELECT	pre_image
        FROM	idp_model_service_pod
        WHERE	id = $1
    ",
    )
    .bind(req.equipment_id)
    .fetch_optional(&pg_pool)
    .await?
    .is_none()
    {
        return Err(ErrorTrace::new("can not rollback"));
    }
    let status = ServiceStatus::Deploying.to_string();
    let equipment: Equipment = sqlx::query_as(
        "
    UPDATE  idp_model_service_pod
    SET     edition_id = pre_edition_id, image = pre_image, package_id  = pre_package_id,
            pre_edition_id = null, pre_image = null, pre_package_id = null, status = $1
    WHERE   id = $2
    RETURNING image, edition_id, equipment_sn
    ",
    )
    .bind(&status)
    .bind(req.equipment_id)
    .fetch_one(&pg_pool)
    .await?;

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    let location;
    let mut project_id = None;
    let mut env_value = None;
    let mode = &crate::app_context::CONFIG.deploy_mode;
    let mode = if mode.eq_ignore_ascii_case("saas") {
        "public".to_string()
    } else {
        "private".to_string()
    };
    let service_type_str = ServiceType::to_string(3);
    if let Some(edition_id) = equipment.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id.to_string());
        env_value = Some(format!(
            r#""TEAM_ID"="{team_id}","PROJECT_ID"="{}","PATH_DIR"="{}""#,
            project_id.clone().unwrap(),
            location.unwrap()
        ));
    }

    destroy_kubeedge_pod(&pg_pool, team_id, req.service_id, req.equipment_id).await?;

    let client = reqwest::ClientBuilder::new().build().unwrap();

    let account = format!("{}-{}", req.service_id, req.equipment_id);
    let req = ModelDeployBody {
        account: account.clone(),
        team_id: team_id.to_string(),
        user_id: user_id.to_string(),
        action: "model-deploy-apply",
        service: "idp",
        region: team_region,
        project: project_id,
        pod_request_memory: None,
        pod_limit_memory: None,
        pod_request_cpu: None,
        pod_limit_cpu: None,
        pod_request_gpu: None,
        pod_limit_gpu: None,
        mode,
        image: equipment.image.clone(),
        replicas: None,
        env: env_value,
        service_id: account.clone(),
        service_type: service_type_str.clone(),
        schedule: None,
        node_sn: Some(equipment.equipment_sn.clone()),
    };
    deploy_model::post_model_deploy(&client, &req, &pg_pool, team_id, &account).await;

    Ok(Rsp::success_without_data())
}

pub async fn update_kubeedge_job(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<UPdateKubeedgeJobReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    let edition = get_edition(&pg_pool, req.edition_id).await?;
    let location = Some(edition.location.clone());
    let project_id = Some(edition.project_id);
    let tag = format!("{}.{}", req.package_id, req.edition_id);
    let image = build_image(
        &edition,
        req.service_id,
        &team_region,
        &pg_pool,
        3,
        Some(tag),
    )
    .await?;

    let status = ServiceStatus::ContainerCreating.to_string();
    let service_pod_id = sqlx::query_as::<_, (i64,)>(
        "
    UPDATE  idp_model_service_pod
    SET     pre_edition_id = edition_id, pre_image = image, pre_package_id = package_id,
            edition_id = $1, package_id = $2, image = $3, status = $4
    WHERE   equipment_sn = $5 and service_id = $6 and is_deleted = false
    RETURNING id;
    ",
    )
    .bind(req.edition_id)
    .bind(req.package_id)
    .bind(&image)
    .bind(&status)
    .bind(&req.equipment)
    .bind(req.service_id)
    .fetch_one(&pg_pool)
    .await?
    .0;

    destroy_kubeedge_pod(&pg_pool, team_id, req.service_id, service_pod_id).await?;

    tokio::spawn(deploy_kubeedge_pod(
        pg_pool.clone(),
        team_id,
        user_id,
        team_region,
        req.service_id,
        service_pod_id,
        image,
        req.equipment,
        project_id,
        location,
    ));

    Ok(Rsp::success_without_data())
}

pub async fn add_kubeedge_job(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<KubeedgeJobReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();
    let user_id = get_cookie_value_by_key(&cookies, "userId");
    let user_id = user_id.parse::<i64>().unwrap();
    let service_info = get_service_detail(&pg_pool, team_id, None, req.service_id).await?;
    let status = ServiceStatus::Deploying.to_string();
    let service_pod_id = sqlx::query_as::<_, (i64,)>(
        "
    INSERT  INTO    idp_model_service_pod 
            (service_id, status, equipment_sn, package_id, edition_id, image)
    VALUES  ($1, $2, $3, $4, $5, $6) 
    RETURNING id
    ",
    )
    .bind(req.service_id)
    .bind(&status)
    .bind(&req.equipment)
    .bind(service_info.package_id)
    .bind(service_info.edition_id)
    .bind(&service_info.image)
    .fetch_one(&pg_pool)
    .await
    .unwrap()
    .0;

    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     equipments = array_append(equipments,$1)
    WHERE   id=$2",
    )
    .bind(&req.equipment)
    .bind(req.service_id)
    .execute(&pg_pool)
    .await?;

    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(&pg_pool)
            .await?
            .0;
    let location;
    let mut project_id = None;
    let mut env_value = None;
    let mode = &crate::app_context::CONFIG.deploy_mode;
    let mode = if mode.eq_ignore_ascii_case("saas") {
        "public".to_string()
    } else {
        "private".to_string()
    };
    let service_type_str = ServiceType::to_string(service_info.service_type);

    if let Some(edition_id) = service_info.edition_id {
        let edition = get_edition(&pg_pool, edition_id).await?;
        location = Some(edition.location.clone());
        project_id = Some(edition.project_id.to_string());
        env_value = Some(format!(
            r#""TEAM_ID"="{team_id}","PROJECT_ID"="{}","PATH_DIR"="{}""#,
            project_id.clone().unwrap(),
            location.unwrap()
        ));
    }

    let client = reqwest::ClientBuilder::new().build().unwrap();

    let account = format!("{}-{service_pod_id}", req.service_id);
    let req = ModelDeployBody {
        account: account.clone(),
        team_id: team_id.to_string(),
        user_id: user_id.to_string(),
        action: "model-deploy-apply",
        service: "idp",
        region: team_region,
        project: project_id,
        pod_request_memory: None,
        pod_limit_memory: None,
        pod_request_cpu: None,
        pod_limit_cpu: None,
        pod_request_gpu: None,
        pod_limit_gpu: None,
        mode,
        image: service_info.image.clone(),
        replicas: None,
        env: env_value,
        service_id: account.clone(),
        service_type: service_type_str.clone(),
        schedule: None,
        node_sn: Some(req.equipment),
    };
    deploy_model::post_model_deploy(&client, &req, &pg_pool, team_id, &account).await;

    Ok(Rsp::success_without_data())
}

pub async fn delete_kubeedge_job(
    TypedHeader(cookies): TypedHeader<Cookies>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<KubeedgeJobReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    //todo 事务操作
    let team_id = get_cookie_value_by_key(&cookies, "teamId");
    let team_id = team_id.parse::<i64>().unwrap();

    let status = ServiceStatus::Stop.to_string();
    let service_pod_id = sqlx::query_as::<_, (i64,)>(
        "  
    UPDATE  idp_model_service_pod
    SET     is_deleted=true, status = $1
    WHERE   equipment_sn = $2 and service_id = $3 and is_deleted = false
    RETURNING id
    ",
    )
    .bind(&status)
    .bind(&req.equipment)
    .bind(req.service_id)
    .fetch_one(&pg_pool)
    .await
    .unwrap()
    .0;

    sqlx::query(
        "
    UPDATE  idp_model_deploy_prod_service
    SET     equipments = array_remove(equipments,$1)
    WHERE   id=$2",
    )
    .bind(&req.equipment)
    .bind(req.service_id)
    .execute(&pg_pool)
    .await?;

    destroy_kubeedge_pod(&pg_pool, team_id, req.service_id, service_pod_id).await?;

    Ok(Rsp::success_without_data())
}

pub async fn deploy_kubeedge_pod(
    pg_pool: sqlx::PgPool,
    team_id: i64,
    user_id: i64,
    region: String,
    service_id: i32,
    service_pod_id: i64,
    image: String,
    equipment: String,
    project_id: Option<i32>,
    location: Option<String>,
) {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    if !deploy_model::get_image_tag(&client, &image).await {
        sqlx::query(
            "
        UPDATE  idp_model_service_pod
        SET     status = $1, status_message = $2
        WHERE   id = $3",
        )
        .bind(ServiceStatus::Stop.to_string())
        .bind("Create Container failed")
        .bind(service_pod_id)
        .execute(&pg_pool)
        .await
        .unwrap();
        return;
    }
    let mut project = None;
    let mut env_value = None;
    if project_id.is_some() && location.is_some() {
        env_value = Some(format!(
            r#""TEAM_ID"="{team_id}","PROJECT_ID"="{}","PATH_DIR"="{}""#,
            project_id.unwrap(),
            location.unwrap()
        ));
        project = Some(project_id.unwrap().to_string())
    }
    let mode = &crate::app_context::CONFIG.deploy_mode;
    let mode = if mode.eq_ignore_ascii_case("saas") {
        "public".to_string()
    } else {
        "private".to_string()
    };
    let service_type_str = ServiceType::to_string(3);
    let account = format!("{service_id}-{service_pod_id}");

    let req = ModelDeployBody {
        account: account.clone(),
        team_id: team_id.to_string(),
        user_id: user_id.to_string(),
        action: "model-deploy-apply",
        service: "idp",
        region,
        project,
        pod_request_memory: None,
        pod_limit_memory: None,
        pod_request_cpu: None,
        pod_limit_cpu: None,
        pod_request_gpu: None,
        pod_limit_gpu: None,
        mode,
        image,
        replicas: None,
        env: env_value,
        service_id: account.clone(),
        service_type: service_type_str,
        schedule: None,
        node_sn: Some(equipment),
    };
    deploy_model::post_model_deploy(&client, &req, &pg_pool, team_id, &account).await;

    if let Err(err) = sqlx::query(
        "
    UPDATE  idp_model_service_pod
    SET     status = $1
    WHERE   id = $2",
    )
    .bind(ServiceStatus::Deploying.to_string())
    .bind(service_pod_id)
    .execute(&pg_pool)
    .await
    {
        tracing::error!("deploy update_service_pod_status{:?}", err)
    }
}

pub async fn destroy_kubeedge_pod(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    service_pod_id: i64,
) -> Result<(), ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build().unwrap();

    let account = format!("{service_id}-{service_pod_id}");
    let team_region =
        sqlx::query_as::<_, (String,)>("select region from team_info where team_id = $1")
            .bind(team_id)
            .fetch_one(pg_pool)
            .await?
            .0;

    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/destroy"))
        .json(&ModelDestroyBody {
            account,
            region: team_region,
            action: "model-destroy",
            service: "idp",
            platform: "idp-model",
            service_type: "job".to_string(),
        })
        .send()
        .await?;
    tracing::info!("{K8S_SERVICE_API_BASE_URL}/destroy end");

    let resp_body = resp.json::<RspBody>().await?;
    let resp_code = resp_body.code.to_string();
    if !resp_code.starts_with('2') {
        tracing::error!(
            "code:{resp_code},message:{},data:{:?}",
            resp_body.message,
            resp_body.data
        );
        return Err(ErrorTrace::new("failed to destroy model"));
    }
    Ok(())
}
