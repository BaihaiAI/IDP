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

use serde::Deserialize;

/// Copyright @baihai 2021
/// @author Kim Huang
/// @date 2022/4/28 am.10:27
///

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentEnvReq {
    pub project_id: u64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvClone {
    pub origin_name: String,
    pub target_name: String,
}
