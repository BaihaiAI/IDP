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
use common_model::service::rsp::Rsp;
use err::ErrorTrace;
use serde::Deserialize;
use serde::Serialize;

use super::CatRspBody;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullPathCatReq {
    pub path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullPathCatRsp {
    pub length: usize,
    #[serde(flatten)]
    pub content: CatRspBody,
}

/// get file content outside workspace, e.g. cat /lib/python3.9/site-packages source when go to define
pub async fn full_path_cat(
    Query(cat_req): Query<FullPathCatReq>,
) -> Result<Rsp<FullPathCatRsp>, ErrorTrace> {
    tracing::info!("full path cat : {:?}", cat_req.path);
    let content = tokio::fs::read_to_string(cat_req.path).await?;
    Ok(Rsp::success(FullPathCatRsp {
        length: content.len(),
        content: CatRspBody::Text(content),
    }))
}
