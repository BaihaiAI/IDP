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
use std::path::Path;

use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
mod cli_args;
mod gateway;
mod spawn_all_services;

#[tokio::main]
async fn main() {
    let args = <cli_args::CliArgs as clap::Parser>::parse();
    let gateway_port = if let Ok(val) = std::env::var("GATEWAY_PORT") {
        val.parse().unwrap()
    } else {
        args.gateway_port
    };

    logger::init_logger();
    // preheat lazy static var
    business::path_tool::store_parent_dir();

    let gateway_exe_path = std::env::current_exe().unwrap();
    let exe_parent_dir = gateway_exe_path.parent().unwrap();

    let mut web_dir = None;
    for candidate_path in [
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../web/dist")).to_path_buf(),
        exe_parent_dir.join("web"),
        Path::new("/var/www").to_path_buf(),
    ] {
        if candidate_path.exists() {
            web_dir = Some(candidate_path.canonicalize().unwrap());
            break;
        }
    }
    let web_dir = web_dir.expect("web dir not found, please yarn build first");
    tracing::info!("web_dir = {web_dir:?}");

    // std::thread::scope(f)
    spawn_all_services::spawn_all_services(&args);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], gateway_port));
    let static_ = hyper_staticfile::Static::new(web_dir);
    let make_svc = make_service_fn(|conn: &AddrStream| {
        let static_ = static_.clone();
        let remote_addr = conn.remote_addr().ip();
        let args_ = args.clone();
        async move {
            Ok::<_, std::convert::Infallible>(service_fn(move |req| {
                gateway::gateway_handler(remote_addr, req, static_.clone(), args_.clone())
            }))
        }
    });
    let server = hyper::Server::bind(&addr).serve(make_svc);
    tracing::info!("listen addr = {addr}");
    // https://github.com/Byron/open-rs
    #[cfg(target_os = "linux")]
    let open_cmd = "xdg-open";
    #[cfg(target_os = "macos")]
    let open_cmd = "open";
    #[cfg(windows)]
    let open_cmd = "cmd";
    let mut cmd = std::process::Command::new(open_cmd);
    #[cfg(windows)]
    cmd.arg("/C").arg("start");

    if let Err(err) = cmd.arg(format!("http://{addr}")).spawn() {
        tracing::warn!("open browser err: {open_cmd} {err}");
    }

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
