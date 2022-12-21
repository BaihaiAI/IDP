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

use std::collections::HashSet;

use axum::Json;
use common_model::Rsp;
use err::ErrorTrace;
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;

use super::models::ExtensionConfig;
use super::models::InitReq;

lazy_static! {
    static ref INIT_EXTENSION: HashSet<&'static str> = {
        let a = std::fs::read_to_string("/opt/config/extension_config.json").unwrap();
        let extension_config: ExtensionConfig = serde_json::from_str(&a).unwrap();
        let mut m: HashSet<&'static str> = HashSet::new();
        extension_config.init.into_iter().for_each(|x| {
            m.insert(Box::leak(x.into_boxed_str()));
        });
        m
    };
}

lazy_static! {
    static ref AIGC_INIT_EXTENSION: HashSet<&'static str> = {
        let a = std::fs::read_to_string("/opt/config/extension_config.json").unwrap();
        let extension_config: ExtensionConfig = serde_json::from_str(&a).unwrap();
        let mut m: HashSet<&'static str> = HashSet::new();
        extension_config
            .aigc_init
            .unwrap()
            .into_iter()
            .for_each(|x| {
                m.insert(Box::leak(x.into_boxed_str()));
            });
        m
    };
}

pub async fn init_install(Json(payload): Json<InitReq>) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = payload.team_id;
    let user_id = payload.user_id;
    tracing::info!("team_id:{team_id},user_id:{user_id},run init_install");
    init_install_handler(team_id, user_id, payload.nav_type).await
}
enum NavType {
    Idp,
    Aigc,
}
pub async fn init_install_handler(
    team_id: u64,
    user_id: u64,
    nav_type: Option<String>,
) -> Result<Rsp<()>, ErrorTrace> {
    let nav_type = if nav_type.is_none() {
        NavType::Idp
    } else {
        match nav_type.unwrap().as_str() {
            "AIGC" => NavType::Aigc,
            _ => NavType::Idp,
        }
    };
    let installed_extensions_path = business::path_tool::user_extensions_path(team_id, user_id);

    let extension_config_path =
        std::path::Path::new(&installed_extensions_path).join("extensions_config.json");

    if !extension_config_path.exists() {
        tokio::fs::create_dir_all(&installed_extensions_path).await?;
    } else {
        tokio::fs::remove_dir_all(&installed_extensions_path).await?;
        tokio::fs::create_dir_all(&installed_extensions_path).await?;
    }

    let recommended_extensions_path = business::path_tool::recommended_extensions();

    let recommended_config_path = recommended_extensions_path.join("extensions_config.json");

    let mut recommended_content = super::get_extensions_config(recommended_config_path).await?;

    for content in recommended_content
        .iter_mut()
        .filter(|content| match nav_type {
            NavType::Idp => INIT_EXTENSION.contains(content.name.as_str()),
            NavType::Aigc => AIGC_INIT_EXTENSION.contains(content.name.as_str()),
        })
    {
        let extension_path = recommended_extensions_path.join(&content.name);
        common_tools::command_tools::copy(
            extension_path.to_str().unwrap(),
            &installed_extensions_path,
        )
        .await?;

        let url = format!(
            "{}/{}/{}/",
            installed_extensions_path, content.name, content.version
        );
        content.url = Some(url);
    }

    let content_str = serde_json::to_string(&recommended_content)?;
    let mut f = tokio::fs::File::create(extension_config_path).await?;
    f.write_all(content_str.as_bytes()).await?;

    Ok(Rsp::success_without_data())
}
