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

use axum::body::Full;
use axum::body::{self};
use axum::extract::Path;
use axum::http::header;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use err::ErrorTrace;
use tokio::io::AsyncReadExt;

pub async fn load(Path(path): Path<String>) -> Result<impl IntoResponse, ErrorTrace> {
    tracing::info!("access extensions load api path:{}", path);
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();
    tracing::info!("{:?}", mime_type);
    let mime_type_str = mime_type.to_string();

    if mime_type_str.starts_with("image") {
        let mut buf = Vec::new();
        let mut f = tokio::fs::File::open(&path).await?;
        f.read_to_end(&mut buf).await?;
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&mime_type_str).unwrap(),
            )
            .body(body::boxed(Full::from(buf)))
            .unwrap());
    }

    if mime_type_str.starts_with("application/gzip") {
        let mut buf = Vec::new();
        let mut f = tokio::fs::File::open(&path).await?;
        f.read_to_end(&mut buf).await?;
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&mime_type_str).unwrap(),
            )
            .header(
                header::CONTENT_ENCODING,
                HeaderValue::from_str("gzip").unwrap(),
            )
            .body(body::boxed(Full::from(buf)))
            .unwrap());
    }

    match std::fs::read_to_string(&path) {
        Ok(body) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&mime_type_str).unwrap(),
            )
            .header(
                header::ACCEPT_RANGES,
                HeaderValue::from_str("bytes").unwrap(),
            )
            .header(
                header::CONNECTION,
                HeaderValue::from_str("keep-alive").unwrap(),
            )
            .header(
                header::ACCESS_CONTROL_MAX_AGE,
                HeaderValue::from_str("7200").unwrap(),
            )
            .body(body::boxed(Full::from(body)))
            .unwrap()),
        Err(_) => {
            tracing::error!("{path:?} load failed");
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(axum::body::Empty::new()))
                .unwrap())
        }
    }
}
