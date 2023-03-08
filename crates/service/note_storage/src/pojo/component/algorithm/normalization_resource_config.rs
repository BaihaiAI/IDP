// Copyright 2023 BaihaiAI, Inc.
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

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/1/18
 * Time: 15:28
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizationResourceConfig {
    pub cpu: String,
}
