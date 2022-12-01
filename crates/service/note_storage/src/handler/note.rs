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

use axum::extract::Multipart;
use common_model::service::rsp::Rsp;
use common_tools::io_tool::file_writer::FileState;

use crate::handler::note_handler;

#[axum_macros::debug_handler]
pub async fn upload_file(
    file_state: axum::extract::State<FileState>,
    multipart: Multipart,
) -> Result<Rsp<String>, err::ErrorTrace> {
    let writer = file_state.writer.clone();

    let res = note_handler::upload_file_handler(multipart, writer).await?;
    Ok(Rsp::success(res))
}
