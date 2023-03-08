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

use axum::extract::Query;
use axum::Extension;
use axum::Json;
use common_model::Rsp;
use common_tools::schedule_tools::get_schedule_list;
use err::ErrorTrace;

use crate::pojo::component::component_dir::ComponentChild;
use crate::pojo::component::component_dir::ComponentDetail;
use crate::pojo::component::component_dir::ComponentDir;
use crate::pojo::component::component_dir::CronConfig;
use crate::pojo::component::component_dir::CronConfigFields;
use crate::pojo::component::component_dir::CronUpdateReq;
use crate::pojo::component::component_dir::DetailReq;
use crate::pojo::component::component_dir::InstanceDetailQto;
use crate::pojo::component::component_dir::JobDelReq;
use crate::pojo::component::component_dir::JobDetail;
use crate::pojo::component::component_dir::JobDetailReq;
use crate::pojo::component::component_dir::JobId;
use crate::pojo::component::component_dir::JobInfo;
use crate::pojo::component::component_dir::JobInsertReq;
use crate::pojo::component::component_dir::JobInstanceDetail;
use crate::pojo::component::component_dir::JobInstanceInfo;
use crate::pojo::component::component_dir::JobInstanceList;
use crate::pojo::component::component_dir::JobInstanceQto;
use crate::pojo::component::component_dir::JobList;
use crate::pojo::component::component_dir::JobListQto;
use crate::pojo::component::component_dir::JobUpdateReq;
use crate::pojo::component::component_dir::TempData;
use crate::pojo::component::component_dir::TempDelReq;
use crate::pojo::component::component_dir::TempDetail;
use crate::pojo::component::component_dir::TempDetailReq;
use crate::pojo::component::component_dir::TempId;
use crate::pojo::component::component_dir::TempInfo;
use crate::pojo::component::component_dir::TempInsertReq;
use crate::pojo::component::component_dir::TempListReq;
use crate::pojo::component::component_dir::TempUpdateReq;
use crate::pojo::component::graph::Graph;

