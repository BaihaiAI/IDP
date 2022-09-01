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
use common_model::service::rsp::Rsp;
use common_tools::io_tool::file_writer::FileState;

use crate::handler::note_handler;

const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10; // 10MB

pub async fn upload_file(
    ContentLengthLimit(multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
    file_state: axum::extract::Extension<FileState>,
) -> Result<Rsp<String>, err::ErrorTrace> {
    let writer = file_state.writer.clone();

    let res = note_handler::upload_file_handler(multipart, writer).await?;
    Ok(Rsp::success(res))
}
