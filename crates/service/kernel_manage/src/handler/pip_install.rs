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

use business::path_tool::get_conda_env_python_path;

use super::prelude::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PipReq {
    package_name: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    project_id: u64,
    version: String,
}

async fn parse_req(req: Request<Body>) -> Result<(String, PipReq), Error> {
    let team_id = team_id_from_cookie(&req)?;
    let req = de_hyper_body::<PipReq>(req).await?;

    let conda_env_name = business::path_tool::project_conda_env(team_id, req.project_id);
    let py_path = get_conda_env_python_path(team_id, conda_env_name);
    Ok((py_path, req))
}

pub async fn pip_install(req: Request<Body>) -> Result<Resp<()>, Error> {
    let (py_path, req) = parse_req(req).await?;
    let package_name = req.package_name;
    let version = req.version;

    real_install(py_path, package_name, version).await
}

pub async fn real_install(
    py_path: String,
    package_name: String,
    version: String,
) -> Result<Resp<()>, Error> {
    // business::team_id_tool::create_team_linux_user_if_not_exist(team_id);
    // let linux_username = business::team_id_tool::team_id_to_user_name(team_id);

    // let install_dir = business::pip_install::ensure_python2user_install_dir_exist(&py_path)
    //     .map_or(Err(Error::new("make dir failed")), Ok)?;
    // business::pip_install::ensure_pth_file_exist(&py_path, &install_dir)
    //     .map_or(Err(Error::new("check pth file failed")), Ok)?;

    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m").arg("pip").arg("install");
    // if business::kubernetes::is_k8s() {
    //     cmd.arg("-U").arg("-t").arg(install_dir);
    // }
    cmd.arg(format!("{}=={}", package_name, version));
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
    Ok(Resp::success(()))
}

pub async fn pip_uninstall(req: Request<Body>) -> Result<Resp<()>, Error> {
    // let team_id = team_id_from_cookie(&req)?;
    let (py_path, req) = parse_req(req).await?;
    // business::team_id_tool::create_team_linux_user_if_not_exist(team_id);
    // let linux_username = business::team_id_tool::team_id_to_user_name(team_id);

    // let mut cmd = tokio::process::Command::new("/usr/sbin/runuser");
    let mut cmd = tokio::process::Command::new(py_path);
    cmd.arg("-m")
        .arg("pip")
        .arg("uninstall")
        .arg("-y")
        .arg(req.package_name);
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
    Ok(Resp::success(()))
}