pub async fn get_directory(
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<Vec<ComponentDir>>, ErrorTrace> {
    tracing::info!("[get_directory]run get_directory func");

    #[derive(sqlx::FromRow)]
    struct ComponentModel {
        component_key: String,
        component_label: String,
        cate_name: String,
        cata_id: i32,
        ports: serde_json::Value,
        meta: serde_json::Value,
        fields: serde_json::Value,
        params: serde_json::Value,
        resource: serde_json::Value,
    }
    let sql = r#"
        SELECT 
            base.key as component_key,
            base.label as component_label,
            ctg.category_name as cate_name,
            ctg.id as cata_id ,
            base.ports,
            base.meta,
            base.fields,
            base.params,
            base.resource
        FROM 
            idp_model_component_base as base 
            INNER JOIN idp_model_component_category as ctg 
                ON base.category_id = ctg.id 
        ORDER BY ctg.display_order , base.display_order asc"#;
    tracing::debug!("[get_directory]sql -> {}", sql);

    let component_model = sqlx::query_as::<_, ComponentModel>(sql)
        .fetch_all(&pg_pool)
        .await?;

    let mut id = 0;
    let length = component_model.len();
    let mut component_dir = ComponentDir::init();
    let mut component_dir_list: Vec<ComponentDir> = Vec::new();
    for (index, item) in component_model.into_iter().enumerate() {
        if id != item.cata_id {
            if id != 0 && index != length {
                component_dir_list.push(component_dir);
            }
            component_dir = ComponentDir::new(item.cata_id, item.cate_name);
        }
        component_dir.push(ComponentChild {
            key: item.component_key,
            name: item.component_label,
            children: Vec::new(),
            ports: item.ports,
            meta: item.meta,
            fields: item.fields,
            params: item.params,
            resource: item.resource,
        });
        id = item.cata_id
    }
    component_dir_list.push(component_dir);

    Ok(Rsp::success(component_dir_list))
}

pub async fn get_component_detail(
    Query(req): Query<DetailReq>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<ComponentDetail>, ErrorTrace> {
    let sql =
        r#"SELECT fields,params,resource,ports,meta FROM idp_model_component_base WHERE key = $1"#;

    let comment_detail = sqlx::query_as::<_, ComponentDetail>(sql)
        .bind(req.key)
        .fetch_one(&pg_pool)
        .await?;

    Ok(Rsp::success(comment_detail))
}

#[cfg(not)]
pub fn params_checker(
    nodes: serde_json::Value,
    edges: serde_json::Value,
) -> Result<(serde_json::Value, serde_json::Value), ErrorTrace> {
    tracing::info!("[params_checker]run params_checker func");
    let nodes: Vec<Node> = serde_json::from_value(nodes)?;
    tracing::debug!("[insert_job] nodes->{:#?}", nodes);
    if nodes.is_empty() {
        return Err(ErrorTrace::new("missing nodes"));
    }

    let nodes = serde_json::json!(nodes);

    let edges: Vec<Edge> = serde_json::from_value(edges)?;
    tracing::debug!("[insert_job] edges->{:#?}", edges);
    let edges = serde_json::json!(edges);
    Ok((nodes, edges))
}

pub async fn insert_job(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<JobInsertReq>,
) -> Result<Rsp<JobId>, ErrorTrace> {
    // let (nodes, edges) = params_checker(req.nodes, req.edges)?;
    let graph = Graph {
        nodes: req.nodes,
        edges: req.edges,
    };
    // graph.validate()?;
    let nodes = serde_json::to_value(&graph.nodes)?;
    let edges = serde_json::to_value(&graph.edges)?;

    let area = &crate::app_context::CONFIG.net_domain;

    let sql = r#"
        INSERT INTO 
            idp_model_job (job_name,team_id,project_id,user_id,nodes,edges,runtime_config,status,area)
        VALUES 
            ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        RETURNING 
            job_id
        "#;
    let job_id = sqlx::query_as::<_, JobId>(sql)
        .bind(req.job_name)
        .bind(req.team_id)
        .bind(req.project_id)
        .bind(req.user_id)
        .bind(nodes)
        .bind(edges)
        .bind(req.runtime_config)
        .bind(req.status)
        .bind(area)
        .fetch_one(&pg_pool)
        .await?;

    Ok(Rsp::success(job_id))
}

pub async fn update_job(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<JobUpdateReq>,
) -> Result<Rsp<JobId>, ErrorTrace> {
    tracing::info!("[update_job] run update_job func");

    let graph = Graph {
        nodes: req.nodes,
        edges: req.edges,
    };
    // graph.validate()?;

    let role_sql = "SELECT * FROM idp_model_job WHERE job_id = $1 and team_id = $2";

    tracing::debug!("[update_job] role_sql ->{}", role_sql);
    let rec = sqlx::query(role_sql)
        .bind(req.job_id)
        .bind(req.team_id)
        .fetch_one(&pg_pool)
        .await;
    if rec.is_err() {
        return Err(ErrorTrace::new("No permission"));
    }

    let sql = r#"
        UPDATE
            idp_model_job 
        SET 
            (job_name,team_id,project_id,user_id,nodes,edges,runtime_config,update_time,status)
            = ($1,$2,$3,$4,$5,$6,$7,CURRENT_TIMESTAMP,$8)
        WHERE 
            job_id = $9
        RETURNING job_id"#;
    tracing::info!("[update_job]runtime_config -> {:#?}", req.runtime_config);
    let nodes = serde_json::to_value(&graph.nodes)?;
    let edges = serde_json::to_value(&graph.edges)?;
    let rec = sqlx::query_as::<_, JobId>(sql)
        .bind(req.job_name)
        .bind(req.team_id)
        .bind(req.project_id)
        .bind(req.user_id)
        .bind(nodes)
        .bind(edges)
        .bind(req.runtime_config)
        .bind(req.status)
        .bind(req.job_id)
        .fetch_one(&pg_pool)
        .await?;

    Ok(Rsp::success(rec))
}

pub async fn update_job_schedule(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<CronUpdateReq>,
) -> Result<Rsp<Vec<String>>, ErrorTrace> {
    let mut next_crons_time = Vec::new();

    if req.cron_config.is_some() && req.status.is_some() {
        let sql = r#"
        UPDATE
            idp_model_job
        SET
            (cron_config, status , update_time)
            = ($1, $2, CURRENT_TIMESTAMP)
        WHERE
            job_id = $3 and team_id = $4
        RETURNING job_id"#;
        let _rec = match sqlx::query_as::<_, JobId>(sql)
            .bind(&req.cron_config)
            .bind(&req.status)
            .bind(req.job_id)
            .bind(req.team_id)
            .fetch_one(&pg_pool)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(ErrorTrace::new(err.to_string().as_str())),
        };

        let cron_config_fields =
            serde_json::from_value::<CronConfigFields>(req.cron_config.unwrap()).unwrap();
        let cron_expression = cron_config_fields.cron_expression;
        next_crons_time = get_schedule_list(cron_expression);
    } else if req.cron_config.is_some() {
        let sql = r#"
        UPDATE
            idp_model_job
        SET
            (cron_config, update_time)
            = ($1,CURRENT_TIMESTAMP)
        WHERE
            job_id = $2 and team_id = $3
        RETURNING job_id"#;
        let _rec = match sqlx::query_as::<_, JobId>(sql)
            .bind(&req.cron_config)
            .bind(req.job_id)
            .bind(req.team_id)
            .fetch_one(&pg_pool)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(ErrorTrace::new(err.to_string().as_str())),
        };

        let cron_config_fields =
            serde_json::from_value::<CronConfigFields>(req.cron_config.unwrap()).unwrap();
        let cron_expression = cron_config_fields.cron_expression;
        next_crons_time = get_schedule_list(cron_expression);
    } else if req.status.is_some() {
        let sql = r#"
        UPDATE
            idp_model_job
        SET
            (update_time,status)
            = (CURRENT_TIMESTAMP,$1)
        WHERE
            job_id = $2 and team_id = $3
        RETURNING job_id"#;
        let _rec = match sqlx::query_as::<_, JobId>(sql)
            .bind(&req.status)
            .bind(req.job_id)
            .bind(req.team_id)
            .fetch_one(&pg_pool)
            .await
        {
            Ok(data) => data,
            Err(err) => return Err(ErrorTrace::new(err.to_string().as_str())),
        };
    };

    // for x in next_crons_time {
    //     println!("{}", x);
    // }
    // {
    //     "code": 200,
    //     "message": "success",
    //     "data": ["2023-02-10 12:07:00", "2023-02-10 12:08:00", "2023-02-10 12:09:00", "2023-02-10 12:10:00", "2023-02-10 12:11:00"]
    // }

    Ok(Rsp::success(next_crons_time))
}

