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

#![deny(warnings)]
#![deny(unused_crate_dependencies)]
mod backtrace_capture;
pub type Result<T> = std::result::Result<T, ErrorTrace>;

// a fully backtrace support version of anyhow require enable debuginfo in release
#[derive(Debug)]
pub struct ErrorTrace {
    // inner: Box<dyn std::error::Error>
    pub message: String,
    pub backtrace: Vec<String>,
    pub err_code: u32,
}

impl ErrorTrace {
    pub const CODE_WARNING: u32 = 41_000_000;
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            backtrace: backtrace_capture::backtrace_capture(),
            err_code: 51_040_000,
        }
    }
    pub fn code(mut self, code: u32) -> Self {
        self.err_code = code;
        self
    }
    pub fn http_status_code(&self) -> u16 {
        u16::try_from(self.err_code).unwrap_or(200)
    }
    #[cfg(not)]
    pub fn http_status_code(&self) -> u16 {
        if self.err_code > 51_000_000 {
            500
        } else if self.err_code > 41_000_000 {
            400
        } else {
            200
        }
    }
}

impl std::fmt::Display for ErrorTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// conflict impl std::error::Error for ErrorTrace {}
impl<E: std::error::Error + 'static> From<E> for ErrorTrace {
    fn from(err: E) -> Self {
        Self::new(&err.to_string())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ErrorTrace {
    fn into_response(self) -> axum::response::Response {
        tracing::warn!("{self:#?}");

        let mut resp = axum::Json(serde_json::json!({
            "code": self.err_code,
            "message": self.message,
            "data": ()
        }))
        .into_response();
        *resp.status_mut() = axum::http::StatusCode::from_u16(self.http_status_code()).unwrap();
        resp
    }
}
