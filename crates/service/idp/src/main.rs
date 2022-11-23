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

const BAIHAI_AID_FILENAME: &str = "baihai_aid-2.0-py3-none-any.whl";
const BAIHAI_AID: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../docker_build/store/1/projects/1/notebooks/baihai_aid-2.0-py3-none-any.whl"
));

#[tokio::main]
async fn main() {
    let args = <cli_args::CliArgs as clap::Parser>::parse();

    #[cfg(feature = "license")]
    if let Err(msg) = license_generator::verify_license(&args.public_key, &args.license) {
        println!("{}", msg);
        std::process::exit(1);
    }

    #[cfg(feature = "license")]
    {
        let token = if let Some(ref token) = args.token {
            token.clone()
        } else {
            use rand::Rng;
            let token = rand::thread_rng().gen_range(10u64.pow(15)..10u64.pow(18));
            format!("{token:#02x}")
        };
        gateway::TOKEN.set(token).unwrap();
    }

    let gateway_port = if let Ok(val) = std::env::var("GATEWAY_PORT") {
        val.parse().unwrap()
    } else {
        args.gateway_port
    };

    logger::init_logger();
    // preheat lazy static var
    business::path_tool::store_parent_dir();

    std::fs::write(BAIHAI_AID_FILENAME, BAIHAI_AID).unwrap();
    std::process::Command::new("pip3")
        .arg("install")
        .arg(BAIHAI_AID_FILENAME)
        .arg("--quiet")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let gateway_exe_path = std::env::current_exe().unwrap();
    let exe_parent_dir = gateway_exe_path.parent().unwrap();

    let mut web_dir = None;
    for candidate_path in [
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../web/dist")).to_path_buf(),
        exe_parent_dir.join("web"),
        gateway_exe_path.parent().unwrap().join("web"),
    ] {
        if candidate_path.exists() {
            web_dir = Some(candidate_path.canonicalize().unwrap());
            break;
        }
    }
    let web_dir = web_dir.expect("web dir not found, please yarn build first");
    tracing::info!("web_dir = {web_dir:?}");

    spawn_all_services::spawn_all_services(&args);

    let listen_addr = match args.listen_addr {
        Some(addr) => addr,
        None => std::net::Ipv4Addr::LOCALHOST,
    };
    let addr = std::net::SocketAddr::from((listen_addr, gateway_port));
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

    #[cfg(not(feature = "license"))]
    let url = format!("http://{addr}");
    #[cfg(feature = "license")]
    let url = format!("http://{addr}?token={}", gateway::TOKEN.get().unwrap());
    if let Err(err) = cmd.arg(&url).spawn() {
        tracing::warn!("open browser err: {open_cmd} {err}");
    }
    tracing::info!("{url}");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
