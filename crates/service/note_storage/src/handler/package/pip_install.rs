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

use axum::body::StreamBody;
use axum::extract::Json;
use common_model::service::rsp::CODE_FAIL;
use common_model::Rsp;
use futures::stream::Stream;

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

/*
curl --location --request POST 'http://localhost:8082/api/v2/idp-note-rs/package/install' \
--header 'Content-Type: application/json' \
--data-raw '{
    "teamId": "1565387256278454272",
    "projectId": "100",
    "packageName": "tensorflow-gpu",
    "version": "2.9.2"
}'
*/
/// TODO cancel pip install command if frontend cancel request
pub async fn pip_install(
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

    let install_dir = match business::pip_install::ensure_python2user_install_dir_exist(&py_path) {
        Ok(install_dir) => install_dir,
        Err(err) => {
            tracing::error!("{err}");
            // FIXME should return stream::once but got type error
            "/usr/lib".to_string()
            /*
            return     StreamBody::new(futures::stream::unfold(
                false,
                |mut is_eof| async move {
                    if is_eof {
                        return None;
                    }
                    let rsp =
                        serde_json::to_string(
                            &Rsp::success(())
                                .message(&err)
                                .code(CODE_FAIL),
                        )
                        .unwrap();
                    Some((Ok(rsp), true))
                },
            ));
            */
        }
    };
    if let Err(err) = business::pip_install::ensure_pth_file_exist(&py_path, &install_dir) {
        tracing::error!("{err}");
    }

    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m").arg("pip").arg("install");
    if business::kubernetes::is_k8s() {
        cmd.arg("-U").arg("--target").arg(install_dir);
    }
    cmd.arg(format!("{}=={}", package_name, version));
    tracing::info!("cmd = {cmd:?}");
    let (tx, rx) = tokio::sync::mpsc::channel(2);
    tokio::spawn(async move {
        match cmd.output().await {
            Ok(output) => {
                if output.status.success() {
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
