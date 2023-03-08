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
use err::ErrorTrace;
use sqlx::PgPool;

use crate::pojo::component::component_dir::CronConfigFields;
use crate::pojo::component::component_dir::JobCronConfig;
use crate::service::component::dag_service::job_run_dag_service;
use crate::service::schedule::schedule_utils::check_to_run_time;

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/2/8
 * Time: 20:01
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

pub async fn check_visual_modeling_job_handler(pg_pool: &sqlx::PgPool) {
    tracing::info!("check_visual_modeling_job / minute ......");
    match check_visual_modeling_job(pg_pool).await {
        Ok(x) => x,
        Err(err) => {
            tracing::error!("{err}")
        }
    };
}
pub async fn check_visual_modeling_job(pg_pool: &PgPool) -> Result<(), ErrorTrace> {
    tracing::info!("########check_visual_modeling_job start");
    let hostname = os_utils::get_hostname();
    let sql = if is_k8s() {
        let mut parts = hostname.split('-').skip(2);
        let region = parts.next().unwrap();
        let team_id = parts.next().unwrap();
        let deploy_mode = &crate::app_context::CONFIG.deploy_mode;
        let area = &crate::app_context::CONFIG.net_domain;

        if deploy_mode.to_lowercase().contains("saas") {
            format!(
                "SELECT j.job_id,j.team_id,j.cron_config FROM idp_model_job as j  \
                inner join team_info as t on t.team_id=j.team_id  \
                WHERE j.status='Schedule' and t.area='{}' and t.region={} ",
                area, region
            )
        } else {
            format!(
                "SELECT job_id,team_id,cron_config FROM idp_model_job WHERE status='Schedule' and area='{}' and team_id={}",
                area, team_id
            )
        }
    } else {
        r#"
            SELECT job_id,team_id,cron_config FROM idp_model_job WHERE status='Schedule'
        "#
        .to_string()
    };
    tracing::debug!("#######sql={:?}", sql);

    let items = sqlx::query_as::<_, JobCronConfig>(sql.as_str())
        .fetch_all(pg_pool)
        .await?;

    tracing::debug!("########check_visual_modeling_job to items={:?}", items);

    for (_index, item) in items.iter().enumerate() {
        tracing::debug!("!!item={:?}", item);
        let json = item.cron_config.clone();
        tracing::debug!("!!json={:?}", json);

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
            tracing::info!("!!do_it={:?}", do_it);

            if do_it {
                let job_id = item.job_id;
                let team_id = item.team_id;
                let run_type = "Schedule";

                let run_node_type = crate::handler::visual_modeling::prelude::RunNodeType::All;
                let node_id = "".to_string();

                if let Err(err) =
                    job_run_dag_service(job_id, team_id, team_id, run_type, run_node_type, node_id)
                        .await
                {
                    tracing::error!("{err}");
                }
            }
        }
    }
    tracing::info!("########check_visual_modeling_job end");
    Ok(())
}
