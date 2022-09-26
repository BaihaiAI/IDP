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

use axum::extract::Path;
use axum::http::header;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use err::ErrorTrace;

pub async fn load(Path(path): Path<String>) -> Result<impl IntoResponse, ErrorTrace> {
    tracing::info!("access extensions load api path:{}", path);
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();
    tracing::info!("{:?}", mime_type);
    let mime_type_str = mime_type.to_string();

    if mime_type_str.starts_with("image") {
        let f = tokio::fs::File::open(&path).await?;
        let stream = tokio_util::io::ReaderStream::new(f);
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&mime_type_str).unwrap(),
            )
            .body(axum::body::StreamBody::new(stream))
            .unwrap());
    }

    if mime_type_str.starts_with("application/gzip") {
        let f = tokio::fs::File::open(&path).await?;
        let stream = tokio_util::io::ReaderStream::new(f);
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(&mime_type_str).unwrap(),
            )
            .header(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"))
            .body(axum::body::StreamBody::new(stream))
            .unwrap());
    }

    let f = tokio::fs::File::open(&path).await?;
    let stream = tokio_util::io::ReaderStream::new(f);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(&mime_type_str).unwrap(),
        )
        .header(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"))
        .header(header::CONNECTION, HeaderValue::from_static("keep-alive"))
        .header(
            header::ACCESS_CONTROL_MAX_AGE,
            HeaderValue::from_static("7200"),
        )
        .body(axum::body::StreamBody::new(stream))
        .unwrap())
}
