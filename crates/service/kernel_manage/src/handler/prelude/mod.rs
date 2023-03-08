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

// mod cookie_team_id;
// mod ipynb_path;

pub use axum::extract::Json;
pub use axum::extract::Query;
pub use axum::extract::State;
pub use business::business_term::ProjectId;
pub use common_model::service::rsp::Rsp;
pub use err::ErrorTrace;
// pub use hyper::header;
// pub use hyper::Body;
// pub use hyper::Request;
// pub use hyper::Response;
// pub use hyper::StatusCode;
pub use kernel_common::typedef::CellId;
pub use kernel_common::typedef::Inode;
pub use kernel_common::typedef::TeamId;
pub use serde::Deserialize;
pub use serde::Serialize;

pub use crate::app_context::AppContext;
pub use crate::app_context::KernelEntryOps;
pub use crate::error::Error;
pub use crate::kernel_entry::kernel_operate::KernelOperate;
pub use crate::kernel_entry::kernel_state::KernelState;
pub use crate::kernel_entry::KernelEntry;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
// #[serde(rename_all = "camelCase")]
pub struct InodeReq {
    pub inode: Inode,
}

#[cfg(not)]
pub async fn de_hyper_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Result<T, Error> {
    let body = req.into_body();
    let body = hyper::body::to_bytes(body).await?.to_vec();
    Ok(serde_json::from_slice::<T>(&body)?)
}
