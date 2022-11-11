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

mod cookie_team_id;
mod ipynb_path;

pub use business::business_term::ProjectId;
pub use cookie_team_id::team_id_from_cookie;
pub use err::ErrorTrace;
pub use hyper::header;
pub use hyper::Body;
pub use hyper::Request;
pub use hyper::Response;
pub use hyper::StatusCode;
pub use ipynb_path::inode_from_query_string;
pub use kernel_common::typedef::CellId;
pub use kernel_common::typedef::Inode;
pub use kernel_common::typedef::TeamId;
pub use serde::Deserialize;
pub use serde::Serialize;

pub use crate::app_context::AppContext;
pub use crate::app_context::KernelEntryOps;
pub use crate::error::Error;
pub use crate::kernel_entry::kernel_operate::KernelOperate;
pub use crate::kernel_entry::kernel_state::State;
pub use crate::kernel_entry::KernelEntry;
pub use crate::resp::Resp;

pub async fn de_hyper_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Result<T, Error> {
    let body = req.into_body();
    let body = hyper::body::to_bytes(body).await?.to_vec();
    Ok(serde_json::from_slice::<T>(&body)?)
}
