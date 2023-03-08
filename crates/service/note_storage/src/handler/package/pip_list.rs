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

use crate::api_model::TeamIdProjectIdQueryString;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub package_name: String,
    pub version: String,
}

pub async fn pip_list(
    Query(TeamIdProjectIdQueryString {
        project_id,
        team_id,
    }): Query<TeamIdProjectIdQueryString>,
) -> Result<Rsp<Vec<PackageInfo>>, err::ErrorTrace> {
    let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
    let py_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);
    let package_list = pip_list_(py_path).await?;

    Ok(Rsp::success(package_list))
}

#[tracing::instrument]
pub async fn pip_list_(py_path: String) -> std::io::Result<Vec<PackageInfo>> {
    tracing::debug!("--> pip_list");
    let py_path = if business::kubernetes::is_k8s() {
        py_path
    } else {
        #[cfg(unix)]
        let a = "python3".to_string();
        #[cfg(windows)]
        let a = "python".to_string();
        a
    };
    let start = std::time::Instant::now();
    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m")
        .arg("pip")
        .arg("list")
        .arg("--format")
        .arg("freeze");
    tracing::debug!("cmd = {cmd:?}");
    let output = cmd.output().await?;
    tracing::debug!(
        "--- pip_list after pip list command, time cost = {:?}",
        start.elapsed()
    );

    if !output.status.success() {
        let stderr = unsafe { String::from_utf8_unchecked(output.stderr) };
        tracing::error!("{stderr}");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("pip list error: {}", stderr),
        ));
    }

    let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };

    let mut pip_list = vec![];

    for line_str in stdout.lines() {
        let (package_name, version) = match line_str.split_once("==") {
            // if split_once failed, it means that line_str is not a package line,break;
            // e.g. could not fetch URL https://pypi.douban.org
            None => {
                break;
            }
            Some((package_name, version)) => (package_name, version),
        };
        pip_list.push(PackageInfo {
            package_name: package_name.to_string(),
            version: version.to_string(),
        });
    }
    tracing::debug!("<-- pip_list, timing = {:?}", start.elapsed());
    Ok(pip_list)
}
