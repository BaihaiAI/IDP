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

use std::path::Path;

use business::path_tool::get_conda_env_python_path;
use walkdir::WalkDir;

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

fn ensure_pth_file_exist(py_path: &String, install_dir: &String) -> Result<String, String> {
    let path = Path::new(py_path);
    let p1 = path
        .parent()
        .map_or(Err(format!("get {} parent failed", path.display())), |x| {
            Ok(x)
        })?;
    let p2 = p1.parent().map_or(
        Err(format!("get {} parent 2 failed", path.display())),
        |x| Ok(x),
    )?;
    let aim_dir = p2.join("lib");

    if aim_dir.is_dir() {
        for entry in WalkDir::new(aim_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.starts_with("python") {
                let aim = entry.path().join("site-packages/0_idp.pth");
                if aim.exists() {
                    return Ok(aim.to_string_lossy().to_string());
                } else {
                    let p1 = aim
                        .parent()
                        .map_or(Err(format!("get {} parent failed", aim.display())), |x| {
                            Ok(x)
                        })?;
                    if !p1.exists() {
                        tracing::error!(
                            "python site-packages dir {} not exist, exit",
                            p1.display()
                        );
                        return Err(format!(
                            "python site-package dir {} not exist",
                            p1.display()
                        ));
                    } else {
                        return write_idp_pth_file(aim, install_dir);
                    }
                }
            }
        }
        // traval all file, not found python
        tracing::error!("not found python lib/pythonxxx dir");
        return Err(format!("not found pyton lib/pythonxxxx dir"));
    } else {
        tracing::error!("not found python lib dir");
        return Err(format!("not found python lib dir"));
    }
}

fn write_idp_pth_file(aim: std::path::PathBuf, install_dir: &String) -> Result<String, String> {
    return std::fs::write(&aim, install_dir)
        .map_or(Err(format!("write to {} failed", aim.display())), |_| {
            Ok(format!("{}", aim.display()))
        });
}

fn ensure_python2user_install_dir_exist(py_path: &String) -> Result<String, String> {
    let path = Path::new(py_path);
    let p1 = path
        .parent()
        .map_or(Err(format!("get {} parent failed", path.display())), |x| {
            Ok(x)
        })?;
    let p2 = p1.parent().map_or(
        Err(format!("get {} parent 2 failed", path.display())),
        |x| Ok(x),
    )?;
    let aim = p2.join("pm_installed");
    std::fs::create_dir_all(&aim)
        .map_or(Err(format!("create dir {} failed", aim.display())), |_| {
            Ok("")
        })?;
    return aim
        .into_os_string()
        .into_string()
        .map_or(Err(format!("trans path XXX to string failed")), |ret| {
            Ok(ret)
        });
}

pub async fn pip_install(req: Request<Body>) -> Result<Resp<()>, Error> {
    let (py_path, req) = parse_req(req).await?;
    let package_name = req.package_name;
    let version = req.version;

    return real_install(py_path, package_name, version).await;
}

pub async fn real_install(
    py_path: String,
    package_name: String,
    version: String,
) -> Result<Resp<()>, Error> {
    // business::team_id_tool::create_team_linux_user_if_not_exist(team_id);
    // let linux_username = business::team_id_tool::team_id_to_user_name(team_id);

    let install_dir = ensure_python2user_install_dir_exist(&py_path)
        .map_or(Err(Error::new("make dir failed")), |ret| Ok(ret))?;
    ensure_pth_file_exist(&py_path, &install_dir)
        .map_or(Err(Error::new("check pth file failed")), |ret| Ok(ret))?;

    let mut cmd = tokio::process::Command::new(py_path);
    cmd
        // .arg("-u")
        // .arg(linux_username)
        // .arg("--")
        // .arg(py_path)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("-U")
        .arg("-t")
        .arg(install_dir)
        .arg(format!("{}=={}", package_name, version));
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
    cmd
        // .arg("-u")
        // .arg(linux_username)
        // .arg("--")
        // .arg(py_path)
        .arg("-m")
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

/*
#[test]
fn test_ensure_py_installed_dir() {
    let py_bin = String::from("/tmp/aa/bb/bin/python");
    ensure_python2user_install_dir_exist(&py_bin);
    assert!(Path::new("/tmp/aa/bb/pm_installed").exists())
}

#[tokio::test]
async fn test_install_bs4() {
    let py_bin = String::from("/home/miniconda3/envs/lz_ray/bin/python");
    let package = String::from("pandas");
    let version = String::from("1.4.0");
    let m = real_install(py_bin, package, version).await;
}
*/
