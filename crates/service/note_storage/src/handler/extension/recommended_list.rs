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

pub async fn recommended_list(
    Query(req): Query<ListReq>,
) -> Result<Rsp<Vec<ExtensionResp>>, ErrorTrace> {
    tracing::info!("recommended_list api run");
    let installed_extensions = business::path_tool::user_extensions_path(req.team_id, req.user_id);
    let installed_config_path =
        std::path::Path::new(&installed_extensions).join("extensions_config.json");

    let recommended_extensions = business::path_tool::recommended_extensions();
    let recommended_config_path =
        std::path::Path::new(&recommended_extensions).join("extensions_config.json");

    let mut recommended_content = super::get_extensions_config(recommended_config_path)?;

    for content in recommended_content.iter_mut() {
        let url = format!(
            "{}/{}-{}/",
            recommended_extensions.to_str().unwrap(),
            content.name,
            content.version
        );
        content.url = Some(url);
    }
    let mut resp = Vec::new();
    if let Ok(installed_content) = super::get_extensions_config(installed_config_path) {
        'a: for i in &recommended_content {
            for j in &installed_content {
                if i == j {
                    continue 'a;
                }
            }
            resp.push(i.clone())
        }
    };

    Ok(Rsp::success(resp))
}
