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

use business::kubernetes::is_k8s;
use chrono::Local;

use super::*;
use crate::pojo::component::component_dir::CronConfigFields;
use crate::service::deploy_prod_service::get_edition;
use crate::service::schedule::schedule_utils::check_to_run_time;

pub async fn check_model_service(pg_pool: &sqlx::PgPool) {
    tracing::info!("########check_model_service start");

    let hostname = os_utils::get_hostname();
    let service_sql = if is_k8s() {
        let mut parts = hostname.split('-').skip(2);
        let region = parts.next().unwrap();
        let team_id = parts.next().unwrap();
        let deploy_mode = &crate::app_context::CONFIG.deploy_mode;
        let area = &crate::app_context::CONFIG.net_domain;

        if deploy_mode.to_lowercase().contains("saas") {
            format!(
                "SELECT imdps.id, imdps.cron_config, imdps.service_type \
                 FROM   idp_model_deploy_prod_service as imdps \
                 INNER JOIN team_info as t on t.team_id = imdps.team_id  \
                 WHERE  imdps.status != 'stop' and imdps.status != 'containerCreating' and and imdps.service_type != 3  \
                 AND    imdps.area='{}' and t.region={} ",
                area, region
            )
        } else {
            format!(
                "SELECT  id, cron_config, service_type \
                 FROM    idp_model_deploy_prod_service \
                 WHERE   status != 'stop' and status != 'containerCreating' and service_type!= 3 \
                 AND area='{}' and team_id={}",
                area, team_id
            )
        }
    } else {
        r#"
        SELECT  id, cron_config, service_type
        FROM    idp_model_deploy_prod_service
        WHERE   status != 'stop' and status != 'containerCreating' and service_type != 3
        "#
        .to_string()
    };

    tracing::info!("##check_model_service service_sql={}", service_sql);

    let service_info: Vec<CheckServiceInfo> = match sqlx::query_as(service_sql.as_str())
        .fetch_all(pg_pool)
        .await
    {
        Ok(info) => info,
        Err(err) => {
            tracing::error!("{err}");
            Vec::new()
        }
    };

    let namespace = &*business::kubernetes::NAMESPACE;
    if let Err(err) = update_service_info_status(service_info, pg_pool, namespace).await {
        tracing::error!("{err}")
    }

    let pod_sql = if is_k8s() {
        let mut parts = hostname.split('-').skip(2);
        let region = parts.next().unwrap();
        let team_id = parts.next().unwrap();
        let deploy_mode = &crate::app_context::CONFIG.deploy_mode;
        let area = &crate::app_context::CONFIG.net_domain;

        if deploy_mode.to_lowercase().contains("saas") {
            format!(
                "SELECT  pod.id, pod.service_id \
                 FROM    idp_model_service_pod as pod \
                 INNER JOIN idp_model_deploy_prod_service as imdps on imdps.id = pod.service_id \
                 INNER JOIN team_info as t on t.team_id = imdps.team_id  \
                 WHERE   pod.status != 'stop' and pod.status != 'containerCreating' \
                 AND    imdps.area='{}' and t.region={} ",
                area, region
            )
        } else {
            format!(
                "SELECT  pod.id, pod.service_id \
                 FROM    idp_model_service_pod as pod \
                 INNER JOIN idp_model_deploy_prod_service as imdps on imdps.id = pod.service_id \
                 INNER JOIN team_info as t on t.team_id = imdps.team_id  \
                 WHERE   pod.status != 'stop' and pod.status != 'containerCreating' \
                 AND   imdps.area='{}' and t.team_id={}",
                area, team_id
            )
        }
    } else {
        r#"
        SELECT  id,service_id
        FROM    idp_model_service_pod
        WHERE   status != 'stop' and status != 'containerCreating'
        "#
        .to_string()
    };

    tracing::info!("##check_model_service pod_sql={}", pod_sql);

    // let pod_sql = r#"
    //     SELECT  id,service_id
    //     FROM    idp_model_service_pod
    //     WHERE   status != 'stop'
    //     "#;

    let pod_info: Vec<CheckPodInfo> =
        match sqlx::query_as(pod_sql.as_str()).fetch_all(pg_pool).await {
            Ok(info) => info,
            Err(err) => {
                tracing::error!("{err}");
                Vec::new()
            }
        };

    if let Err(err) = update_pod_status(pod_info, pg_pool, namespace).await {
        tracing::error!("{err}")
    }

    tracing::info!("########check_model_service end");
}

