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

use std::net::SocketAddr;

use common_model::Rsp;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use serde::Serialize;

use crate::app_context::AppContext;
use crate::handler;
use crate::handler::prelude::Resp;

trait RspExt {
    fn to_hyper(self) -> Response<Body>;
}

impl<T: Serialize> RspExt for Rsp<T> {
    fn to_hyper(self) -> Response<Body> {
        Response::builder()
            .status(hyper::StatusCode::from_u16(self.http_status_code()).unwrap())
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_vec(&self).unwrap()))
            .unwrap()
    }
}

/// https://vorner.github.io/2020/04/13/hyper-traps.html
pub async fn service_route(
    ctx: AppContext,
    req: Request<Body>,
    remote_addr: SocketAddr,
) -> Result<Response<Body>, std::convert::Infallible> {
    match service_route_inner(ctx, req, remote_addr).await {
        Ok(rsp) => Ok(rsp),
        Err(err) => {
            tracing::warn!("{err:#?}");
            Ok(Resp {
                code: err.err_code,
                message: err.message,
                data: (),
            }
            .to_hyper())
        }
    }
}

async fn service_route_inner(
    ctx: AppContext,
    req: Request<Body>,
    // remote_addr is k8s node ip, not pod ip
    _remote_addr: SocketAddr,
) -> Result<Response<Body>, crate::error::Error> {
    const BASE_URL: &str = "/api/v1/execute";
    let req_method = req.method();
    let req_path = req.uri().path().trim_start_matches(BASE_URL);
    if req_path == "/kernel/list" || req_path.starts_with("/ws/kernel/execute") {
        tracing::trace!("{req_method} {req_path}");
    } else if let Some(query) = req.uri().query() {
        tracing::info!("{req_method} {req_path}?{query}");
    } else {
        tracing::info!("{req_method} {req_path}");
    }

    // all post API
    if req_method == Method::POST {
        return match req_path {
            "/package/install" => Ok(handler::pip_install(req).await?.to_hyper()),
            "/package/uninstall" => Ok(handler::pip_uninstall(req).await?.to_hyper()),

            "/kernel/shutdown" => Ok(handler::post_shutdown_or_restart(ctx, req)
                .await?
                .to_hyper()),
            "/kernel/core_dump_report" => Ok(handler::core_dump_report(ctx, req).await?.to_hyper()),
            _ => return Ok(not_found()),
        };
    }

    // all get API
    let resp = match req_path {
        "/version" => Response::builder()
            .body(Body::from(env!("VERSION").to_string()))
            .unwrap(),
        _ if req_path.starts_with("/ws/kernel/execute") => handler::accept_execute_ws(ctx, req)?,
        "/ws/kernel/connect" => {
            let kernel_info_str = req.headers()[kernel_common::KernelInfo::HTTP_HEADER]
                .to_str()
                .unwrap();
            let kernel_info_str = urlencoding::decode(kernel_info_str).unwrap();
            let kernel_info =
                serde_json::from_str::<kernel_common::KernelInfo>(&kernel_info_str).unwrap();
            // kernel_info.ip = remote_addr;
            tracing::info!("kernel_info = {kernel_info:?}");
            handler::accept_ws_kernel_connect(ctx, req, kernel_info)?
        }

        "/notebook/cell_state" => handler::cell_state(ctx, req).await?.to_hyper(),
        "/notebook/vars" => handler::vars(req)?.to_hyper(),

        "/kernel/list" => handler::kernel_list(ctx, req).await?.to_hyper(),

        // TODO these should be POST API
        "/kernel/interrupt" => handler::interrupt(ctx, req).await?.to_hyper(),
        "/kernel/shutdown" => handler::get_shutdown_or_restart(ctx, req).await?.to_hyper(),
        "/kernel/shutdown_all" => handler::shutdown_all(ctx, req).await?.to_hyper(),
        "/kernel/pause" => handler::pause(ctx, req).await?.to_hyper(),
        "/kernel/resume" => handler::resume(ctx, req).await?.to_hyper(),

        path => {
            tracing::debug!("GET {path} not found in route");
            return Ok(not_found());
        }
    };
    Ok(resp)
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(Body::from("404 not found".to_string()))
        .unwrap()
}
