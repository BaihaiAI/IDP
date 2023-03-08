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

use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

use super::graph::Edge;
use super::graph::Node;
use crate::pojo::time_tool::change_tz;
use crate::pojo::time_tool::OperateFlag;
use crate::service::schedule::schedule_utils::translate_to_chinese;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDelReq {
    pub job_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobCronConfig {
    pub job_id: i32,
    pub team_id: i64,
    // pub user_id: i64,
    pub cron_config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronConfigFields {
    pub cron_type: String,
    pub cron_expression: String,
    pub cron_start_date: String,
    pub cron_end_date: String,
    pub cron_start_time: Option<String>,
    pub cron_end_time: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobInstanceList {
    pub data: Vec<JobInstanceInfo>,
    pub size: i32,
    pub current: i32,
    pub total: i32,
    pub pages: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobInstanceInfo {
    pub job_instance_id: i32,
    pub job_name: String,
    pub run_type: String,
    pub status: String,
    pub running_time: Option<i64>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

impl JobInstanceInfo {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.start_time = change_tz(self.start_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.start_time);
        self.end_time = change_tz(self.end_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.end_time);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobInstanceQto {
    pub size: i32,
    pub current: i32,
    pub search_info: Option<String>,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub status: Option<String>,
    pub sort_field: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceDetailQto {
    pub job_instance_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobList {
    pub data: Vec<JobInfo>,
    pub size: i32,
    pub current: i32,
    pub total: i32,
    pub pages: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobInfo {
    pub job_id: i32,
    pub job_name: String,
    pub status: String,
    pub cron_config: serde_json::Value,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,

    pub cron_expression_disp: Option<String>,
    pub cron_start_date: Option<String>,
    pub cron_end_date: Option<String>,
}

impl JobInfo {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.create_time = change_tz(self.create_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.create_time);
        self.update_time = change_tz(self.update_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.update_time);
    }

    pub async fn set_cron_relation_fields(&mut self) {
        let cron_config = self.cron_config.take();
        if cron_config.to_string().len() > 10 {
            let obj: CronConfigFields = serde_json::from_value(cron_config).unwrap();
            tracing::info!("!!obj.cron_expression={:?}", obj.cron_expression);
            if !obj.cron_expression.is_empty() {
                let mut cron_exp = "".to_string();
                cron_exp += "0 ";
                cron_exp += obj.cron_expression.as_str();
                let cron_expression_disp_value = translate_to_chinese(cron_exp).await;
                self.cron_expression_disp = Some(cron_expression_disp_value);

                let mut temp_start = "".to_string();
                temp_start += obj.cron_start_date.as_str();
                temp_start += " 00:00:00";

                let mut temp_end = "".to_string();
                temp_end += obj.cron_end_date.as_str();
                temp_end += " 00:00:00";

                self.cron_start_date = Some(temp_start);
                self.cron_end_date = Some(temp_end);
            } else {
                self.cron_expression_disp = Some("".to_string());
            }
        } else {
            self.cron_expression_disp = Some("".to_string());
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListQto {
    pub size: i32,
    pub current: i32,
    pub search_info: Option<String>,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub status: Option<String>,
    pub sort_field: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobDetail {
    pub job_id: i32,
    pub team_id: i64,
    pub project_id: i32,
    pub user_id: i64,
    pub job_name: String,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub status: String,
}

impl JobDetail {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.create_time = change_tz(self.create_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.create_time);
        self.update_time = change_tz(self.update_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.update_time);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobDetailReq {
    pub job_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempInfo {
    //template_id
    pub key: String,
    //job_template_name
    pub name: String,
    pub temp_data: TempData,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempData {
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
    pub area: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempDetail {
    pub template_id: i32,
    pub team_id: i64,
    pub job_template_name: String,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempListReq {
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempDetailReq {
    pub template_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempUpdateReq {
    pub template_id: i32,
    pub temp_name: String,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempDelReq {
    pub template_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempId {
    pub template_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobUpdateReq {
    pub job_id: i32,
    pub job_name: String,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub runtime_config: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronUpdateReq {
    pub job_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub cron_config: Option<serde_json::Value>,
    pub status: Option<String>,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CronUpdateReq {
//
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobInsertReq {
    pub job_name: String,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub runtime_config: serde_json::Value,
    pub status: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TempInsertReq {
    pub job_name: String,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub team_id: i64,
    pub project_id: i32,
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub user_id: i64,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobId {
    pub job_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailReq {
    pub key: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDetail {
    pub fields: serde_json::Value,
    pub params: serde_json::Value,
    pub ports: serde_json::Value,
    pub resource: serde_json::Value,
    pub meta: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDir {
    pub key: String,
    pub name: String,
    pub ports: serde_json::Value,
    pub meta: serde_json::Value,
    pub fields: serde_json::Value,
    pub params: serde_json::Value,
    pub resource: serde_json::Value,
    pub children: Vec<ComponentChild>,
}

impl ComponentDir {
    pub fn init() -> Self {
        Self {
            key: 0.to_string(),
            name: "".to_string(),
            children: Vec::new(),
            ports: serde_json::json!({}),
            meta: serde_json::json!({}),
            fields: serde_json::json!({}),
            params: serde_json::json!({}),
            resource: serde_json::json!({}),
        }
    }
    pub fn new(key: i32, name: String) -> Self {
        let key = key.to_string();
        let children: Vec<ComponentChild> = Vec::new();
        Self {
            key,
            name,
            children,
            ports: serde_json::json!({}),
            meta: serde_json::json!({}),
            fields: serde_json::json!({}),
            params: serde_json::json!({}),
            resource: serde_json::json!({}),
        }
    }

    pub fn push(&mut self, new_child: ComponentChild) {
        self.children.push(new_child)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentChild {
    pub key: String,
    pub name: String,
    pub ports: serde_json::Value,
    pub meta: serde_json::Value,
    pub fields: serde_json::Value,
    pub params: serde_json::Value,
    pub resource: serde_json::Value,
    pub children: Vec<ComponentChild>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelJobInstanceReq {
    pub job_instance_id: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobInstanceId {
    pub job_instance_id: i32,
}

#[cfg(not)]
#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ModelJobInstance {
    pub job_instance_id: i32,
    pub job_id: i32,
    pub job_name: String,
    pub team_id: i64,
    pub project_id: i32,
    pub user_id: i64,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobInstanceDetail {
    pub job_instance_id: i32,
    pub job_id: i32,
    pub job_name: String,
    pub nodes: serde_json::Value,
    pub edges: serde_json::Value,
    pub cron_config: serde_json::Value,
    pub runtime_config: serde_json::Value,
    pub run_type: String,
    pub nodes_status: serde_json::Value,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub area: String,
}

impl JobInstanceDetail {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.start_time = change_tz(self.start_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.start_time);
        self.end_time = change_tz(self.end_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.end_time);
        self.create_time = change_tz(self.create_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.create_time);
        self.update_time = change_tz(self.update_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.update_time);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronConfig {
    pub cron_end_date: String,
    pub cron_expression: String,
    pub cron_start_date: String,
    pub cron_type: String,
}