pub async fn update_service_info_status(
    service_info: Vec<CheckServiceInfo>,
    pg_pool: &sqlx::PgPool,
    namespace: &str,
) -> Result<(), ErrorTrace> {
    tracing::debug!("########update_service_info_status start");

    let mut get_status_futs = Vec::new();

    for item in service_info.iter() {
        get_status_futs.push(get_status(
            item.id.to_string(),
            namespace.to_string(),
            item.service_type,
        ));
    }

    let mut update_service_futs = Vec::new();
    for resp in futures::future::join_all(get_status_futs).await {
        match resp {
            Ok(resp) => {
                let id = resp.account.parse::<i32>()?;
                let fut = sqlx::query(
                    "
                UPDATE  idp_model_deploy_prod_service 
                SET     status=$1 ,status_message=$2 
                WHERE   id=$3
                ",
                )
                .bind(resp.status)
                .bind(resp.stauts_message)
                .bind(id)
                .execute(pg_pool);
                update_service_futs.push(fut);
            }
            Err(err) => tracing::error!("get_status failed:{err}"),
        }
    }
    futures::future::try_join_all(update_service_futs).await?;

    tracing::debug!("########update_service_info_status end");
    Ok(())
}

pub async fn update_pod_status(
    pod_info: Vec<CheckPodInfo>,
    pg_pool: &sqlx::PgPool,
    namespace: &str,
) -> Result<(), ErrorTrace> {
    tracing::debug!("########update_pod_status start");

    let mut get_status_futs = Vec::new();

    for item in pod_info.iter() {
        let account = format!("{}-{}", item.service_id, item.id);
        get_status_futs.push(get_status(account, namespace.to_string(), 3));
    }

    let mut update_pod_futs = Vec::new();
    for resp in futures::future::join_all(get_status_futs).await {
        match resp {
            Ok(resp) => {
                let service_pod: Vec<&str> = resp.account.split('-').collect();
                let pod_id = service_pod[1].parse::<i64>()?;
                let fut = sqlx::query(
                    "
                UPDATE  idp_model_service_pod 
                SET     status=$1 ,status_message=$2 
                WHERE   id=$3
                ",
                )
                .bind(resp.status)
                .bind(resp.stauts_message)
                .bind(pod_id)
                .execute(pg_pool);
                update_pod_futs.push(fut);
            }
            Err(err) => tracing::error!("get_status failed:{err}"),
        }
    }
    futures::future::try_join_all(update_pod_futs).await?;

    tracing::debug!("########update_service_status end");
    Ok(())
}

#[derive(Debug, Default)]
struct Status {
    pub account: String,
    pub status: String,
    pub stauts_message: Option<String>,
}

