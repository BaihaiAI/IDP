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

use std::fmt;

use chrono::NaiveDateTime;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::pojo::time_tool::change_tz;
use crate::pojo::time_tool::OperateFlag;

pub mod idp_k8s;
pub mod service_status;
pub use idp_k8s::*;
pub use service_status::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployServiceReq {
    pub service_type: i32,
    pub service_name: String,
    pub service_intro: Option<String>,
    pub package_id: Option<i32>,
    pub edition_id: Option<i32>,
    pub image: String,
    pub image_warehouse: Option<String>,
    pub pod_memory: Option<i32>,
    pub pod_cpu: Option<i32>,
    pub pod_gpu: Option<i32>,
    pub instance_count: Option<i32>,
    pub env: Option<String>,
    // pub cron: Option<String>,
    pub cron_config: Option<serde_json::Value>,
    pub support_system: Option<String>,
    pub support_chip: Option<String>,
    pub equipments: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceReq {
    pub service_id: i32,
    pub service_type: i32,
    pub service_name: String,
    pub service_intro: Option<String>,
    pub package_id: Option<i32>,
    pub edition_id: Option<i32>,
    pub image: String,
    pub image_warehouse: Option<String>,
    pub pod_memory: Option<i32>,
    pub pod_cpu: Option<i32>,
    pub pod_gpu: Option<i32>,
    pub instance_count: Option<i32>,
    pub env: Option<Value>,
    // pub cron: Option<String>,
    pub cron_config: Option<serde_json::Value>,
    pub equipments: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceListReq {
    pub page_size: i32,
    pub page_index: i32,
    pub service_type: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceReq {
    pub service_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEquipmentReq {
    pub service_id: i32,
    pub page_size: i32,
    pub page_index: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReq {
    pub service_id: i32,
    pub pod_cpu: Option<i32>,
    pub pod_gpu: Option<i32>,
    pub pod_memory: Option<i32>,
    pub instance_count: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildImageReq {
    pub service_id: i32,
    pub image: String,
    pub edition_id: Option<i32>,
    pub package_id: Option<i32>,
    pub env: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronReq {
    pub service_id: i32,
    // pub cron: String,
    pub cron_config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubeedgeJobReq {
    pub service_id: i32,
    pub equipment: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UPdateKubeedgeJobReq {
    pub service_id: i32,
    pub equipment: String,
    pub edition_id: i32,
    pub package_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollBackJobReq {
    #[serde(deserialize_with = "serde_helper::de_i64_from_str")]
    pub equipment_id: i64,
    pub service_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicInfoReq {
    pub service_id: i32,
    pub service_name: String,
    pub intro: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceListRsp {
    pub service_info: Vec<ServiceInfo>,
    pub pages: i32,
    pub total: i32,
    pub size: i32,
    pub current: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentListRsp {
    pub equipment_info: Vec<EquipmentInfo>,
    pub pages: i32,
    pub total: i32,
    pub size: i32,
    pub current: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EditionInfo {
    pub location: String,
    pub runtime_env: String,
    pub team_id: i64,
    pub project_id: i32,
    pub image: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInfo {
    pub equipment_id: i32,
    pub equipment_status: String,
    pub equipment_name: String,
    pub serial_number: String,
    pub memo: Option<String>,
    pub update_time: NaiveDateTime,
    pub pod_status: Option<String>,
    pub status_message: Option<String>,
    pub is_latest: bool,
    pub model: Option<String>,
}

impl EquipmentInfo {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.update_time = change_tz(self.update_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.update_time);
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub id: i32,
    pub team_id: i64,
    pub user_id: i64,
    pub service_intro: Option<String>,
    pub service_name: String,
    pub status: String,
    pub status_message: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub package_id: Option<i32>,
    pub package_intro: Option<String>,
    pub package_name: Option<String>,
    pub edition_id: Option<i32>,
    pub edition_version: Option<String>,
    pub testing_input: Option<String>,
    pub cron_config: Option<serde_json::Value>,
    pub instance_count: Option<i32>,
    pub last_run_time: Option<NaiveDateTime>,
    pub pod_cpu: Option<i32>,
    pub pod_gpu: Option<i32>,
    pub pod_memory: Option<i32>,
    pub image: String,
    pub url: Option<String>,
    pub id_token: Option<String>,
    pub env: Option<Value>,
    pub service_type: i32,
    pub equipments: Option<Vec<String>>,
    pub support_system: Option<String>,
    pub support_chip: Option<String>,
    pub input_types: Option<String>,
    pub output_types: Option<String>,
}
impl ServiceInfo {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.create_time = change_tz(self.create_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.create_time);
        self.update_time = change_tz(self.update_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.update_time);
        if let Some(last_run_time) = self.last_run_time {
            self.last_run_time = Some(
                change_tz(last_run_time, team_id, pg_pool, OperateFlag::Switch)
                    .await
                    .unwrap_or(last_run_time),
            );
        }
    }
    pub async fn set_some_field(&mut self, id_token: Option<String>) {
        let area = &crate::app_context::CONFIG.net_domain as &str;
        let uri = "/0/api/v1/model-service/deploy/run-request?mid=";
        let mut tmp = "".to_string();
        tmp += area;
        tmp += uri;
        tmp += self.id.to_string().as_str();
        self.url = Some(tmp);

        self.id_token = id_token;
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct CheckServiceInfo {
    pub id: i32,
    pub service_type: i32,
    pub cron_config: Option<serde_json::Value>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CheckBatchServiceInfo {
    pub id: i32,
    pub team_id: i64,
    pub user_id: i64,
    pub cron_config: Option<serde_json::Value>,
    // pub cron_config: Option<serde_json::Value>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CheckPodInfo {
    pub id: i64,
    pub service_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationRsp {
    pub operation_info: Vec<OperationInfo>,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OperationInfo {
    pub team_id: i64,
    pub creator_id: i64,
    pub create_time: NaiveDateTime,
    pub content: Value,
}

impl OperationInfo {
    pub async fn set_time(&mut self, pg_pool: &sqlx::PgPool, team_id: Option<i64>) {
        self.create_time = change_tz(self.create_time, team_id, pg_pool, OperateFlag::Switch)
            .await
            .unwrap_or(self.create_time);
    }
}

pub struct ServiceType {}
impl ServiceType {
    pub fn to_string(r#type: i32) -> String {
        match r#type {
            1 => "Deployment".to_string(),
            2 => "CronJob".to_string(),
            3 => "Kubeedge".to_string(),
            _ => "".to_string(),
        }
    }
    pub fn legal(r#type: i32) -> bool {
        matches!(r#type, 1 | 2 | 3)
    }
}

pub enum SupportConstant {
    PYTHON38,
    PYTHON39,
    PYTHON310,
    JAVA8,
    JAVA11,
    JAVA17,
}
impl SupportConstant {
    pub fn bin_path(&self) -> String {
        match self {
            SupportConstant::PYTHON38 => "/opt/miniconda/envs/python38/bin/pip".to_string(),
            SupportConstant::PYTHON39 => "/opt/miniconda/envs/python39/bin/pip".to_string(),
            SupportConstant::PYTHON310 => "/opt/miniconda/envs/python310/bin/pip".to_string(),
            SupportConstant::JAVA8 => todo!(),
            SupportConstant::JAVA11 => todo!(),
            SupportConstant::JAVA17 => todo!(),
        }
    }
    pub fn use_python(&self) -> bool {
        matches!(
            self,
            SupportConstant::PYTHON38 | SupportConstant::PYTHON39 | SupportConstant::PYTHON310
        )
    }
    pub fn get_enum(constant: &str) -> Result<SupportConstant, ErrorTrace> {
        match constant.to_ascii_uppercase().as_str() {
            "PYTHON 3.8" | "IDP:PYTHON38" => Ok(SupportConstant::PYTHON38),
            "PYTHON 3.9" | "IDP:PYTHON39" => Ok(SupportConstant::PYTHON39),
            "PYTHON 3.10" | "IDP:PYTHON310" => Ok(SupportConstant::PYTHON310),
            "JAVA8" => Ok(SupportConstant::JAVA8),
            "JAVA11" => Ok(SupportConstant::JAVA11),
            "JAVA17" => Ok(SupportConstant::JAVA17),
            _ => Err(ErrorTrace::new("Don't support")),
        }
    }
}

pub enum ServiceStatus {
    Deploying,
    Normal,
    Abnormal,
    Stop,
    PreSchedule, //预定时
    _Schedule,   //定时服务
    _Alarm,
    ContainerCreating,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceStatus::Deploying => write!(f, "deploying"),
            ServiceStatus::Normal => write!(f, "normal"),
            ServiceStatus::Abnormal => write!(f, "abnormal"),
            ServiceStatus::Stop => write!(f, "stop"),
            ServiceStatus::PreSchedule => write!(f, "PreSchedule"),
            ServiceStatus::_Schedule => write!(f, "Schedule"),
            ServiceStatus::_Alarm => write!(f, "alarm"),
            ServiceStatus::ContainerCreating => write!(f, "containerCreating"),
        }
    }
}

pub enum ServiceOperation {
    Renew,
    Deploy,
    Start,
    Stop,
    Delete,
}

impl fmt::Display for ServiceOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceOperation::Renew => write!(f, "Renew"),
            ServiceOperation::Deploy => write!(f, "Deploy"),
            ServiceOperation::Start => write!(f, "Start"),
            ServiceOperation::Stop => write!(f, "Stop"),
            ServiceOperation::Delete => write!(f, "Delete"),
        }
    }
}

#[cfg(not)]
pub enum TaskHistoryStatus {
    Success,
    Running,
    Fail,
}
#[cfg(not)]
impl fmt::Display for TaskHistoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskHistoryStatus::Success => write!(f, "Success"),
            TaskHistoryStatus::Running => write!(f, "Running"),
            TaskHistoryStatus::Fail => write!(f, "Fail"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceListQto {
    pub serial_number: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ServiceModel {
    pub service_id: String,
    pub service_name: String,
    pub service_intro: Option<String>,
    pub model_name: Option<String>,
    pub version: Option<String>,
    pub latest_edition: Option<String>,
    pub status: Option<String>,
    pub status_message: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ServiceList {
    pub service_id: String,
    pub service_name: String,
    pub service_intro: Option<String>,
    pub model: String,
    pub update_flag: bool,
    pub status: Option<String>,
    pub status_message: Option<String>,
}

impl ServiceList {
    pub fn init(service_model: ServiceModel) -> ServiceList {
        let update_flag = service_model.version != service_model.latest_edition;
        let model = format!(
            "{:?} ({:?})",
            service_model.model_name, service_model.version
        );
        ServiceList {
            service_id: service_model.service_id,
            service_name: service_model.service_name,
            service_intro: service_model.service_intro,
            model,
            update_flag,
            status: service_model.status,
            status_message: service_model.status_message,
        }
    }
}
