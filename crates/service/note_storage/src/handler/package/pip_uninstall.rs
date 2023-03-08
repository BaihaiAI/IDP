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

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Json;
use axum::extract::Query;
use axum::extract::State;
use common_model::Rsp;
use err::ErrorTrace as Error;
use tokio::sync::Mutex;

use crate::api_model::TeamIdQueryString;

type ProjectInfoMap = Arc<Mutex<HashMap<String, HashMap<String, String>>>>;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipUninstallReq {
    pub package_name: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
    pub version: String,
}

pub async fn pip_uninstall(
    State((_pg_option, project_info_map)): State<(Option<sqlx::PgPool>, ProjectInfoMap)>,
    Query(TeamIdQueryString { team_id }): Query<TeamIdQueryString>,
    Json(PipUninstallReq {
        package_name,
        project_id,
        version: _,
    }): Json<PipUninstallReq>,
) -> Result<Rsp<()>, Error> {
    let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
    let py_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);

    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m")
        .arg("pip")
        .arg("uninstall")
        .arg("-y")
        .arg(&package_name);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output().await?;
    let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };
    let stderr = unsafe { String::from_utf8_unchecked(output.stderr) };
    tracing::info!("stdout = {stdout}");
    tracing::warn!("stderr = {stderr}");
    if !output.status.success() {
        tracing::error!("command not success");
        return Err(Error::new(&stderr));
    }

    {
        let project_info_key = format!("{team_id}+{project_id}");
        if let Some(package_map) = project_info_map.lock().await.get_mut(&project_info_key) {
            package_map.remove(&package_name);
        }
    }

    Ok(Rsp::success(()))
}