async fn get_status(
    account: String,
    namespace: String,
    service_type: i32,
) -> Result<Status, ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build()?;

    let service_type = match service_type {
        1 => "Deployment",
        2 => "CronJob",
        3 => "Pod",
        _ => "",
    };

    let mut status_resp = Status {
        account: account.clone(),
        ..Default::default()
    };

    let params = ServiceStatusReq {
        account,
        namespace,
        service_type: service_type.to_string(),
    };
    tracing::info!("get_status:params:{:?}", params);
    let resp = client
        .post(format!("{K8S_SERVICE_API_V2_BASE_URL}/status"))
        .json(&params)
        .send()
        .await?;

    match service_type {
        "Deployment" => {
            let resp_body = resp.json::<DeploymentStatus>().await?;
            if resp_body.data.is_empty() {
                status_resp.status = ServiceStatus::Abnormal.to_string();
                status_resp.stauts_message = Some("deployment not found".to_string());
                return Ok(status_resp);
            }
            let _available_replicas = resp_body.data[0].available_replicas;

            let mut deploying_flag = false;
            let mut normal_flag = false;
            let mut abnormal_flag = false;
            let mut message = None;
            for condition in &resp_body.data[0].conditions {
                if condition.r#type == "Progressing" && condition.status == "True" {
                    deploying_flag = true;
                }
                if condition.r#type == "Available" && condition.status == "True" {
                    normal_flag = true;
                }
                if condition.r#type == "Progressing" && condition.status == "False" {
                    abnormal_flag = true;
                    message = Some(condition.message.clone());
                }
            }

            if deploying_flag {
                status_resp.status = ServiceStatus::Deploying.to_string();
            }
            if normal_flag {
                status_resp.status = ServiceStatus::Normal.to_string();
            }
            if abnormal_flag {
                status_resp.status = ServiceStatus::Abnormal.to_string();
                status_resp.stauts_message = message;
            }
        }
        "CronJob" => {
            let resp_body = resp.json::<RspBody>().await?;
            let resp_code = resp_body.code.to_string();
            if !resp_code.starts_with('2') {
                tracing::error!(
                    "code:{resp_code},message:{},data:{:?}",
                    resp_body.message,
                    resp_body.data
                );
                return Err(ErrorTrace::new(&resp_body.message));
            }

            let resp = serde_json::from_value::<Vec<CronJobStatusData>>(resp_body.data)?;
            if resp.is_empty() {
                status_resp.status = ServiceStatus::Abnormal.to_string();
                status_resp.stauts_message = Some("CronJob not found".to_string());
                return Ok(status_resp);
            }
            status_resp.status = ServiceStatus::Normal.to_string();
            // if let Some(_active) = &resp[0].active {
            //     todo!()
            // }
        }
        "Pod" => {
            let resp_body = resp.json::<PodStatus>().await?;
            let mut deploying_flag = false;
            let mut normal_flag = false;
            let mut abnormal_flag = false;
            let mut message = None;

            if resp_body.data.is_empty() {
                status_resp.status = ServiceStatus::Abnormal.to_string();
                status_resp.stauts_message = Some("Pod not found".to_string());
                return Ok(status_resp);
            }

            for pod_status in &resp_body.data[0] {
                if pod_status.r#type == "PodScheduled" && pod_status.status == "True" {
                    deploying_flag = true;
                }
                if pod_status.r#type == "Ready" && pod_status.status == "True" {
                    normal_flag = true;
                }
                if pod_status.r#type == "Unschedulable" && pod_status.status == "True" {
                    abnormal_flag = true;
                    message = pod_status.message.clone();
                }
            }

            if deploying_flag {
                status_resp.status = ServiceStatus::Deploying.to_string();
            }
            if normal_flag {
                status_resp.status = ServiceStatus::Normal.to_string();
            }
            if abnormal_flag {
                status_resp.status = ServiceStatus::Abnormal.to_string();
                status_resp.stauts_message = message;
            }
        }
        _ => return Err(ErrorTrace::new("invalid service type")),
    };

    Ok(status_resp)
}

//是否生成定时K8S的CronJob检查
pub async fn check_model_batch_service_handler(pg_pool: &sqlx::PgPool) {
    tracing::info!("check_model_batch_service_handler / minute ......");
    match check_model_batch_service(pg_pool).await {
        Ok(x) => x,
        Err(err) => {
            tracing::error!("{err}")
        }
    };
}

