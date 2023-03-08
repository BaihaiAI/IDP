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

//! a http reverse proxy like nginx
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper_reverse_proxy::ReverseProxy;
use hyper_staticfile::Static;
use once_cell::sync::Lazy;
#[cfg(feature = "license")]
use once_cell::sync::OnceCell;

use crate::cli_args::CliArgs;

#[cfg(feature = "license")]
pub(crate) static TOKEN: OnceCell<String> = OnceCell::new();

pub async fn gateway_handler(
    client_ip: std::net::IpAddr,
    req: Request<Body>,
    static_: Static,
    args: CliArgs,
) -> Result<Response<Body>, std::convert::Infallible> {
    Ok(handle_(client_ip, req, static_, args).await)
}

async fn handle_(
    client_ip: std::net::IpAddr,
    mut req: Request<Body>,
    static_: Static,
    args: CliArgs,
) -> Response<Body> {
    let req_path = req.uri().path();
    match req_path {
        _ if req_path.starts_with("/a/api/v2/idp-note-rs") => {
            #[cfg(feature = "license")]
            {
                let cookie = match req.headers().get(hyper::http::header::COOKIE) {
                    Some(cookie) => match cookie.to_str() {
                        Ok(cookie) => cookie,
                        Err(_) => {
                            return rsp_403("cookie to_str err");
                        }
                    },
                    None => return rsp_403("no cookie"),
                };
                let mut token = None;
                for kv in cookie.split("; ") {
                    if let Some((k, v)) = kv.split_once('=') {
                        if k == "token" {
                            token = Some(v.to_string());
                            break;
                        }
                    }
                }
                let token = match token {
                    Some(token) => token,
                    None => return rsp_403("token key not found in cookie"),
                };
                if token != *TOKEN.get().unwrap() {
                    return Response::builder()
                        .status(StatusCode::FORBIDDEN)
                        .body(Body::from("token is invalid or missing"))
                        .unwrap();
                }
            }

            uri_rewrite_remove_region(&mut req);
            // hyper would tokio spawn coroutinue to handler, so we doesn't need to spawn
            proxy_pass(req, client_ip, args.note_storage_port).await
        }
        // _ if req_path.starts_with("/a/api/v1/execute/kernel/ws") => {},
        _ if req_path.starts_with("/a/api/v1/execute") => {
            uri_rewrite_remove_region(&mut req);
            proxy_pass(req, client_ip, args.kernel_manage_port).await
        }
        _ if req_path.starts_with("/a/api/v1/lsp") => {
            uri_rewrite_lsp(&mut req);
            proxy_pass(req, client_ip, args.lsp_port).await
        }
        _ if req_path.starts_with("/a/api/v1/terminal")
            || req_path.starts_with("/a/api/v2/terminal") =>
        {
            uri_rewrite_remove_region(&mut req);
            proxy_pass(req, client_ip, args.terminal_port).await
        }
        _ if req_path.starts_with("/0/api/v1") => rsp_404(),
        _ => {
            // static file
            if req_path.contains("workspace") || req_path.contains("tensorboard") {
                uri_rewrite_workspace_or_tensorboard(&mut req);
            }
            match static_.serve(req).await {
                Ok(resp) => resp,
                Err(err) => {
                    dbg!(err);
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap()
                }
            }
        }
    }
}

async fn proxy_pass(
    req: Request<Body>,
    client_ip: std::net::IpAddr,
    proxy_pass_port: u16,
) -> Response<Body> {
    static PROXY_CLIENT: Lazy<ReverseProxy<hyper::client::HttpConnector>> =
        Lazy::new(|| ReverseProxy::new(hyper::Client::new()));
    match PROXY_CLIENT
        .call(
            client_ip,
            &format!("http://127.0.0.1:{proxy_pass_port}"),
            req,
        )
        .await
    {
        Ok(response) => response,
        Err(err) => {
            dbg!(err, proxy_pass_port);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}

/**
 * Input:  /a/api/v1
 * Output: /api/v1
*/
fn uri_rewrite_remove_region(req: &mut Request<Body>) {
    let mut uri_parts = req.uri().clone().into_parts();
    uri_parts.path_and_query = Some(
        req.uri()
            .path_and_query()
            .unwrap()
            .to_string()
            .replace("/a/", "/")
            .parse()
            .unwrap(),
    );
    *req.uri_mut() = hyper::Uri::from_parts(uri_parts).unwrap();
}

fn uri_rewrite_lsp(req: &mut Request<Body>) {
    let mut uri_parts = req.uri().clone().into_parts();
    uri_parts.path_and_query = Some(
        req.uri()
            .path_and_query()
            .unwrap()
            .to_string()
            .replace("/a/api/v1/lsp/lsp", "")
            .parse()
            .unwrap(),
    );
    *req.uri_mut() = hyper::Uri::from_parts(uri_parts).unwrap();
}

fn uri_rewrite_workspace_or_tensorboard(req: &mut Request<Body>) {
    let mut uri_parts = req.uri().clone().into_parts();
    uri_parts.path_and_query = Some(
        req.uri()
            .path_and_query()
            .unwrap()
            .to_string()
            .replace("/workspace", "/")
            .replace("/tensorboard", "/")
            .parse()
            .unwrap(),
    );
    *req.uri_mut() = hyper::Uri::from_parts(uri_parts).unwrap();
}

fn rsp_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

#[cfg(feature = "license")]
fn rsp_403(reason: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header(hyper::http::header::CONTENT_TYPE, "text/html".to_string())
        .body(Body::from(format!(
            "<h1>HTTP 403</h1><h2>token is invalid or missing</h2><h3>{reason}</h3>"
        )))
        .unwrap()
}
