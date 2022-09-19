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

use std::io::Write;

use axum::extract::Query;
use common_model::Rsp;
use err::ErrorTrace;

use super::models::ListReq;

pub async fn init_install(Query(req): Query<ListReq>) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = req.team_id;
    let user_id = req.user_id;
    init_install_handler(team_id, user_id).await
}

pub async fn init_install_handler(team_id: u64, user_id: u64) -> Result<Rsp<()>, ErrorTrace> {
    let installed_extensions_path = business::path_tool::user_extensions_path(team_id, user_id);

    let extension_config_path =
        std::path::Path::new(&installed_extensions_path).join("extensions_config.json");

    if !extension_config_path.exists() {
        std::fs::create_dir_all(&installed_extensions_path)?;
    }

    let mut recommended_extensions_path = business::path_tool::recommended_extensions()
        .to_str()
        .unwrap()
        .to_owned();

    recommended_extensions_path += "/.";

    common_tools::command_tools::copy(&recommended_extensions_path, &installed_extensions_path)?;

    let recommended_config_path =
        std::path::Path::new(&recommended_extensions_path).join("extensions_config.json");

    let mut recommended_content = super::get_extensions_config(recommended_config_path)?;

    for content in recommended_content.iter_mut() {
        let url = format!(
            "{}/{}/{}/",
            installed_extensions_path, content.name, content.version
        );
        content.url = Some(url);
    }

    let content_str = serde_json::to_string(&recommended_content)?;
    let mut f = std::fs::File::create(extension_config_path).unwrap();
    f.write_all(content_str.as_bytes()).unwrap();

    Ok(Rsp::success_without_data())
}
