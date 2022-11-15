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

use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct Rsp<T>
where
    T: Serialize,
{
    /// T is () means data is null in response json
    pub data: T,
    // #[serde(skip)]
    // pub http_status_code: u16,
    pub code: u32,
    pub message: String,
}

pub const CODE_SUCCESS: u32 = 21_000_000;
pub const CODE_FAIL: u32 = 51_000_000;

impl Rsp<()> {
    pub fn success_without_data() -> Self {
        Self::success(())
    }
    pub fn error_code_msg(code: u32, message: &str) -> Self {
        Rsp {
            data: (),
            code,
            message: message.to_string(),
        }
    }
}

impl<T> Rsp<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Rsp {
            data,
            code: 21_000_000,
            message: "success".to_string(),
        }
    }
    pub fn code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }
    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn http_status_code(&self) -> u16 {
        if self.code < 40_000_000 {
            200
        } else if self.code < 50_000_000 {
            400
        } else {
            500
        }
    }
}

#[cfg(feature = "axum")]
impl<T> axum::response::IntoResponse for Rsp<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
