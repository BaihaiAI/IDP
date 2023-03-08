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

use reqwest::Client;
use tokio::time::Duration;

use super::*;
use crate::pojo::component::component_dir::CronConfigFields;

pub async fn deploy_model(
    pg_pool: sqlx::PgPool,
    team_id: i64,
    user_id: i64,
    service_id: i32,
    image: String,
    pod_memory: Option<i32>,
    pod_cpu: Option<i32>,
    pod_gpu: Option<i32>,
    service_type: i32,
    region: String,
    instance_count: Option<i32>,
    cron_config: Option<serde_json::Value>,
    location: Option<String>,
    project_id: Option<i32>,
    equipments: Option<Vec<String>>,
    package_id: Option<i32>,
    edition_id: Option<i32>,
) {
    //if !cron_config {
    //update_service_status(preschedule)
    //      return;
    // }

    let client = reqwest::ClientBuilder::new().build().unwrap();
    if !get_image_tag(&client, &image).await {
        update_service_status(
            &pg_pool,
            team_id,
            service_id,
            ServiceStatus::Stop,
            Some("Deploy failed"),
        )
        .await
        .unwrap();
        return;
    }
    tracing::info!("run deploy_model,team_id={:?}", team_id);
    //todo env
    let mut pod_request_memory_value = None;
    let mut pod_limit_memory_value = None;
    let mut pod_request_cpu_value = None;
    let mut pod_limit_cpu_value = None;
    let mut pod_request_gpu_value = None;
    let mut pod_limit_gpu_value = None;
    let mode = &crate::app_context::CONFIG.deploy_mode;
    let mut env_value = None;
    let mut replicas = None;
    let mut project = None;

    let mut cron_expression = None;

    if let Some(cron_config) = cron_config {
        let tmp = Some(&cron_config);
        if tmp.unwrap().to_string().len() > 10 {
            let obj: CronConfigFields = serde_json::from_value(cron_config).unwrap();
            cron_expression = Some(obj.cron_expression);
        }
    }

    if let Some(pod_memory) = pod_memory {
        pod_request_memory_value = Some(format!("{}Gi", pod_memory));
        pod_limit_memory_value = Some(format!("{}Gi", pod_memory));
    }
    if let Some(pod_cpu) = pod_cpu {
        pod_request_cpu_value = Some(pod_cpu.to_string());
        pod_limit_cpu_value = Some(pod_cpu.to_string());
    }
    if let Some(pod_gpu) = pod_gpu {
        pod_request_gpu_value = Some(pod_gpu.to_string());
        pod_limit_gpu_value = Some(pod_gpu.to_string());
    }
    if let Some(instance_count) = instance_count {
        replicas = Some(instance_count.to_string());
    }

    if project_id.is_some() && location.is_some() {
        env_value = Some(format!(
            r#""TEAM_ID"="{team_id}","PROJECT_ID"="{}","PATH_DIR"="{}""#,
            project_id.unwrap(),
            location.unwrap()
        ));
        project = Some(project_id.unwrap().to_string())
    }

    let mode = if mode.eq_ignore_ascii_case("saas") {
        "public".to_string()
    } else {
        "private".to_string()
    };

    let service_type_str = ServiceType::to_string(service_type);
    if service_type != 3 {
        let service_id = service_id.to_string();
        let req = ModelDeployBody {
            account: service_id.clone(),
            team_id: team_id.to_string(),
            user_id: user_id.to_string(),
            action: "model-deploy-apply",
            service: "idp",
            region: region.clone(),
            project: project.clone(),
            pod_request_memory: pod_request_memory_value,
            pod_limit_memory: pod_limit_memory_value,
            pod_request_cpu: pod_request_cpu_value,
            pod_limit_cpu: pod_limit_cpu_value,
            pod_request_gpu: pod_request_gpu_value,
            pod_limit_gpu: pod_limit_gpu_value,
            mode: mode.to_string(),
            image: image.clone(),
            replicas,
            env: env_value.clone(),
            service_id: service_id.clone(),
            service_type: service_type_str.clone(),
            schedule: cron_expression,
            node_sn: None,
        };
        post_model_deploy(&client, &req, &pg_pool, team_id, &service_id).await;
    } else {
        for equipment in equipments.unwrap() {
            let service_pod_id = sqlx::query_as::<_, (i64,)>(
                "
            SELECT  id
            FROM    idp_model_service_pod
            WHERE   service_id = $1 and equipment_sn = $2 and is_deleted = false
                ",
            )
            .bind(service_id)
            .bind(&equipment)
            .fetch_optional(&pg_pool)
            .await
            .unwrap();
            let service_pod_id = match service_pod_id {
                Some(id) => {
                    let id = id.0;
                    sqlx::query(
                        "
                    UPDATE  idp_model_service_pod
                    SET     status=$1
                    WHERE id = $2
                        ",
                    )
                    .bind(ServiceStatus::Deploying.to_string())
                    .bind(id)
                    .execute(&pg_pool)
                    .await
                    .unwrap();
                    id
                }
                None => {
                    let status = ServiceStatus::Deploying.to_string();

                    sqlx::query_as::<_, (i64,)>(
                        "
                    INSERT  INTO    idp_model_service_pod 
                            (service_id, status, equipment_sn, package_id, edition_id, image)
                    VALUES  ($1, $2, $3, $4, $5, $6) 
                    RETURNING id
                        ",
                    )
                    .bind(service_id)
                    .bind(status)
                    .bind(&equipment)
                    .bind(package_id)
                    .bind(edition_id)
                    .bind(&image)
                    .fetch_one(&pg_pool)
                    .await
                    .unwrap()
                    .0
                }
            };

            let account = format!("{service_id}-{service_pod_id}");
            let req = ModelDeployBody {
                account: account.clone(),
                team_id: team_id.to_string(),
                user_id: user_id.to_string(),
                action: "model-deploy-apply",
                service: "idp",
                region: region.clone(),
                project: project.clone(),
                pod_request_memory: None,
                pod_limit_memory: None,
                pod_request_cpu: None,
                pod_limit_cpu: None,
                pod_request_gpu: None,
                pod_limit_gpu: None,
                mode: mode.to_string(),
                image: image.clone(),
                replicas: None,
                env: env_value.clone(),
                service_id: account.clone(),
                service_type: service_type_str.clone(),
                schedule: None,
                node_sn: Some(equipment),
            };
            post_model_deploy(&client, &req, &pg_pool, team_id, &account).await;
        }
    }
    if let Err(err) = update_service_status(
        &pg_pool,
        team_id,
        service_id,
        ServiceStatus::Deploying,
        None,
    )
    .await
    {
        tracing::error!("deploy update_service_status{:?}", err)
    };
}