pub async fn check_model_batch_service(pg_pool: &sqlx::PgPool) -> Result<(), ErrorTrace> {
    tracing::info!("########check_model_batch_service start");

    let hostname = os_utils::get_hostname();
    let service_sql = if is_k8s() {
        let mut parts = hostname.split('-').skip(2);
        let region = parts.next().unwrap();
        let team_id = parts.next().unwrap();
        let deploy_mode = &crate::app_context::CONFIG.deploy_mode;
        let area = &crate::app_context::CONFIG.net_domain;

        if deploy_mode.to_lowercase().contains("saas") {
            format!(
                "SELECT  imdps.id, imdps.cron_config, imdps.team_id, imdps.user_id  \
                 FROM    idp_model_deploy_prod_service as imdps  \
                 INNER JOIN team_info as t on t.team_id = imdps.team_id  \
                 WHERE imdps.is_deleted=false and   imdps.status = 'PreSchedule' and imdps.service_type = 2  \
                 AND imdps.area='{}' and t.region={} ",
                area, region
            )
        } else {
            format!(
                "SELECT  id, cron_config, team_id, user_id  \
                 FROM    idp_model_deploy_prod_service  \
                 WHERE   is_deleted=false and  status = 'PreSchedule' and service_type = 2  \
                 AND area='{}' and team_id={}",
                area, team_id
            )
        }
    } else {
        r#"
            SELECT  id, cron_config, team_id, user_id  FROM    idp_model_deploy_prod_service WHERE  is_deleted=false and  status = 'PreSchedule' and service_type = 2
        "#.to_string()
    };

    tracing::info!("service_sql={:?}", service_sql);

    let service_list: Vec<CheckBatchServiceInfo> = sqlx::query_as(service_sql.as_str())
        .fetch_all(pg_pool)
        .await?;

    tracing::info!("service_list={:?}", service_list);

    for (_index, item) in service_list.iter().enumerate() {
        tracing::debug!("!!item={:?}", item);
        if item.cron_config.is_some() {
            let json = item.cron_config.clone().unwrap();
            tracing::debug!("!!json={:?}", json);

            let service_id = item.id;
            let team_id = item.team_id;
            let user_id = item.user_id;

            if json.to_string().len() > 10 {
                //cron_config is a validate string
                let obj: CronConfigFields = serde_json::from_value(json)?;
                tracing::debug!("!!obj.cron_expression={:?}", obj.cron_expression);
                tracing::debug!("!!obj.cron_start_date={:?}", obj.cron_start_date);
                tracing::debug!("!!obj.cron_end_date={:?}", obj.cron_end_date);

                let cron_start_time = if obj.cron_start_time.is_some() {
                    let mut tmp = obj.cron_start_time.unwrap();
                    tmp = tmp.replace(':', "-");
                    tmp += "-00";
                    tmp
                } else {
                    "00-00-00".to_string()
                };

                let cron_end_time = if obj.cron_end_time.is_some() {
                    let mut tmp = obj.cron_end_time.unwrap();
                    tmp = tmp.replace(':', "-");
                    tmp += "-00";
                    tmp
                } else {
                    "00-00-00".to_string()
                };

                let do_it = check_to_run_time(
                    Local::now(),
                    obj.cron_expression.as_str(),
                    obj.cron_start_date.as_str(),
                    obj.cron_end_date.as_str(),
                    cron_start_time.as_str(),
                    cron_end_time.as_str(),
                )?;

                tracing::info!("-------------------------------");
                tracing::info!("!!check_model_batch_service do_it={:?}", do_it);

                if do_it {
                    let service_info =
                        get_service_detail(pg_pool, team_id, None, service_id).await?;
                    tracing::info!("!!service_info={:?}", service_info);

                    let team_region = sqlx::query_as::<_, (String,)>(
                        "select region from team_info where team_id = $1",
                    )
                    .bind(team_id)
                    .fetch_one(pg_pool)
                    .await?
                    .0;

                    let mut location = None;
                    let mut project_id = None;
                    if let Some(edition_id) = service_info.edition_id {
                        let edition = get_edition(pg_pool, edition_id).await?;
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
                }
            }
        }
    }
    tracing::info!("########check_model_batch_service end");
    Ok(())
}
