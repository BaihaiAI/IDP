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

#[cfg(not)]
macro_rules! await_timeout {
    ($expr:expr) => {
        match tokio::time::timeout(std::time::Duration::from_secs(10), $expr).await {
            Ok(val) => Ok(val),
            Err(err) => {
                let err = format!("await timeout {err}");
                tracing::error!("{}", err);
                Err(crate::error::Error::new(&err))
            }
        }
    };
}