pub async fn insert_template(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<TempInsertReq>,
) -> Result<Rsp<TempId>, ErrorTrace> {
    let sql = r#"
        INSERT INTO 
            idp_model_job_template (job_template_name,team_id,project_id,user_id,nodes,edges,cron_config,runtime_config)
        VALUES 
            ($1,$2,$3,$4,$5,$6,$7,$8)
        RETURNING 
            id as template_id
        "#;
    let temp_id = sqlx::query_as::<_, TempId>(sql)
        .bind(req.job_name)
        .bind(req.team_id)
        .bind(req.project_id)
        .bind(req.user_id)
        .bind(req.nodes)
        .bind(req.edges)
        .bind(req.cron_config)
        .bind(req.runtime_config)
        .fetch_one(&pg_pool)
        .await?;

    Ok(Rsp::success(temp_id))
}

pub async fn update_template(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<TempUpdateReq>,
) -> Result<Rsp<TempId>, ErrorTrace> {
    tracing::info!("[update_template] run update_template func");
    let role_sql = "SELECT * FROM idp_model_job_template WHERE id = $1 and team_id = $2";

    tracing::debug!("[update_template] role_sql ->{}", role_sql);
    let rec = sqlx::query(role_sql)
        .bind(req.template_id)
        .bind(req.team_id)
        .fetch_one(&pg_pool)
        .await;
    if rec.is_err() {
        return Err(ErrorTrace::new("No permission"));
    }

    let sql = r#"
        UPDATE
            idp_model_job_template 
        SET 
            (job_template_name,team_id,project_id,user_id,nodes,edges,cron_config,runtime_config,update_time)
            = ($1,$2,$3,$4,$5,$6,$7,$8,CURRENT_TIMESTAMP)
        WHERE 
            id = $9
        RETURNING job_id"#;
    let rec = sqlx::query_as::<_, TempId>(sql)
        .bind(req.temp_name)
        .bind(req.team_id)
        .bind(req.project_id)
        .bind(req.user_id)
        .bind(req.nodes)
        .bind(req.edges)
        .bind(req.cron_config)
        .bind(req.runtime_config)
        .bind(req.template_id)
        .fetch_one(&pg_pool)
        .await?;

    Ok(Rsp::success(rec))
}

