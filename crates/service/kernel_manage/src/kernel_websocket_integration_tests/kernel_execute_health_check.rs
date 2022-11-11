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

use hyper_tungstenite::tungstenite;

use crate::handler::execute_code::ExecuteCodeReq;

#[derive(serde::Deserialize, Debug)]
struct Config {
    host: String,
    region: String,
    team_id: u64,
    project_id: u64,
    path: String,
    cell_id: String,
    code: String,
}

impl Config {
    fn cookie(&self) -> String {
        format!("region={}; teamId={}", self.region, self.team_id)
    }
    fn api_base_url(&self) -> String {
        format!("{}/{}/api/v1/execute", self.host, self.region)
    }
    fn ws_execute_url(&self) -> String {
        format!(
            "ws://{}/ws/kernel/execute?projectId={}",
            self.api_base_url(),
            self.project_id
        )
    }
    fn ws_code_req(&self) -> ExecuteCodeReq {
        ExecuteCodeReq {
            header: kernel_common::Header {
                project_id: self.project_id,
                team_id: self.team_id,
                path: self.path.clone(),
                cell_id: self.cell_id.clone(),
                ..Default::default()
            },
            batch_id: 0,
            resource: kernel_common::spawn_kernel_process::Resource::default(),
            input_reply: None,
            cell_type: crate::handler::execute_code::execute_req_model::CellTypeMeta::Code {},
            code: self.code.clone(),
            region: self.region.clone(),
        }
    }
}

#[test]
#[ignore]
fn main() {
    let config = toml::de::from_str::<Config>(
        &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml")).unwrap(),
    )
    .unwrap();

    // shutdown kernel first before we send ws req, make sure we can receive start_kernel message later
    let shutdown_req = crate::handler::shutdown_or_restart::Req {
        inode: kernel_common::Header {
            project_id: config.project_id,
            team_id: config.team_id,
            cell_id: config.cell_id.clone(),
            path: config.path.clone(),
            pipeline_opt: None,
        }
        .inode(),
        project_id: config.project_id,
        team_id: config.team_id,
        path: config.path.clone(),
        resource: None,
        restart: false,
    };
    let shutdown_url = format!(
        "http://{}/kernel/shutdown?{}",
        config.api_base_url(),
        serde_urlencoded::to_string(&shutdown_req).unwrap()
    );
    let http_client = reqwest::blocking::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .unwrap();
    let shutdown_rsp = http_client
        .get(shutdown_url)
        .header(reqwest::header::COOKIE, config.cookie())
        .send()
        .unwrap();
    // assert!(shutdown_rsp.status().is_success());
    assert!(shutdown_rsp.status().as_u16() >= 200);

    let mut req =
        tungstenite::client::IntoClientRequest::into_client_request(config.ws_execute_url())
            .unwrap();
    req.headers_mut().insert(
        reqwest::header::COOKIE,
        hyper::http::HeaderValue::from_str(&config.cookie()).unwrap(),
    );
    let (mut stream, rsp) = tungstenite::connect(req).unwrap();
    assert_eq!(rsp.status(), reqwest::StatusCode::SWITCHING_PROTOCOLS);
    stream
        .write_message(tungstenite::Message::Text(
            serde_json::to_string(&config.ws_code_req()).unwrap(),
        ))
        .unwrap();
    for rsp_msg_type in [
        "start_kernel",
        "execute_input",
        "stream",
        "execute_reply",
        "status",
        "duration",
    ] {
        let rsp = stream.read_message().unwrap().into_text().unwrap();
        let rsp = serde_json::from_str::<serde_json::Value>(&rsp).unwrap();
        let msg_type = rsp["msgType"].as_str().unwrap();
        assert_eq!(rsp_msg_type, msg_type);
    }
    stream.close(None).unwrap();
}

#[test]
#[ignore]
fn run_main_multi_times() {
    for _ in 0..10000 {
        main();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
