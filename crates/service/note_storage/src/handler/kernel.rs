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

use business::business_term::ProjectId;
use err::ErrorTrace;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownKernelByDirReq {
    pub project_id: u64,
    pub dir_path: String,
}

#[cfg(not)]
pub async fn shutdown_by_dir_path_api(
    Json(shutdown_kernel_by_dir_req): Json<ShutdownKernelByDirReq>,
) -> Result<Rsp<()>, IdpGlobalError> {
    shutdown_by_project_id_and_kernel_idpnb_starts_with_path(
        shutdown_kernel_by_dir_req.project_id,
        shutdown_kernel_by_dir_req.dir_path,
    )
    .await?;
    Ok(Rsp::success(()))
}

pub async fn shutdown_by_project_id_and_kernel_idpnb_starts_with_path(
    project_id: ProjectId,
    kernel_idpnb_starts_with_path: &str,
) -> Result<(), ErrorTrace> {
    let kernel_idpnb_starts_with_path = if kernel_idpnb_starts_with_path.starts_with('/') {
        kernel_idpnb_starts_with_path.to_string()
    } else {
        format!("/{}", kernel_idpnb_starts_with_path)
    };
    let dir_path = urlencoding::encode(&kernel_idpnb_starts_with_path);
    let url = format!(
        "http://127.0.0.1:{}/api/v1/execute/kernel/shutdown_all?projectId={project_id}&path={dir_path}",
        business::kernel_manage_port()
    );
    tracing::info!("--> shutdown_by_dir_path, url = {url}");
    let rsp = reqwest::get(url).await?;
    assert!(rsp.status().is_success());
    Ok(())
}

#[cfg(not)]
pub async fn close_kernels(
    redis_cache: &mut CacheService,
    project_id: ProjectId,
    dir_path: Option<String>,
) -> Result<(), IdpGlobalError> {
    let port = business::kernel_manage_port();
    let kernel_list = redis_cache.get_kernel_state_list(project_id).await?;
    if !kernel_list.is_empty() {
        //2.convert kernel_state_list, get all hostname ,and deduplicate
        let hostname_list = kernel_list
            .into_iter()
            .map(|kernel_state| kernel_state.hostname)
            .collect::<std::collections::HashSet<String>>();

        // hostname_list.sort_unstable();
        // hostname_list.dedup();
        //if call api failed,throw a error.
        let response_vec = if let Some(dir_path) = dir_path {
            futures::future::try_join_all(hostname_list.into_iter().map(|hostname| {
                reqwest::get(format!(
                    "http://{}:{port}/api/v1/execute/kernel/shutdown_all?projectId={}path={}",
                    hostname, project_id, dir_path
                ))
            }))
            .await?
        } else {
            futures::future::try_join_all(hostname_list.into_iter().map(|hostname| {
                reqwest::get(format!(
                    "http://{}:{port}/api/v1/execute/kernel/shutdown_all?projectId={}",
                    hostname, project_id
                ))
            }))
            .await?
        };
        debug!("{:?}", response_vec);
    }
    Ok(())
}
