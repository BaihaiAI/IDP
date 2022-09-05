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
use common_model::Rsp;
use err::ErrorTrace;

use super::models::ExtensionReq;

pub async fn detail(Query(req): Query<ExtensionReq>) -> Result<Rsp<String>, ErrorTrace> {
    let extensions_path = business::path_tool::user_extensions_path(req.team_id, req.user_id);
    tracing::info!("run extensions detail api");
    let detail_extension = std::path::Path::new(&extensions_path)
        .join(req.name)
        .join(req.version)
        .join("README.md");
    tracing::info!("detail_extension:{detail_extension:?}");
    match std::fs::read_to_string(detail_extension) {
        Ok(resp) => Ok(Rsp::success(resp)),
        Err(_) => Ok(Rsp::success("null".to_string())),
    }
}
