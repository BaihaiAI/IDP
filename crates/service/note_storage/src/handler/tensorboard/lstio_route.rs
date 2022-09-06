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

use err::ErrorTrace;
use tracing::error;
use tracing::info;

const K8S_SERVICE_API_BASE_URL: &str = "http://idp-k8s-service-svc:8084/api/command/k8s";

/// /api/command/k8s/channel
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddRouteReq {
    /// unique hostname:port
    /// account is same as tbid in cookie
    account: String,
    pod_name: String,
    pod_port: String,
    /// this field is `idp` only(k8s namespace is in hostname)
    service: &'static str,
    /// this field is `setting-extend-channel`
    action: &'static str,
}

pub async fn add_lstio_route_by_tbid(port: u16) -> Result<(), ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build()?;
    let hostname = os_utils::get_hostname();
    let tbid = super::tbid(&hostname, port);
    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/channel"))
        .json(&AddRouteReq {
            account: tbid,
            pod_name: hostname,
            pod_port: port.to_string(),
            service: "idp",
            action: "setting-extend-channel",
        })
        .send()
        .await?;
    if !resp.status().is_success() {
        let err = format!("add route resp http code not 200: {}", resp.status());
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }
    info!("send add route req success");
    let resp = resp.json::<serde_json::Value>().await?;
    info!("resp = {resp:#?}");
    if resp["code"].as_u64() != Some(200) {
        let err = format!(
            "add route resp json code not 200: {:?}",
            resp["code"].as_u64()
        );
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }
    Ok(())
}

/// /api/command/k8s/destroy
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeleteRouteReq {
    /// team_id-project_id
    account: String,
    region: String,
    /// this field is `idp` only(k8s namespace is in hostname)
    service: &'static str,
    /// this field is `destroy-extend-channel`
    action: &'static str,
}

pub async fn delete_lstio_route_by_tbid(port: u16, region: String) -> Result<(), ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build()?;
    let hostname = os_utils::get_hostname();
    let tbid = super::tbid(&hostname, port);
    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/destroy"))
        .json(&DeleteRouteReq {
            account: tbid,
            region,
            service: "idp",
            action: "destroy-extend-channel",
        })
        .send()
        .await?;
    if !resp.status().is_success() {
        let err = format!("delete route resp http code not 200: {}", resp.status());
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }
    info!("send delete route req success");
    let resp = resp.json::<serde_json::Value>().await?;
    info!("resp = {resp:#?}");
    if resp["code"].as_u64() != Some(200) {
        let err = format!(
            "delete route resp json code not 200: {:?}",
            resp["code"].as_u64()
        );
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }
    Ok(())
}
