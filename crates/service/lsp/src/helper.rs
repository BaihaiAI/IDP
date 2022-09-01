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

use std::env;
use std::net::SocketAddr;
use std::process::Command;

use tokio::sync::oneshot::Sender;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::handshake::server::Response;
use tokio_tungstenite::tungstenite::http;
use tokio_tungstenite::tungstenite::http::StatusCode;

pub fn get_python_site(python_bin: &str) -> Result<String, String> {
    let mut cmd = Command::new(python_bin);
    cmd.arg("-c")
        .arg("import sys; print(sys.path[-1])")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                if let Ok(with_newline) = String::from_utf8(output.stdout) {
                    Ok(String::from(with_newline.trim_end()))
                } else {
                    Err(String::from("Could not get python site"))
                }
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(err) => {
            let err = format!("{cmd:?} {err}");
            tracing::error!("{err}");
            Err(err)
        }
    }
}

pub fn env_or_default(key: &str, default_val: &str) -> String {
    if let Some(v) = env::var_os(key) {
        v.into_string().expect("change osString to string failed")
    } else {
        String::from(default_val)
    }
}

pub fn count_u8_char(aim: &[u8], u: &u8) -> usize {
    aim.iter().filter(|c| c.eq(&u)).count()
}

pub fn complete_u8(aim: &[u8]) -> bool {
    // todo: u8 trim needed
    let trimmed = aim;
    trimmed.ends_with(&[b'}'])
        && count_u8_char(trimmed, &b'{') > 0
        && count_u8_char(trimmed, &b'{') == count_u8_char(trimmed, &b'}')
}

pub const WEBSOCKET_PROTO_HEADER: &str = "sec-websocket-protocol";
pub type ResponseError = http::Response<Option<String>>;
pub type HandshakeResponse = Result<Response, ResponseError>;

pub fn handshake_err<E: std::fmt::Display>(err: E) -> ResponseError {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Some(format!("{}", err)))
        .expect("Response builder error")
}

pub fn handle_handshake(
    tx: Sender<url::Url>,
    address: SocketAddr,
) -> impl FnOnce(&Request, Response) -> HandshakeResponse {
    move |req: &Request, mut response: Response| {
        let uri = req.uri();
        let path = uri.path();
        let query = uri.query();
        let mut url = url::Url::parse(&format!("ws://{}", address)).map_err(handshake_err)?;
        url.set_path(path);
        url.set_query(query);

        tracing::debug!("New ws handshake for {}", url);

        if let Some(sp) = req.headers().get(WEBSOCKET_PROTO_HEADER) {
            let headers = response.headers_mut();
            headers.append(WEBSOCKET_PROTO_HEADER, sp.clone());
        }

        tx.send(url.clone()).map_err(handshake_err)?;

        Ok(response)
    }
}

/* ------------------------------  test below ------------------------------------- */

#[cfg(target_os = "macos")]
#[test]
fn test_get_python_site() {
    if let Ok(x) = get_python_site(&String::from(
        "/Users/liuzhe/miniconda3/envs/torch_env/bin/python",
    )) {
        println!("{:?}", x);
        assert!(x.ends_with("site-packages"));
    }
}

#[test]
fn test_count_u8() {
    let inner = String::from("kdkdkd{{kdkdk}}kdkd");
    let x = inner.as_bytes();
    assert_eq!(count_u8_char(x, &b'k'), 8);
    assert_eq!(count_u8_char(x, &b'd'), 7);
    assert_eq!(count_u8_char(x, &b'{'), 2);
    assert_eq!(count_u8_char(x, &b'}'), 2);
}

#[test]
fn test_complete_u8() {
    assert!(!complete_u8(String::from("kdkdkdi").as_bytes()));
    assert!(!complete_u8(String::from("{}}").as_bytes()));
    assert!(!complete_u8(String::from("{{{}}").as_bytes()));
    assert!(complete_u8(String::from("{{kkkk}}").as_bytes()));
    assert!(complete_u8(String::from("{{kkkk}mmm}").as_bytes()));
}