pub async fn delete_template(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<TempDelReq>,
) -> Result<Rsp<TempId>, ErrorTrace> {
    tracing::info!("[delete_template] run delete_template func");

    let sql = r#"
        UPDATE
            idp_model_job_template 
        SET 
            del_flag = 1
        WHERE 
            id = $1 and team_id = $2
        RETURNING id as template_id"#;
    let rec = match sqlx::query_as::<_, TempId>(sql)
        .bind(req.template_id)
        .bind(req.team_id)
        .fetch_one(&pg_pool)
        .await
    {
        Ok(data) => data,
        Err(_) => return Err(ErrorTrace::new("no permission")),
    };

    Ok(Rsp::success(rec))
}

pub async fn get_template_list(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<TempListReq>,
) -> Result<Rsp<Vec<TempInfo>>, ErrorTrace> {
    #[derive(sqlx::FromRow)]
    struct TempModel {
        template_id: i32,
        job_template_name: String,
        nodes: serde_json::Value,
        edges: serde_json::Value,
        cron_config: serde_json::Value,
        runtime_config: serde_json::Value,
        area: String,
    }

    let sql = r#"
        SELECT 
            id as template_id,
            job_template_name,
            nodes,
            edges,
            cron_config,
            runtime_config,
            area
        FROM
            idp_model_job_template
        WHERE team_id = $1 and del_flag = 0
     "#;
    let rec = sqlx::query_as::<_, TempModel>(sql)
        .bind(req.team_id)
        .fetch_all(&pg_pool)
        .await?;

    let mut remp_info_vec: Vec<TempInfo> = Vec::new();
    for item in rec.into_iter() {
        let temp_data = TempData {
            nodes: item.nodes,
            edges: item.edges,
            cron_config: item.cron_config,
            runtime_config: item.runtime_config,
            area: item.area,
        };
        let temp_info = TempInfo {
            key: item.template_id.to_string(),
            name: item.job_template_name,
            temp_data,
        };
        remp_info_vec.push(temp_info)
    }

    Ok(Rsp::success(remp_info_vec))
}

pub async fn get_template_detail(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<TempDetailReq>,
) -> Result<Rsp<TempDetail>, ErrorTrace> {
    let sql = r#"
        SELECT 
            id as template_id,
            team_id,
            job_template_name,
            nodes,
            edges,
            cron_config,
            runtime_config
        FROM 
            idp_model_job_template 
        WHERE 
            id = $1
     "#;
    let rec = match sqlx::query_as::<_, TempDetail>(sql)
        .bind(req.template_id)
        .fetch_one(&pg_pool)
        .await
    {
        Ok(data) => data,
        Err(_) => return Err(ErrorTrace::new("no such template in database")),
    };

    if rec.team_id != req.team_id {
        return Err(ErrorTrace::new("no permission"));
    }

    Ok(Rsp::success(rec))
}

pub async fn get_job_detail_handler(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<JobDetailReq>,
) -> Result<Rsp<JobDetail>, ErrorTrace> {
    let job_id = req.job_id;
    let team_id = req.team_id;

    get_job_detail(&pg_pool, job_id, team_id).await
}

pub async fn get_job_detail(
    pg_pool: &sqlx::PgPool,
    job_id: i32,
    team_id: i64,
) -> Result<Rsp<JobDetail>, ErrorTrace> {
    let sql = r#"
        SELECT
            job_id,
            job_name,nodes,edges,
            team_id,project_id,user_id,
            cron_config,runtime_config,status,
            create_time,update_time
        FROM
            idp_model_job
        WHERE
            job_id = $1 and team_id = $2"#;

    let mut rec = sqlx::query_as::<_, JobDetail>(sql)
        .bind(job_id)
        .bind(team_id)
        .fetch_one(pg_pool)
        .await?;
    rec.set_time(pg_pool, Some(team_id)).await;

    Ok(Rsp::success(rec))
}

