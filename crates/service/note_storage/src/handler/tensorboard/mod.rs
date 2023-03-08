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

pub(crate) mod lstio_route;
pub(crate) mod spawn_tensorboard_process;
use std::collections::BTreeMap;
use std::sync::Arc;

use axum::extract::Json;
use business::path_tool::get_store_full_path;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use common_tools::cookies_tools::Cookies;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;
use tracing::info;

use crate::common::error::IdpGlobalError;

pub type ProjectId = u64;

pub struct TensorboardEntry {
    hostname: String,
    port: u16,
    relative_log_dir: String,
    child: tokio::process::Child,
}

/// e.g. hostname is idp-develop-b-executor-7b77cd4c6c-n866m port is 9090
/// tbid is 7b77cd4c6c-n866m-9090
pub(crate) fn tbid(hostname: &str, port: u16) -> String {
    let mut host = hostname.rsplit('-').take(2).collect::<Vec<_>>();
    host.reverse();
    let host = host.join("-");
    format!("{host}-{port}")
}

#[test]
fn test_tbid() {
    let hostname = "idp-develop-b-executor-7b77cd4c6c-n866m";
    let mut host = hostname.rsplit('-').take(2).collect::<Vec<_>>();
    host.reverse();
    let host = host.join("-");
    assert_eq!(host, "7b77cd4c6c-n866m");
}

impl TensorboardEntry {
    fn to_resp(&self) -> StartTensorboardResp {
        StartTensorboardResp {
            hostname: self.hostname.clone(),
            port: self.port,
            tbid: tbid(&self.hostname, self.port),
            log_dir: self.relative_log_dir.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct StartTensorboardReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    project_id: ProjectId,
    log_dir: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
// #[cfg_attr(test, derive(Deserialize))]
pub struct StartTensorboardResp {
    hostname: String,
    port: u16,
    tbid: String,
    log_dir: String,
}

pub async fn start_tensorboard(
    axum::TypedHeader(cookies): axum::TypedHeader<Cookies>,
    project_id_tensorboard_port_mapping: axum::extract::State<
        Arc<Mutex<BTreeMap<ProjectId, TensorboardEntry>>>,
    >,
    Json(req): Json<StartTensorboardReq>,
) -> Result<Rsp<StartTensorboardResp>, ErrorTrace> {
    let team_id = get_cookie_value_by_team_id(cookies);
    info!("--> start_tensorboard: team_id={team_id}, req={req:?}");
    let log_abs_dir = get_store_full_path(team_id, req.project_id as _, &req.log_dir);
    info!("log_abs_dir = {log_abs_dir:?}");

    if let Some(mut entry) = project_id_tensorboard_port_mapping
        .lock()
        .await
        .get_mut(&req.project_id)
    {
        if entry.relative_log_dir == req.log_dir {
            return Ok(Rsp::success(entry.to_resp()).message("project_id+log_dir exist"));
        }

        info!("project_id exist, log_dir changed");
        entry.child.kill().await?;
        // TODO reuse port should delay for TCP time wait?
        let child =
            spawn_tensorboard_process::spawn_tensorboard_process(&log_abs_dir, entry.port).await?;
        entry.child = child;
        entry.relative_log_dir = req.log_dir;
        return Ok(Rsp::success(entry.to_resp()).message("project_id exist, log_dir changed"));
    }

    let hostname = os_utils::get_hostname();
    let port = os_utils::get_unused_port();

    if !log_abs_dir.exists() {
        return Err(ErrorTrace::new("log_dir not exist"));
    }

    let child = spawn_tensorboard_process::spawn_tensorboard_process(&log_abs_dir, port).await?;
    lstio_route::add_lstio_route_by_tbid(port).await?;
    let entry = TensorboardEntry {
        hostname,
        port,
        child,
        relative_log_dir: req.log_dir,
    };

    let resp = entry.to_resp();
    project_id_tensorboard_port_mapping
        .lock()
        .await
        .insert(req.project_id, entry);
    info!("<-- start_tensorboard");
    Ok(Rsp::success(resp).message("project_id+log_dir create"))
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TensorboardReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    project_id: ProjectId,
}

pub async fn stop_tensorboard(
    project_id_tensorboard_port_mapping: axum::extract::State<
        Arc<Mutex<BTreeMap<ProjectId, TensorboardEntry>>>,
    >,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(req): Json<TensorboardReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = get_cookie_value_by_team_id(cookies);
    let region = business::region::REGION.clone();
    info!("--> stop_tensorboard: team_id={team_id}, region={region}, req={req:?}");

    let mut mapping = project_id_tensorboard_port_mapping.lock().await;
    let entry = match mapping.remove(&req.project_id) {
        Some(entry) => entry,
        None => {
            return Err(ErrorTrace::new("no tensorboard found in project"));
        }
    };
    lstio_route::delete_lstio_route_by_tbid(entry.port, region).await?;
    drop(entry);

    mapping.remove(&req.project_id);
    Ok(Rsp::success(()))
}

pub async fn tensorboard_info(
    project_id_tensorboard_port_mapping: axum::extract::State<
        Arc<Mutex<BTreeMap<ProjectId, TensorboardEntry>>>,
    >,
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(req): Json<TensorboardReq>,
) -> Result<Rsp<Option<StartTensorboardResp>>, IdpGlobalError> {
    let team_id = get_cookie_value_by_team_id(cookies.clone());
    let region = business::region::REGION.clone();
    info!("--> tensorboard_info: team_id={team_id}, region={region}, req={req:?}");

    let mapping = project_id_tensorboard_port_mapping.lock().await;
    match mapping.get(&req.project_id) {
        Some(entry) => Ok(Rsp::success(Some(entry.to_resp()))),
        None => Ok(Rsp::success(None)),
    }
}

#[test]
#[ignore]
fn test_tensorboard() {
    const BASE_URL: &str = "http://127.0.0.1:8082/api/v2/idp-note-rs";
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    for _ in 0..3 {
        let resp = client
            .post(format!("{BASE_URL}/tensorboard/start"))
            .header(reqwest::header::COOKIE, "teamId=1")
            .json(&StartTensorboardReq {
                project_id: 1,
                log_dir: "/home/w/Downloads/logs".to_string(),
            })
            .send()
            .unwrap();
        dbg!(resp.status());
        assert!(resp.status().is_success());
        dbg!(resp.json::<serde_json::Value>().unwrap());
    }

    let resp = client
        .post(format!("{BASE_URL}/tensorboard/stop"))
        .header(reqwest::header::COOKIE, "teamId=1;region=b")
        .json(&TensorboardReq { project_id: 1 })
        .send()
        .unwrap();
    assert!(resp.status().is_success());
}
