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
use axum::Json;
use chrono::Duration;
use common_model::Rsp;
use err::ErrorTrace;
use url::form_urlencoded::byte_serialize;
use urlencoding::encode;

use crate::api_model::service_log::ContainerInfo;
use crate::api_model::service_log::ContainerListRsp;
use crate::api_model::service_log::IOInfo;
use crate::api_model::service_log::InfoMap;
use crate::api_model::service_log::LogListQto;
use crate::api_model::service_log::LogQto;
use crate::api_model::service_log::LogResult;
use crate::api_model::service_log::LogsRsp;
use crate::api_model::service_log::Node;
use crate::api_model::service_log::PodListRsp;
use crate::api_model::service_log::QueryForm;
use crate::api_model::service_log::QueryItem;
use crate::api_model::service_log::ResourceInfoQto;
use crate::api_model::service_log::ResourceSupervisor;
use crate::api_model::service_log::SeriesInfo;

//TODO change namespace
pub async fn log_list(Query(req): Query<LogListQto>) -> Result<Rsp<Vec<SeriesInfo>>, ErrorTrace> {
    let namespace = get_namespace();
    let match_item = format!("{{job=\"{}/sidecar-{}-log\"}}", namespace, req.service_id);
    let match_item = encode(&match_item);

    let timestamp = chrono::Utc::now();
    let start_timestamp = timestamp
        .checked_sub_signed(Duration::days(365))
        .unwrap()
        .timestamp_nanos()
        .to_string();
    let end_timestamp = timestamp.timestamp_nanos().to_string();

    tracing::info!("[log_list]match_item -> {:#?}", match_item);
    tracing::info!("[log_list]start_timestamp -> {:#?}", start_timestamp);
    tracing::info!("[log_list]end_timestamp -> {:#?}", end_timestamp);

    let series_url = format!(
        "http://loki:3100/loki/api/v1/series?start={start_timestamp}&end={end_timestamp}&match%5B%5D={match_item}",
    );

    let client = reqwest::ClientBuilder::new().build()?;

    let resp = client.get(&series_url).send().await?;

    tracing::info!("[log_list] series_url : {:#?}", series_url);
    tracing::info!("[log_list] resp : {:#?}", resp);
    let body = resp.text().await?;

    tracing::info!("[log_list] body: {:#?}", body);
    let log_list = serde_json::from_str::<Node>(&body)?;

    Ok(Rsp::success(log_list.data))
}