pub async fn get_job_list(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<JobListQto>,
) -> Result<Rsp<JobList>, ErrorTrace> {
    tracing::info!("[get_job_list] run get_job_list func");

    let status_sql = if let Some(status) = req.status {
        format!(" and status = '{status}'")
    } else {
        "".to_string()
    };
    let sort_field = req.sort_field.unwrap_or_else(|| "job_id".to_string());
    let sort = req.sort.unwrap_or_else(|| "desc".to_string());
    let skip = req.size * (req.current - 1);
    let size = req.size;
    let search_info = req.search_info.unwrap_or_default();

    let cnt_sql = format!(
        r#"
        SELECT count(*)
        FROM 
            idp_model_job 
        WHERE 
            concat(job_id,job_name) LIKE '%{}%' 
            and del_flag = 0 and team_id = $1 {}
        "#,
        search_info.clone(),
        status_sql
    );
    let total_model = sqlx::query_as::<_, (i64,)>(&cnt_sql)
        .bind(req.team_id)
        .fetch_one(&pg_pool);
    tracing::debug!("[get_job_list]cnt_sql->{}", cnt_sql);

    let job_sql = format!(
        r#"
        SELECT 
            job_id,job_name,status,cron_config,create_time,update_time,
            null as cron_expression_disp,null as cron_start_date,null as cron_end_date
        FROM 
            idp_model_job
        WHERE 
            concat(job_id,job_name) LIKE '%{}%'
            and del_flag = 0 and team_id = $1 {}
        ORDER BY {} {}
        OFFSET $2 LIMIT $3
        "#,
        search_info, status_sql, sort_field, sort
    );
    let job_model = sqlx::query_as::<_, JobInfo>(&job_sql)
        .bind(req.team_id)
        .bind(skip)
        .bind(req.size)
        .fetch_all(&pg_pool);
    tracing::debug!("[get_job_list]job_sql->{}", job_sql);

    let (total, job_info) = futures::future::join(total_model, job_model).await;
    let total = total?.0 as i32;
    let mut pages = if total % size == 0 {
        total / size
    } else {
        total / size + 1
    };
    if pages == 0 {
        pages = 1
    }

    let mut job_info = job_info?;
    for item in job_info.iter_mut() {
        item.set_time(&pg_pool, Some(req.team_id)).await;
        item.set_cron_relation_fields().await;
    }

    Ok(Rsp::success(JobList {
        data: job_info,
        size,
        current: req.current,
        total,
        pages,
    }))
}

