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

use axum::extract::Query;
use axum::extract::State;
use common_model::Rsp;
use err::ErrorTrace;

use super::kubernetes_pod_status_watcher::RuntimeStatusMap;
use super::ProjectIdQueryString;

/// all unit is percentage
#[derive(serde::Serialize, Default)]
pub struct ResourceUsageRsp {
    cpu: f64,
    memory: f64,
    gpu: f64,
    disk: f64,
}

/*
200 {"status":"success","data":{"resultType":"vector","result":[{"metric":{},"value":[1672908141.171,"0.008128651905260235"]}]}}

if pod not exist

200 {"status":"success","data":{"resultType":"vector","result":[]}}
*/
#[derive(serde::Deserialize)]
struct PrometheusRsp {
    // status: String
    data: PrometheusRspData,
}

#[derive(serde::Deserialize)]
struct PrometheusRspData {
    result: Vec<PrometheusRspDataResult>,
}

#[derive(serde::Deserialize)]
struct PrometheusRspDataResult {
    value: Vec<serde_json::Value>,
}

pub async fn runtime_resource_usage(
    Query(ProjectIdQueryString { project_id }): Query<ProjectIdQueryString>,
    State(runtime_status_map): State<RuntimeStatusMap>,
) -> Result<Rsp<ResourceUsageRsp>, ErrorTrace> {
    if !business::kubernetes::is_k8s() {
        return Ok(Rsp::success(ResourceUsageRsp::default()));
    }
    let is_running = runtime_status_map
        .project_pod_status(project_id)
        .await
        .is_running();
    if !is_running {
        return Ok(Rsp::success(ResourceUsageRsp::default()));
    }

    let namespace = &*business::kubernetes::NAMESPACE;
    let pod = business::kubernetes::runtime_pod_name(project_id);
    let container = business::kubernetes::runtime_pod_container(project_id);

    let prometheus_base_url = "http://prometheus:9090/api/v1/query?query=";
    let get_cpu_usage_query = format!(
        "
    max( rate(container_cpu_usage_seconds_total{{namespace='{namespace}', pod='{pod}', container='{container}'}}[1m]) )
/
max(kube_pod_container_resource_limits{{resource='cpu', namespace='{namespace}', pod='{pod}', container='{container}'}})
    "
    );
    let get_cpu_usage_query = urlencoding::encode(&get_cpu_usage_query);
    let cpu_usage_rsp = reqwest::get(format!("{prometheus_base_url}{get_cpu_usage_query}"))
        .await?
        .json::<PrometheusRsp>()
        .await?
        .data
        .result;
    // pod not found
    if cpu_usage_rsp.is_empty() {
        return Ok(Rsp::success(ResourceUsageRsp::default()));
    }
    let cpu_usage = cpu_usage_rsp[0].value[1].as_str().unwrap().parse::<f64>()?;

    let get_memory_usage_query = format!(
        "
    max( rate(container_memory_working_set_bytes{{namespace='{namespace}', pod='{pod}', container='{container}'}}[1m]) )
/
max(kube_pod_container_resource_limits{{resource='memory', namespace='{namespace}', pod='{pod}', container='{container}'}})
    "
    );
    let usage_rsp = reqwest::get(format!("{prometheus_base_url}{get_memory_usage_query}"))
        .await?
        .json::<PrometheusRsp>()
        .await?
        .data
        .result;
    // pod not found
    if usage_rsp.is_empty() {
        return Ok(Rsp::success(ResourceUsageRsp::default()));
    }
    let memory_usage = usage_rsp[0].value[1].as_str().unwrap().parse::<f64>()?;

    Ok(Rsp::success(ResourceUsageRsp {
        cpu: cpu_usage,
        memory: memory_usage,
        ..Default::default()
    }))
}
