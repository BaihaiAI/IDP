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

use std::fmt::Display;
use std::fmt::Formatter;

use common_model::service::rsp::Rsp;
pub use err::ErrorTrace;

/// defined the global error that service need to return.

#[cfg(not)]
mod status {
    pub const INVALID_REQUEST_PARAM_ERROR_CODE: u32 = 51_002_002;
    pub const INVALID_REQUEST_PARAM_ERROR_MSG: &str = "invalid request parameter!";
    pub const UNDEFINED_ERROR_CODE: u32 = 51_001_002;
}

#[derive(Debug)]
// #[allow(deprecated)]
// #[deprecated]
pub enum IdpGlobalError {
    NoteError(String),
    ErrorCodeMsg(u32, String),
}

impl Display for IdpGlobalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorTrace: => {:?}", self)
    }
}

impl axum::response::IntoResponse for IdpGlobalError {
    fn into_response(self) -> axum::response::Response {
        let res: Rsp<()> = match self {
            IdpGlobalError::NoteError(message) => Rsp {
                data: (),
                code: 51_000_000,
                message,
            },
            IdpGlobalError::ErrorCodeMsg(code, message) => Rsp::error_code_msg(code, &message),
        };
        res.into_response()
    }
}
impl From<std::io::Error> for IdpGlobalError {
    fn from(err: std::io::Error) -> Self {
        ErrorTrace::from(err).into()
    }
}
impl From<reqwest::Error> for IdpGlobalError {
    fn from(err: reqwest::Error) -> Self {
        ErrorTrace::from(err).into()
    }
}

impl From<ErrorTrace> for IdpGlobalError {
    fn from(err: ErrorTrace) -> Self {
        tracing::warn!("error_trace: {:#?}", err);
        IdpGlobalError::ErrorCodeMsg(err.err_code, err.message)
    }
}
