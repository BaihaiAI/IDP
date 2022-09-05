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

#![deny(unused_crate_dependencies)]
#[cfg(unix)]
pub fn main_(port: u16) {
    let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).unwrap();
    let mut ctx = std::collections::HashMap::new();
    for stream_res in listener.incoming() {
        let stream = stream_res.unwrap();
        handle_connection(stream, &mut ctx);
    }
}

#[cfg(windows)]
pub fn main_(port: u16) {
    let listener = std::sync::Arc::new(tiny_http::Server::http(format!("0.0.0.0:{port}")).unwrap());

    let mut ctx = std::collections::HashMap::new();
    for request in listener.incoming_requests() {
        handle_connection(request, &mut ctx);
    }
}
#[cfg(unix)]
fn handle_connection(
    mut stream: std::net::TcpStream,
    ctx: &mut std::collections::HashMap<String, u32>,
) {
    use std::io::Read;
    use std::io::Write;
    let mut req = [0; 4096];
    let n_read = stream.read(&mut req).unwrap();
    let req_str = String::from_utf8_lossy(&req[..n_read]);
    let body_idx = req_str.find("\r\n\r\n").unwrap() + "\r\n\r\n".len();
    let body = String::from_utf8_lossy(&req[body_idx..n_read]);

    let contents = if req_str.starts_with("POST /start_kernel") {
        let req = serde_json::from_str::<kernel_common::spawn_kernel_process::SpawnKernel>(&body)
            .unwrap();
        println!("--> start_kernel, req={req:?}");
        let path = req.header.path.clone();
        #[cfg_attr(target_os = "windows", allow(unused_mut))]
        let mut child = match kernel_common::spawn_kernel_process::spawn_kernel_process(req.header)
        {
            Ok(child) => child,
            Err(err) => {
                eprintln!("{}:{} {err:#?}", module_path!(), line!());
                let contents = format!(r#"{{"code":500,"message":{}}}"#, err.message);
                let response = format!(
                    "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                return;
            }
        };
        let pid = child.id();
        ctx.insert(path, pid);
        std::thread::spawn(move || {
            use std::os::unix::process::ExitStatusExt;
            let exit_status = child.wait().unwrap();

            if !exit_status.core_dumped() {
                return;
            }

            eprintln!("kernel core dump");
            let ip = os_utils::network::dns_resolve(&os_utils::get_hostname());
            let url = format!(
                "http://127.0.0.1:{}/api/v1/execute/kernel/core_dump_report?ip={ip}&pid={pid}",
                business::kernel_manage_port()
            );
            let resp = reqwest::blocking::Client::new().post(&url).send().unwrap();
            dbg!(url, resp.status());
        });
        r#"{"message":"ok"}"#
    } else if req_str.starts_with("GET /cluster/suggest") {
        r#"{"code":21300000,"data":{"CPU":0.2,"GPU":0,"memory":214748364.8,"node:127.0.0.1":1.0,"node:10.9.115.195":1.0,"object_store_memory":4776877670.0},"message":"success"}"#
    } else {
        r#"{"message":"404"}"#
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// windows stream.read only return status line in first read
#[cfg(windows)]
fn handle_connection(
    mut request: tiny_http::Request,
    ctx: &mut std::collections::HashMap<String, u32>,
) {
    let status_line = format!("{} {}", request.method(), request.url());
    let mut buf = [0; 4096];
    let n_read = request.as_reader().read(&mut buf).unwrap();
    let contents = if status_line.starts_with("POST /start_kernel") {
        let req = serde_json::from_slice::<kernel_common::spawn_kernel_process::SpawnKernel>(
            &buf[..n_read],
        )
        .unwrap();

        println!("--> start_kernel, req={req:?}");
        let path = req.header.path.clone();
        #[cfg_attr(target_os = "windows", allow(unused_mut))]
        let mut child =
            kernel_common::spawn_kernel_process::spawn_kernel_process(req.header).unwrap();
        let pid = child.id();
        ctx.insert(path, pid);
        r#"{"message":"ok"}"#
    } else if status_line.starts_with("GET /cluster/suggest") {
        r#"{"code":21300000,"data":{"CPU":0.2,"GPU":0,"memory":214748364.8,"node:127.0.0.1":1.0,"node:10.9.115.195":1.0,"object_store_memory":4776877670.0},"message":"success"}"#
    } else {
        r#"{"message":"404"}"#
    };

    let response = tiny_http::Response::from_string(contents);
    request.respond(response).unwrap()
}
