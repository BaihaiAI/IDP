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

use crate::handler::visual_modeling::graph_run_impl::GraphRunCtx;
use crate::handler::visual_modeling::prelude::RunNodeType;
use crate::pojo::component::graph::Graph;
use crate::service::component::component_lib_service::get_job_detail;

type JobInstanceId = i32;
/*
should add these up/down sql to database migration, but current we has not migration

alter table idp_model_job_instance add column run_node_type varchar;
alter table idp_model_job_instance alter column run_node_type set not null;
alter table idp_model_job_instance alter column run_node_type type varchar(6);

pg_dump -U postgres -d idp_prod_saas -t idp_model_job_instance --schema-only
*/
pub async fn job_run_dag_service(
    job_id: i32,
    team_id: i64,
    user_id: i64,
    run_type: &str,
    run_node_type: RunNodeType,
    node_id: String,
) -> Result<JobInstanceId, ErrorTrace> {
    debug_assert!(run_type == "Manual" || run_type == "Schedule");
    let pg_pool = &*crate::app_context::DB_POOL;
    let mut transaction = pg_pool.begin().await?;

    let pg_pool = &*crate::app_context::DB_POOL;
    let model_job = get_job_detail(pg_pool, job_id, team_id).await?.data;

    let status = "Running";

    let job_instance_id = sqlx::query!(
        "insert into idp_model_job_instance (job_id,job_name,team_id,project_id,user_id,nodes,edges,cron_config,runtime_config,run_type,status,run_node_type)
        values ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
        RETURNING job_instance_id
        ",
        model_job.job_id,
        model_job.job_name,
        model_job.team_id,
        model_job.project_id,
        model_job.user_id,
        model_job.nodes,
        model_job.edges,
        model_job.cron_config,
        model_job.runtime_config,
        run_type,
        status,
        format!("{run_node_type:?}")
    ).fetch_one(&mut transaction).await?.job_instance_id;

    let job_id = model_job.job_id;
    let project_id = model_job.project_id;

    let graph = Graph {
        nodes: serde_json::from_value(model_job.nodes).unwrap(),
        edges: serde_json::from_value(model_job.edges).unwrap(),
    };

    let graph_run_ctx = GraphRunCtx {
        job_id,
        job_instance_id,
        team_id,
        user_id,
        project_id,
        region: business::kubernetes::REGION.clone(),
        run_node_type,
        node_id,
    };
    let graph = graph.before_run(&graph_run_ctx, transaction).await?;
    tokio::spawn(async move {
        graph.run(graph_run_ctx).await;
    });

    tracing::info!("##job_run_dag running end");
    Ok(job_instance_id)
}

#[cfg(not)]
pub async fn model_job_save_to_model_job_instance(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    job_id: i32,
    team_id: i64,
    run_type: &str,
    run_node_type: &RunNodeType,
) -> Result<i32, ErrorTrace> {
    Ok(job_instance_id)
}

#[cfg(not)]
pub async fn job_run_dag(
    model_job_instance: ModelJobInstance,
    deploy_mode: &String,
) -> Result<Rsp<ModelJobInstanceDto>, ErrorTrace> {
}
