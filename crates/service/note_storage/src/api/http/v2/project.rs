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

use axum::extract::ContentLengthLimit;
use axum::extract::Multipart;
use axum::Json;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use common_tools::io_tool::file_writer::FileState;

use crate::api_model::project::ProjectId;
use crate::handler::project_handler;

const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 1024 * 10; // 10GB

pub async fn new(
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
    file_state: axum::extract::Extension<FileState>,
) -> Result<Rsp<String>, err::ErrorTrace> {
    let writer = file_state.writer.clone();
    let rsp = project_handler::new_project(multipart, writer).await?;
    Ok(Rsp::success(rsp))
}

pub async fn delete(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<ProjectId>,
) -> Result<Rsp<()>, err::ErrorTrace> {
    tracing::debug!("access project delete api");
    let id = payload.id;

    // let team_id = 0u64;
    let team_id = get_cookie_value_by_team_id(cookies);
    project_handler::delete(team_id, id).await
}

#[cfg(not)]
pub async fn ray_chown_fix_one_time(
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::debug!("access ray_chown_fix_one_time api");
    project_handler::ray_chown_fix_one_time(pg_pool).await
}
