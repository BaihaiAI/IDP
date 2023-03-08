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

use super::time_tool::change_tz;
use super::time_tool::OperateFlag;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentListQto {
    pub size: i32,
    pub current: i32,
    pub search_info: Option<String>,
    pub status: Option<String>,
    pub sort_field: Option<String>,
    pub sort: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentList {
    pub data: Vec<EquipmentInfo>,
    pub size: i32,
    pub current: i32,
    pub total: i32,
    pub pages: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentInfo {
    pub equipment_id: i32,
    pub equipment_name: String,
    pub status: String,
    pub serial_number: String,
    pub memo: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl EquipmentInfo {
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
pub struct EquipmentDelReq {
    pub equipment_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipInsertReq {
    pub equipment_name: String,
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StatusRsp {
    pub code: u32,
    pub message: String,
    pub data: Vec<StatusRspData>,
}

#[derive(Serialize, Deserialize)]
pub struct StatusRspData {
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateReq {
    pub equipment_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TokenRsp {
    pub code: u32,
    pub message: String,
    pub data: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipDetailQto {
    pub equip_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub equipment_id: i32,
    pub equipment_name: Option<String>,
    pub memo: Option<String>,
}