pub async fn get_log(Query(req): Query<LogQto>) -> Result<Rsp<LogResult>, ErrorTrace> {
    let query_item = format!(
        "{{job=\"{}\",filename=\"{}\"}} |= \"\" ",
        req.job, req.file_name
    );
    let query_item = encode(&query_item);

    let start_timestamp = req.start;
    let end_timestamp = req.end;
    // let timestamp = chrono::Utc::now();
    // let start_timestamp = timestamp
    //     .checked_sub_signed(Duration::seconds(3600))
    //     .unwrap()
    //     .timestamp_nanos()
    //     .to_string();
    // let end_timestamp = timestamp.timestamp_nanos().to_string();

    tracing::info!("[get_log]query_item -> {:#?}", query_item);
    tracing::info!("[get_log]start_timestamp -> {:#?}", start_timestamp);
    tracing::info!("[get_log]end_timestamp -> {:#?}", end_timestamp);

    let log_url = format!(
    "http://loki:3100/loki/api/v1/query_range?direction=FORWARD&limit=1000&query={query_item}&start={start_timestamp}&end={end_timestamp}&step=60
    ");

    let client = reqwest::ClientBuilder::new().build()?;

    let resp = client.get(&log_url).send().await?;
    tracing::info!("[get_log] resp : {:#?}", resp);

    let body = resp.text().await?;
    let log_list = serde_json::from_str::<LogsRsp>(&body)?;

    Ok(Rsp::success(log_list.data))
}

pub async fn container_list(Json(req): Json<LogListQto>) -> Result<Rsp<Vec<String>>, ErrorTrace> {
    let start_with = format!("idp-model-{}-", req.service_id);
    let namespace = get_namespace();
    let kube_pod_info = format!("kube_pod_info{{namespace=~\"{namespace}\"}}");
    let kube_pod_info: String = byte_serialize(kube_pod_info.as_bytes()).collect();

    let current = chrono::Utc::now();
    let start = current
        .checked_sub_signed(Duration::days(365))
        .unwrap()
        .timestamp()
        .to_string();
    let end = current.timestamp().to_string();
    tracing::info!("[container_list] kube_pod_info: {:#?}", kube_pod_info);

    let user_info = encode("idp@baihai.ai");
    let series_url =
        format!("http://admin:{user_info}@grafana:3000/api/datasources/proxy/1/api/v1/series");
    let client = reqwest::Client::new();

    let body = format!(r#"match%5B%5D={kube_pod_info}&start={start}&end={end}"#);
    tracing::info!("[container_list] body -> {:#?}", body);

    let rsp = client
        .post(&series_url)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;
    tracing::info!("[container_list] rsp : {:#?}", rsp);

    let body = rsp.text().await?;
    tracing::info!("[container_list] body: {:#?}", body);
    let pod_model = serde_json::from_str::<PodListRsp>(&body)?;

    let mut pod_list = Vec::new();
    for item in pod_model.data.into_iter() {
        if item.pod.contains(&start_with) {
            pod_list.push(item.pod);
        }
    }

    Ok(Rsp::success(pod_list))
}

pub async fn resource_usage(
    Json(req): Json<ResourceInfoQto>,
) -> Result<Rsp<ResourceSupervisor>, ErrorTrace> {
    let namespace = get_namespace();
    let container_info = get_container(&req.pod_name).await?;
    tracing::info!("[resource_usage]container_info -> {:#?}", container_info);

    //call cpu
    let cpu_query_item = QueryItem::init(
        Some(false),
        format!(
            "sum by (pod) (rate(container_cpu_usage_seconds_total{{image!=\"\", namespace=\"{}\", pod=~\"{}\", container=~\".*\",container!=\"POD\"}}[1m]))",
            namespace, req.pod_name
        ),
        "".to_string(),
        2,
        "Current: {{ pod }}".to_string(),
        None,
        "2A".to_string(),
        "A".to_string(),
        30,
        672,
    );
    let mut cpu_vec = Vec::new();
    cpu_vec.push(cpu_query_item);
    let cpu_param = QueryForm::init(cpu_vec);
    let cpu_param = serde_json::json!(cpu_param);
    let user_info = encode("idp@baihai.ai");
    let query_url = format!("http://admin:{user_info}@grafana:3000/api/ds/query");

    //call memory
    let memory_query_item = QueryItem::init(
        None,
        format!(
            "container_memory_working_set_bytes{{pod=~\"{}\",container!=\"POD\",container!=\"\",image!=\"\", namespace=\"{}\",container=~\".*\"}}",
            req.pod_name, namespace
        ),
        "10s".to_string(),
        1,
        "Current: {{ pod }}".to_string(),
        Some("container_memory_usage_bytes".to_string()),
        "1A".to_string(),
        "A".to_string(),
        10,
        672,
    );
    let mut memory_vec = Vec::new();
    memory_vec.push(memory_query_item);
    let memory_param = QueryForm::init(memory_vec);
    let memory_param = serde_json::json!(memory_param);

    //call io
    let trainsmit_query_item = QueryItem::init(
        Some(false),
        format!(
            "rate(container_network_transmit_bytes_total{{pod=~\"{}\",interface=~\"eth0|ens.*\"}}[5m])",
            req.pod_name
        ),
        "".to_string(),
        2,
        "TX: {{ pod }}".to_string(),
        None,
        "3A".to_string(),
        "A".to_string(),
        30,
        672,
    );
    let receive_query_item = QueryItem::init(
        Some(false),
        format!(
            "rate(container_network_receive_bytes_total{{pod=~\"{}\",interface=~\"eth0|ens.*\"}}[5m])",
            req.pod_name
        ),
        "".to_string(),
        2,
        "RX: {{ pod }}".to_string(),
        None,
        "3B".to_string(),
        "B".to_string(),
        30,
        672,
    );
    let mut io_vec = Vec::new();
    io_vec.push(trainsmit_query_item);
    io_vec.push(receive_query_item);
    let io_param = QueryForm::init(io_vec);
    let io_param = serde_json::json!(io_param);

    tracing::info!("[resource_usage] query_url -> {:#?}", query_url);
    tracing::info!("[resource_usage] cpu_param -> {}", cpu_param);
    tracing::info!("[resource_usage] memory_param -> {}", memory_param);
    tracing::info!("[resource_usage] io_param -> {}", io_param);

    let cpu_future = query_reqwest_sending(&query_url, cpu_param);
    let memory_future = query_reqwest_sending(&query_url, memory_param);
    let io_future = query_reqwest_sending(&query_url, io_param);

    let (cpu_rsp, memory_rsp, io_rsp) =
        futures::future::join3(cpu_future, memory_future, io_future).await;

    let cpu_body = cpu_rsp?.text().await?;
    tracing::info!("[resource_usage] cpu_body -> {:#?}", cpu_body);
    let cpu_model = serde_json::from_str::<serde_json::Value>(&cpu_body)?;
    let cpu_time = &cpu_model["results"]["A"]["frames"][0]["data"]["values"][0];
    let cpu_value = &cpu_model["results"]["A"]["frames"][0]["data"]["values"][1];
    let cpu_time = serde_json::from_value::<Vec<i64>>(cpu_time.clone()).unwrap_or_default();
    let cpu_value = serde_json::from_value::<Vec<f64>>(cpu_value.clone()).unwrap_or_default();
    let mut cpu_map = Vec::new();
    for (index, item) in cpu_time.into_iter().enumerate() {
        cpu_map.push(InfoMap::new(item.to_string(), cpu_value[index].to_string()));
    }

    let memory_body = memory_rsp?.text().await?;
    tracing::info!("[resource_usage] memory_body -> {:#?}", memory_body);
    let memory_model = serde_json::from_str::<serde_json::Value>(&memory_body)?;

    let mut memory_vec = Vec::new();
    for item in 0..container_info.len() {
        let memory_time = &memory_model["results"]["A"]["frames"][item]["data"]["values"][0];
        let memory_value = &memory_model["results"]["A"]["frames"][item]["data"]["values"][1];
        let memory_time =
            serde_json::from_value::<Vec<i64>>(memory_time.clone()).unwrap_or_default();
        let memory_value =
            serde_json::from_value::<Vec<f64>>(memory_value.clone()).unwrap_or_default();
        let mut memory_map = Vec::new();
        for (index, item) in memory_time.into_iter().enumerate() {
            memory_map.push(InfoMap::new(
                item.to_string(),
                memory_value[index].to_string(),
            ));
        }
        memory_vec.push(memory_map);
    }

    let io_body = io_rsp?.text().await?;
    tracing::info!("[resource_usage] io_body -> {:#?}", io_body);
    let io_model = serde_json::from_str::<serde_json::Value>(&io_body)?;

    let tx_time = &io_model["results"]["A"]["frames"][0]["data"]["values"][0];
    let tx_value = &io_model["results"]["A"]["frames"][0]["data"]["values"][1];
    let tx_time = serde_json::from_value::<Vec<i64>>(tx_time.clone()).unwrap_or_default();
    let tx_value = serde_json::from_value::<Vec<f64>>(tx_value.clone()).unwrap_or_default();

    let rx_time = &io_model["results"]["B"]["frames"][0]["data"]["values"][0];
    let rx_value = &io_model["results"]["B"]["frames"][0]["data"]["values"][1];
    let rx_time = serde_json::from_value::<Vec<i64>>(rx_time.clone()).unwrap_or_default();
    let rx_value = serde_json::from_value::<Vec<f64>>(rx_value.clone()).unwrap_or_default();

    let mut tx_map = Vec::new();
    for (index, item) in tx_time.into_iter().enumerate() {
        tx_map.push(InfoMap::new(item.to_string(), tx_value[index].to_string()));
    }

    let mut rx_map = Vec::new();
    for (index, item) in rx_time.into_iter().enumerate() {
        rx_map.push(InfoMap::new(item.to_string(), rx_value[index].to_string()));
    }

    Ok(Rsp::success(ResourceSupervisor {
        cpu_info: cpu_map,
        memory_info: memory_vec,
        io_info: IOInfo {
            tx_info: tx_map,
            rx_info: rx_map,
        },
    }))
}

pub async fn get_container(pod_name: &String) -> Result<Vec<ContainerInfo>, ErrorTrace> {
    let namespace = get_namespace();
    let kube_pod_info =
        format!("kube_pod_container_info{{namespace=~\"{namespace}\",pod=~\"{pod_name}\"}}");
    let kube_pod_info: String = byte_serialize(kube_pod_info.as_bytes()).collect();

    let current = chrono::Utc::now();
    let start = current
        .checked_sub_signed(Duration::seconds(600))
        .unwrap()
        .timestamp()
        .to_string();
    let end = current.timestamp().to_string();
    tracing::info!("[get_container] kube_pod_info: {:#?}", kube_pod_info);

    let user_info = encode("idp@baihai.ai");
    let series_url =
        format!("http://admin:{user_info}@grafana:3000/api/datasources/proxy/1/api/v1/series");
    let client = reqwest::Client::new();

    let body = format!(r#"match%5B%5D={kube_pod_info}&start={start}&end={end}"#);
    tracing::info!("[get_container] body -> {:#?}", body);

    let rsp = client
        .post(&series_url)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;
    tracing::info!("[get_container] rsp : {:#?}", rsp);

    let body = rsp.text().await?;
    tracing::info!("[get_container] body: {:#?}", body);

    //podmodel
    let pod_model = serde_json::from_str::<ContainerListRsp>(&body)?;

    Ok(pod_model.data)
}

pub async fn query_reqwest_sending(
    query_url: &String,
    param: serde_json::Value,
) -> Result<reqwest::Response, ErrorTrace> {
    let client = reqwest::Client::new();
    match client
        .post(query_url)
        .header("Content-Type", "application/json")
        .json(&param)
        .send()
        .await
    {
        Ok(data) => {
            tracing::info!("[query_request_sending] rsp -> {:#?}", data);
            Ok(data)
        }
        Err(err) => {
            tracing::error!("[query_request_sending] err -> {:#?}", err);
            Err(err.into())
        }
    }
}

pub fn get_namespace() -> String {
    match &crate::app_context::CONFIG.net_domain as &str {
        "idp.ucloud.cn" => "box".to_string(),
        "nightly.ilinksure.com" => "nightly".to_string(),
        "idp.baihai.co" => "idp".to_string(),
        _ => "host".to_string(),
    }
}
