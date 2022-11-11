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
use axum::response::IntoResponse;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use common_tools::cookies_tools::Cookies;
use err::ErrorTrace;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadWorkspaceFile {
    pub path: String,
    pub project_id: u64,
    // pub team_id: u64,
    pub output_type: Option<String>,
}

pub async fn download(
    Query(req): Query<DownloadWorkspaceFile>,
    axum::TypedHeader(cookies): axum::TypedHeader<Cookies>,
) -> impl IntoResponse {
    let team_id = get_cookie_value_by_team_id(cookies);
    let abs_path = business::path_tool::get_store_full_path(team_id, req.project_id, req.path);
    tracing::info!("-->download: {abs_path:?}");

    download_file(abs_path, "application/octet-stream;charset=UTF-8").await
}

pub async fn download_file(
    abs_path: std::path::PathBuf,
    content_type: &'static str,
) -> Result<impl IntoResponse, ErrorTrace> {
    if !abs_path.exists() {
        return Err(ErrorTrace::new("download file not exist"));
    }
    let file_name = abs_path.file_name().unwrap().to_str().unwrap();
    let file = tokio::fs::File::open(&abs_path).await?;

    let stream = tokio_util::io::ReaderStream::new(file);

    // frontend has urlencode path, so the header must all ASCII
    let attachment_str = format!("attachment; filename=\"{file_name}\"");
    let body = axum::body::StreamBody::new(stream);
    let mut resp = axum::response::IntoResponse::into_response(body);
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(content_type),
    );
    resp.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment_str).unwrap(),
    );
    Ok(resp)
}
