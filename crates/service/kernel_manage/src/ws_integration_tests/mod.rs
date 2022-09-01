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

use std::net::TcpStream;

use hyper_tungstenite::tungstenite;
use hyper_tungstenite::tungstenite::stream::MaybeTlsStream;
use hyper_tungstenite::tungstenite::WebSocket;
use kernel_common::Message;
use tungstenite::Message as WsMsg;

use crate::handler::execute_code::execute_req_model::CellTypeMeta;
use crate::handler::execute_code::execute_req_model::ExecuteCodeReq;

const DOCKER_ALL_IN_ONE_PORT: u16 = 3003;
const REGION: &str = "a";
const TEAM_ID: u64 = 12345;
const PROJECT_ID: u64 = 6789;

fn ws_code_req(code: &str) -> ExecuteCodeReq {
    ExecuteCodeReq {
        header: kernel_common::Header {
            project_id: PROJECT_ID,
            team_id: TEAM_ID,
            ..Default::default()
        },
        resource: kernel_common::spawn_kernel_process::Resource::default(),
        input_reply: None,
        cell_type: CellTypeMeta::Code {},
        code: code.to_string(),
        region: REGION.to_string(),
    }
}

fn connect() -> WebSocket<MaybeTlsStream<TcpStream>> {
    // const TEAM_ID: u64 = 12345;
    let url = format!(
        "ws://127.0.0.1:{DOCKER_ALL_IN_ONE_PORT}/{REGION}/api/v1/execute/ws/kernel/execute?projectId={PROJECT_ID}"
    );
    let mut req = tungstenite::client::IntoClientRequest::into_client_request(url).unwrap();
    req.headers_mut().insert(
        "Cookie",
        hyper::http::HeaderValue::from_static("teamId=12345"),
    );
    let (conn, _rsp) = tungstenite::connect(req).unwrap();
    conn
}

// cat docker_build/store/12345/projects/6789/notebooks/demo.ipynb | jq .cells
#[test]
#[ignore]
fn test_input() {
    logger::init_logger();
    let shutdown_all_url = format!(
        "http://127.0.0.1:{DOCKER_ALL_IN_ONE_PORT}/{REGION}/api/v1/execute/kernel/shutdown_all?projectId={PROJECT_ID}&path="
    );
    let shutdown_rsp = reqwest::blocking::get(shutdown_all_url)
        .unwrap()
        .text()
        .unwrap();
    tracing::info!("shutdown_rsp = {shutdown_rsp}");
    let mut req = ws_code_req("input()");
    req.header.path = "/demo.ipynb".to_string();
    req.header.cell_id = "90c21aa3-5d17-4fef-b64c-640e17b24203".to_string();
    let mut stream = connect();
    stream
        .write_message(WsMsg::Text(serde_json::to_string(&req).unwrap()))
        .unwrap();
    let _execute_input = stream.read_message().unwrap().into_text().unwrap();

    let input_request = stream.read_message().unwrap().into_text().unwrap();
    let input_request = serde_json::from_str::<Message>(&input_request).unwrap();
    dbg!(&input_request);
    req.input_reply = Some("user_input".to_string());
    stream
        .write_message(WsMsg::Text(serde_json::to_string(&req).unwrap()))
        .unwrap();

    let rsp = stream.read_message().unwrap().into_text().unwrap();
    dbg!(rsp);
}
