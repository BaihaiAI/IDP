// Copyright 2023 BaihaiAI, Inc.
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

use axum::{extract::Json, TypedHeader};
use common_model::Rsp;
use err::ErrorTrace;
use kernel_common::spawn_kernel_process::Resource;

#[derive(serde::Deserialize)]
pub struct Req {
    project_id: u32,
    resource: Resource
}

pub async fn start_runtime_pod(
    Json(req): Json<Req>,
    TypedHeader(cookies): TypedHeader<axum::headers::Cookie>,
) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = cookies.get("teamId").unwrap_or_default().parse::<i64>()?;
    let user_id = cookies.get("userId").unwrap_or_default().parse::<i64>()?;
    
    todo!()
}
