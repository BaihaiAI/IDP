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

use axum::body::StreamBody;
use axum::extract::Json;
use axum::extract::State;
use common_model::service::rsp::CODE_FAIL;
use common_model::Rsp;
use futures::stream::Stream;
use tokio::sync::Mutex;

type ProjectInfoMap = Arc<Mutex<HashMap<String, HashMap<String, String>>>>;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PipInstallReq {
    pub package_name: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    pub version: String,
}

/// TODO cancel pip install command if frontend cancel request
#[axum_macros::debug_handler]
pub async fn pip_install(
    State((_pg_option, project_info_map)): State<(Option<sqlx::PgPool>, ProjectInfoMap)>,
    Json(PipInstallReq {
        package_name,
        project_id,
        team_id,
        version,
    }): Json<PipInstallReq>,
) -> StreamBody<impl Stream<Item = Result<String, std::io::Error>>> {
    // ) -> impl IntoResponse {
    let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
    let py_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);

    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m").arg("pip").arg("install");
    // if business::kubernetes::is_k8s() {
    //     cmd.arg("-U").arg("--target").arg(install_dir);
    // }
    cmd.arg(format!("{}=={}", package_name, version));
    tracing::info!("cmd = {cmd:?}");
    let (tx, rx) = tokio::sync::mpsc::channel(2);
    tokio::spawn(async move {
        match cmd.output().await {
            Ok(output) => {
                if output.status.success() {
                    {
                        let project_info_key = format!("{team_id}+{project_id}");
                        if let Some(package_map) =
                            project_info_map.lock().await.get_mut(&project_info_key)
                        {
                            package_map.insert(package_name, version);
                        }
                    }
                    if let Err(err) = tx
                        .send(serde_json::to_string(&Rsp::success(())).unwrap())
                        .await
                    {
                        tracing::error!("{err}");
                    }
                } else if let Err(err) = tx
                    .send(
                        serde_json::to_string(
                            &Rsp::success(())
                                .message(&String::from_utf8_lossy(&output.stderr))
                                .code(CODE_FAIL),
                        )
                        .unwrap(),
                    )
                    .await
                {
                    tracing::error!("{err}");
                }
            }
            Err(err) => {
                let err = serde_json::to_string(
                    &Rsp::success(()).message(&err.to_string()).code(CODE_FAIL),
                )
                .unwrap();
                if let Err(err) = tx.send(format!("err\n{err}")).await {
                    tracing::error!("{err}");
                }
            }
        }
    });
    StreamBody::new(futures::stream::unfold(
        (rx, false),
        |(mut rx, is_eof)| async move {
            if is_eof {
                return None;
            }
            match tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await {
                Ok(output) => output.map(|output| (Ok(output), (rx, true))),
                Err(_timeout) => Some((Ok("keep_alive\n".to_string()), (rx, false))),
            }
        },
    ))
}
