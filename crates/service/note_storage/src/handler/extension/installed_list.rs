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

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;

use super::models::ExtensionResp;
use super::models::ListReq;

pub async fn installed_list(
    Query(req): Query<ListReq>,
) -> Result<Rsp<Vec<ExtensionResp>>, ErrorTrace> {
    let extensions_path = business::path_tool::user_extensions_path(req.team_id, req.user_id);

    let extension_config_path =
        std::path::Path::new(&extensions_path).join("extensions_config.json");

    if !extension_config_path.exists() {
        std::fs::create_dir_all(&extensions_path)?;
        std::fs::File::create(&extension_config_path)?;
    }

    let content = super::get_extensions_config(&extension_config_path)?;

    Ok(Rsp::success(content))
}
