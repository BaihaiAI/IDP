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
use axum::extract::State;
use cache_io::CacheService;
use common_model::entity::notebook::Notebook;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;

use crate::app_context::AppContext;

pub mod file_mime_magic;
pub mod full_path_cat;
pub mod get_zip_file_list;
pub mod load;

use std::sync::Arc;
use std::sync::Mutex;

pub async fn cat(
    Query(cat_req): Query<CatReq>,
    State(app_context): State<AppContext>,
) -> Result<Rsp<CatRsp>, ErrorTrace> {
    let now = std::time::Instant::now();
    let ret = cat_(
        cat_req.path.clone(),
        cat_req.team_id,
        cat_req.project_id,
        &app_context.redis_cache,
    )
    .await;

    tracing::info!("cat {} time cost {:?}", cat_req.path.clone(), now.elapsed());
    ret
}

/*
INTEGRATION_TEST=1 GATEWAY_PORT=3000 cargo test --package note_storage --lib -- handler::content::cat::cat_demo_ipynb_it --exact --nocapture
*/
#[test]
#[cfg(not)]
fn cat_demo_ipynb_it() {
    let ctx = test_runner::IntegrationTestCtx::get();
    let path = "demo.ipynb";
    let resp = ctx
        .client
        .get(format!(
            "{}&path={path}",
            ctx.note_storage_api_url("/content/cat")
        ))
        .send()
        .unwrap();
    assert!(
        resp.status().is_success(),
        "{} {}",
        resp.status().as_u16(),
        resp.text().unwrap()
    );
    let rsp = resp
        .json::<serde_json::Map<String, serde_json::Value>>()
        .unwrap();
    assert_eq!(
        rsp["code"].as_u64().unwrap(),
        common_model::service::rsp::CODE_SUCCESS as u64
    );
    let rsp = serde_json::from_value::<CatRsp>(rsp["data"].to_owned()).unwrap();
    assert_eq!(rsp.mime, "application/x-ipynb+json");
    assert!(matches!(rsp.content, CatRspBody::Notebook(_)));
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatReq {
    pub path: String,
    pub project_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct CatRsp {
    // TODO: remove last_modified and length, because we already have them in content field
    pub mime: String,
    pub length: usize,
    #[serde(flatten)]
    pub content: CatRspBody,
    pub last_modified: String,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "camelCase")]
#[serde(tag = "contentType", content = "content")]
pub enum CatRspBody {
    Notebook(Notebook),
    Text(String),
    Zip(Vec<Arc<Mutex<get_zip_file_list::ZipNode>>>),
}

pub async fn cat_(
    path_str: String,
    team_id: u64,
    project_id: u64,
    redis_cache: &CacheService,
) -> Result<Rsp<CatRsp>, ErrorTrace> {
    let path = business::path_tool::get_store_full_path(team_id, project_id, &path_str);
    let mime = file_mime_magic::get_mime_type(&path)?;

    // TODO(code_review): last_modified should use std::fs::metadata.modified instead of now
    let now = chrono::Local::now();
    let last_modified = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let inode = kernel_common::message::header::kernel_header_hash(project_id, &path_str, -1);
    let cat_data =
        file_mime_magic::cat_file_content_by_mime(path, &mime, project_id, inode, redis_cache)
            .await?;
    Ok(Rsp::success(CatRsp {
        mime,
        length: 0,
        last_modified,
        content: cat_data,
    }))
}