pub async fn post_model_deploy(
    client: &Client,
    req: &ModelDeployBody,
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: &str,
) {
    tracing::info!("run post_model_deploy.req:{:?}", req);

    let resp = match client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/model-deploy"))
        .json(req)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(err) => {
            tracing::error!("{err}");
            service_failed(pg_pool, team_id, service_id).await;
            return;
        }
    };

    let resp_body = match resp.json::<RspBody>().await {
        Ok(body) => body,
        Err(err) => {
            tracing::error!("{err}");
            service_failed(pg_pool, team_id, service_id).await;
            return;
        }
    };
    let resp_code = resp_body.code.to_string();
    if !resp_code.starts_with('2') {
        tracing::error!(
            "code:{resp_code},message:{},data:{:?}",
            resp_body.message,
            resp_body.data
        );
        service_failed(pg_pool, team_id, service_id).await;
    }
}

async fn service_failed(pg_pool: &sqlx::PgPool, team_id: i64, service_id: &str) {
    if let Ok(service_id) = service_id.parse::<i32>() {
        update_service_status(
            pg_pool,
            team_id,
            service_id,
            ServiceStatus::Stop,
            Some("Deploy failed"),
        )
        .await
        .unwrap();
    } else {
        let ids: Vec<&str> = service_id.split('-').collect();
        update_service_pod_status(
            pg_pool,
            ids[1].parse::<i32>().unwrap(),
            ServiceStatus::Stop,
            Some("Deploy failed"),
        )
        .await
        .unwrap();
    }
}

pub async fn get_image_tag(client: &Client, image: &str) -> bool {
    tracing::info!("run get_image_tag");

    let image: Vec<&str> = image.split(':').collect();
    let image_name = image[0].to_string();
    let tag_name = if image.len() > 1 {
        image[1].to_string()
    } else {
        "latest".to_string()
    };
    let req = GetImageBody {
        repo_name: "idp-saas",
        image_name,
        tag_name,
    };

    let mut i = 0;
    loop {
        i += 1;
        if i > 120 {
            tracing::error!("get_image_tag timeout");
            return false;
        }
        let resp = match client
            .post(format!("{ADMIN_API_BASE_URL}/cloud-api/get-image-tag"))
            .json(&req)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(err) => {
                tracing::error!("{err}");
                return false;
            }
        };

        let resp_body = match resp.json::<RspBody>().await {
            Ok(body) => body,
            Err(err) => {
                tracing::error!("{err}");
                return false;
            }
        };
        let resp_code = resp_body.code.to_string();
        if !resp_code.starts_with('2') {
            tracing::error!(
                "code:{resp_code},message:{},data:{:?}",
                resp_body.message,
                resp_body.data
            );
        }
        if resp_body.data == true {
            return true;
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
