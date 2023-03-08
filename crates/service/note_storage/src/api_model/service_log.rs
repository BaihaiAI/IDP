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

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogListQto {
    pub service_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub data: Vec<SeriesInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeriesInfo {
    pub filename: String,
    pub job: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQto {
    pub service_id: i32,
    pub file_name: String,
    pub job: String,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogsRsp {
    pub data: LogResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogResult {
    pub result: Vec<LogInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogInfo {
    pub stream: serde_json::Value,
    //contains file_name & job
    pub values: serde_json::Value,
    //0 represents timestamp ,1 represents log content
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PodListRsp {
    pub data: Vec<PodInfoModel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PodInfoModel {
    pub instance: Option<String>,
    pub job: Option<String>,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub pod: String,
    pub pod_ip: Option<String>,
    pub uid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerListRsp {
    pub data: Vec<ContainerInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerInfo {
    pub container: String,
    pub namespace: String,
    pub pod: String,
    pub uid: String,
}

#[derive(Debug, Serialize)]
pub struct QueryForm {
    pub queries: Vec<QueryItem>,
    pub range: QueryRange,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryItem {
    pub expr: String,
    pub format: String,
    pub instant: Option<bool>,
    pub interval: String,
    pub interval_factor: i64,
    pub legend_format: String,
    pub ref_id: String,
    pub step: i64,
    pub datasource: DataSource,
    pub query_type: String,
    pub exemplar: bool,
    pub request_id: String,
    pub utc_offset_sec: i64,
    pub datasource_id: i64,
    pub interval_ms: i64,
    pub max_data_points: i64,
    pub metric: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct DataSource {
    pub uid: String,
    pub r#type: String,
}

#[derive(Debug, Serialize)]
pub struct QueryRange {
    //"from":"2023-02-25T08:33:35.233Z",
    pub from: String,
    pub to: String,
    pub raw: RawRange,
}

#[derive(Debug, Serialize)]
pub struct RawRange {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInfoQto {
    pub service_id: i32,
    pub pod_name: String,
}

//interval: 10s,"",""
//step: 10,30,30

impl QueryItem {
    pub fn init(
        instant: Option<bool>,
        expr: String,
        interval: String,
        interval_factor: i64,
        legend_format: String,
        metric: Option<String>,
        request_id: String,
        ref_id: String,
        step: i64,
        max_data_points: i64,
    ) -> Self {
        let datasource = DataSource {
            uid: "PBFA97CFB590B2093".to_string(),
            r#type: "prometheus".to_string(),
        };
        let exemplar = false;
        let format = "time_series".to_string();
        let interval_ms = 5000;
        let query_type = "timeSeriesQuery".to_string();
        //写过
        let utc_offset_sec = 28800;
        Self {
            expr,
            format,
            instant,
            interval,
            interval_factor,
            legend_format,
            ref_id,
            step,
            datasource,
            query_type,
            exemplar,
            request_id,
            utc_offset_sec,
            datasource_id: 1,
            interval_ms,
            max_data_points,
            metric,
        }
    }
}
impl QueryForm {
    pub fn init(query_item: Vec<QueryItem>) -> Self {
        let to = chrono::Utc::now();
        let from = to
            .checked_sub_signed(chrono::Duration::minutes(15))
            .unwrap();
        let from_timestamp = from.timestamp_millis().to_string();
        let to_timestamp = to.timestamp_millis().to_string();
        let to = to.to_string();

        Self {
            queries: query_item,
            range: QueryRange {
                from: from.to_string(),
                to,
                raw: RawRange {
                    from: "now-15m".to_string(),
                    to: "now".to_string(),
                },
            },
            from: from_timestamp,
            to: to_timestamp,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceSupervisor {
    pub cpu_info: Vec<InfoMap>,
    pub memory_info: Vec<Vec<InfoMap>>,
    pub io_info: IOInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IOInfo {
    pub tx_info: Vec<InfoMap>,
    pub rx_info: Vec<InfoMap>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InfoMap {
    pub key: String,
    pub value: String,
}

impl InfoMap {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
