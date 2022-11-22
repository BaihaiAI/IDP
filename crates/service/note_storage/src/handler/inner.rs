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

use axum::extract::Extension;
use axum::extract::Query;
use common_model::service::rsp::Rsp;

pub async fn version() -> Rsp<GitVersionRep> {
    Rsp::success(GitVersionRep {
        version: env!("VERSION").to_string(),
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitVersionRep {
    pub version: String,
}

#[derive(serde::Deserialize)]
pub struct QueryString {
    level: String,
}

// GET /idp-note-rs/inner/change_log?level=info,sqlx=warn,note_storage=debug,cache_io=debug
pub async fn change_log_level(
    Query(qs): Query<QueryString>,
    Extension(reload_handle): Extension<logger::ReloadLogLevelHandle>,
) -> String {
    tracing::info!("qs.level = {}", qs.level);
    match qs.level.parse::<logger::EnvFilter>() {
        Ok(env_filter) => {
            reload_handle.modify(|filter| *filter = env_filter).unwrap();
            "ok".to_string()
        }
        Err(err) => err.to_string(),
    }
}
