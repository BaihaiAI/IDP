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

use err::ErrorTrace;

use crate::pojo::component::component_dir::ModelJobInstance;

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/2/4
 * Time: 16:29
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */
#[cfg(not)]
pub async fn _get_model_job_instance_detail_handler(
    Query(req): Query<ModelJobInstanceReq>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<ModelJobInstance>, ErrorTrace> {
    let job_instance_id = req.job_instance_id;
    get_model_job_instance_detail(job_instance_id, &pg_pool).await
}

#[cfg(not)]
pub async fn get_model_job_instance_detail(
    job_instance_id: i32,
    pg_pool: &sqlx::PgPool,
) -> Result<ModelJobInstance, ErrorTrace> {
    let sql = r#"SELECT job_instance_id,job_id,job_name,team_id,project_id,user_id,nodes,edges,cron_config,runtime_config FROM idp_model_job_instance WHERE job_instance_id = $1 for share"#;

    let model_job_instance_detail = sqlx::query_as::<_, ModelJobInstance>(sql)
        .bind(job_instance_id)
        .fetch_one(pg_pool)
        .await?;

    Ok(model_job_instance_detail)
}