pub async fn get_job_instance_list(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<JobInstanceQto>,
) -> Result<Rsp<JobInstanceList>, ErrorTrace> {
    tracing::debug!("[get_job_instance_list] run get_job_instance_list func");

    let status_sql = if req.status.is_none() {
        "".to_string()
    } else {
        let status = req.status.unwrap();
        let status_vec: Vec<&str> = status.split(',').collect();
        if status_vec.is_empty() {
            "".to_string()
        } else {
            let mut status_sql = format!(" and status = '{:#?}'", status_vec.first());
            for item in status_vec.into_iter().skip(1) {
                status_sql = format!("{} or status = '{}'", status_sql, item);
            }
            status_sql
        }
    };

    let sort_field = req
        .sort_field
        .unwrap_or_else(|| "job_instance_id".to_string());
    let sort = req.sort.unwrap_or_else(|| "desc".to_string());
    let skip = req.size * (req.current - 1);
    let size = req.size;
    let search_info = req.search_info.unwrap_or_default();

    let cnt_sql = format!(
        r#"
        SELECT count(*)
        FROM 
            idp_model_job_instance 
        WHERE 
            concat(job_instance_id,job_name) LIKE '%{}%' 
            and del_flag = 0 and team_id = $1 {}
        "#,
        search_info.clone(),
        status_sql
    );
    let total_model = sqlx::query_as::<_, (i64,)>(&cnt_sql)
        .bind(req.team_id)
        .fetch_one(&pg_pool);
    tracing::debug!("[get_job_instance_list]cnt_sql->{}", cnt_sql);

    let instance_sql = format!(
        r#"
        SELECT 
            job_instance_id,job_name,run_type,status,null as running_time,start_time,end_time
        FROM 
            idp_model_job_instance
        WHERE 
            concat(job_instance_id,job_name) LIKE '%{}%'
            and del_flag = 0 and team_id = $1 {}
        ORDER BY {} {}
        OFFSET $2 LIMIT $3
        for share
        "#,
        search_info, status_sql, sort_field, sort
    );
    let instance_model = sqlx::query_as::<_, JobInstanceInfo>(&instance_sql)
        .bind(req.team_id)
        .bind(skip)
        .bind(req.size)
        .fetch_all(&pg_pool);
    tracing::debug!("[get_job_instance_list]job_sql->{}", instance_sql);

    let (total, instance_info) = futures::future::join(total_model, instance_model).await;
    let total = total?.0 as i32;
    let mut pages = if total % size == 0 {
        total / size
    } else {
        total / size + 1
    };
    if pages == 0 {
        pages = 1
    }

    let mut instance_info = instance_info?;
    for item in instance_info.iter_mut() {
        let duration = item
            .end_time
            .signed_duration_since(item.start_time)
            .num_seconds();
        item.running_time = Some(duration);
        item.set_time(&pg_pool, Some(req.team_id)).await;
    }

    Ok(Rsp::success(JobInstanceList {
        data: instance_info,
        size,
        current: req.current,
        total,
        pages,
    }))
}

pub async fn delete_job(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Json(req): Json<JobDelReq>,
) -> Result<Rsp<i32>, ErrorTrace> {
    let update_job_sql = format!(
        r#"
        UPDATE 
            idp_model_job 
        SET 
            del_flag = 1
        WHERE 
            job_id = {} and team_id = {}
        RETURNING
            job_id
            "#,
        req.job_id, req.team_id
    );
    let job_model = sqlx::query_as::<_, (i32,)>(&update_job_sql).fetch_one(&pg_pool);

    let update_instance_sql = format!(
        r#"
        UPDATE
            idp_model_job_instance
        SET 
            del_flag = 1
        WHERE 
            job_id = {} and team_id = {}"#,
        req.job_id, req.team_id
    );
    let instance_model = sqlx::query(&update_instance_sql).execute(&pg_pool);

    let (job_id, res) = futures::future::join(job_model, instance_model).await;
    let job_id = job_id?.0;
    let _res = res?;

    Ok(Rsp::success(job_id))
}

pub async fn get_job_instance_detail(
    Extension(pg_pool): Extension<sqlx::PgPool>,
    Query(req): Query<InstanceDetailQto>,
) -> Result<Rsp<JobInstanceDetail>, ErrorTrace> {
    let sql = format! {r#"
        SELECT
            job_instance_id,job_id,job_name,
            nodes,edges,
            cron_config,runtime_config,
            run_type,
            nodes_status,status,
            start_time,end_time,create_time,update_time,area
        FROM 
            idp_model_job_instance
        WHERE
            job_instance_id = {} and team_id = {}
        FOR SHARE"#,
    req.job_instance_id,
    req.team_id};
    tracing::debug!("[get_job_instance_detail]sql -> {}", sql);
    let mut res = sqlx::query_as::<_, JobInstanceDetail>(&sql)
        .fetch_one(&pg_pool)
        .await?;
    res.set_time(&pg_pool, Some(req.team_id)).await;
    Ok(Rsp::success(res))
}

pub async fn get_time_list(Query(req): Query<CronConfig>) -> Result<Rsp<Vec<String>>, ErrorTrace> {
    let next_crons_time = get_schedule_list(req.cron_expression);
    // {
    //     "code": 200,
    //     "message": "success",
    //     "data": ["2023-02-10 12:07:00", "2023-02-10 12:08:00", "2023-02-10 12:09:00", "2023-02-10 12:10:00", "2023-02-10 12:11:00"]
    // }
    Ok(Rsp::success(next_crons_time))
}
