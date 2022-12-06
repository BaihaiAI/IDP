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

use axum::extract::Query;
use axum::http::header;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use err::ErrorTrace;
use serde::Deserialize;

use super::file_mime_magic;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadReq {
    pub path: String,
    pub project_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
}

pub async fn load(Query(cat_req): Query<LoadReq>) -> Result<impl IntoResponse, ErrorTrace> {
    let now = std::time::Instant::now();
    let ret = load_handler(cat_req.path.clone(), cat_req.team_id, cat_req.project_id).await;
    tracing::info!(
        "load {} time cost {:?}",
        cat_req.path.clone(),
        now.elapsed()
    );
    ret
}

pub async fn load_handler(
    path_str: String,
    team_id: u64,
    project_id: u64,
) -> Result<impl IntoResponse, ErrorTrace> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, &path_str);
    let mime = file_mime_magic::get_mime_type(&path)?;

    let f = tokio::fs::File::open(&path).await?;
    let stream = tokio_util::io::ReaderStream::new(f);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, HeaderValue::from_str(&mime).unwrap())
        .body(axum::body::StreamBody::new(stream))
        .unwrap())
}
