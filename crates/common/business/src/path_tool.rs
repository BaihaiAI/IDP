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

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::business_term::ProjectFolder;
use crate::business_term::ProjectId;
use crate::business_term::TeamFolder;
use crate::business_term::TeamId;
use crate::business_term::UserId;

static STORE_PARENT_DIR: Lazy<PathBuf> = Lazy::new(|| {
    #[cfg(unix)]
    let home_dir = std::env::var("HOME").unwrap();
    #[cfg(windows)]
    let home_dir = std::env::var("HOMEPATH").unwrap();
    let dir = std::path::Path::new(&home_dir).join(".idp");
    if !dir.exists() {
        tracing::warn!("{dir:?} not exists, creating");
        std::fs::create_dir(&dir).unwrap();
    }

    let custom_python_packages = dir.join("custom_python_packages");
    if !custom_python_packages.exists() {
        std::fs::create_dir(&custom_python_packages).unwrap();
    }
    std::fs::write(
        custom_python_packages.join("baihai_matplotlib_backend.py"),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../service/idp_kernel/baihai_matplotlib_backend.py"
        )),
    )
    .unwrap();

    if crate::kubernetes::is_k8s() {
        // k8s deploy
        let path = Path::new("/");
        return path.to_path_buf();
    }

    if !dir.join("store").exists() {
        let gateway_exe_path = std::env::current_exe().unwrap();
        let exe_parent_dir = gateway_exe_path.parent().unwrap();
        let store_template_path = if exe_parent_dir.join("store").exists() {
            exe_parent_dir.join("store")
        } else {
            Path::new(&concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../docker_build/store"
            ))
            .canonicalize()
            .unwrap()
        };
        #[cfg(unix)]
        {
            let mut cmd = std::process::Command::new("cp");
            cmd.arg("-r")
                .arg(store_template_path)
                .arg(dir.join("store"));
            tracing::info!("cmd = {cmd:?}");
            cmd.spawn().unwrap().wait().unwrap();
        }
        #[cfg(windows)]
        {
            let options = fs_extra::dir::CopyOptions {
                copy_inside: true,
                ..Default::default()
            };
            fs_extra::dir::copy(store_template_path, dir.join("store"), &options).unwrap();
        }
    }
    assert!(dir.join("store").exists());

    dir
});

pub fn store_parent_dir() -> PathBuf {
    STORE_PARENT_DIR.to_owned()
}

#[inline]
fn team_id_root(team_id: TeamId) -> PathBuf {
    // format!("{dir}/store/{team_id}")
    STORE_PARENT_DIR.join("store").join(team_id.to_string())
}

#[inline]
pub fn conda_root(team_id: TeamId) -> String {
    // team_id_root(team_id).join(TeamFolder::CONDA.inner())
    format!(
        "{}/{}",
        team_id_root(team_id).to_str().unwrap(),
        TeamFolder::CONDA.inner()
    )
}

#[inline]
pub fn get_conda_env_name_root(team_id: TeamId, conda_env_name: String) -> String {
    format!("{}/envs/{}", conda_root(team_id), conda_env_name)
}

pub fn get_nbconvert_by_team_id(_team_id: String) -> String {
    // format!(
    //     "/store/{}/miniconda3/envs/python39/bin/jupyter-nbconvert",
    //     team_id
    // )
    "/home/ray/anaconda3/bin/jupyter-nbconvert".to_string()
}
#[cfg(unix)]
pub fn get_conda_env_python_path(team_id: TeamId, conda_env_name: String) -> String {
    format!(
        "{}/bin/python",
        get_conda_env_name_root(team_id, conda_env_name)
    )
}
#[cfg(windows)]
pub fn get_conda_env_python_path(_team_id: TeamId, _conda_env_name: String) -> String {
    "python".to_string()
}

#[inline]
pub fn get_conda_path(team_id: TeamId) -> String {
    format!("/store/{team_id}/miniconda3/bin/conda", team_id = team_id)
}

#[inline]
pub fn get_model_file_path(team_id: i64, project_id: i32) -> String {
    format!(
        "/store/model_files/{team_id}/projects/{project_id}/models/",
        team_id = team_id,
        project_id = project_id
    )
}

#[inline]
pub fn project_root(team_id: TeamId, project_id: ProjectId) -> String {
    format!(
        "{}/{}/{project_id}",
        team_id_root(team_id).to_str().unwrap(),
        TeamFolder::PROJECTS.inner()
    )
}

pub fn project_conda_env(team_id: TeamId, project_id: ProjectId) -> String {
    let project_env_path = format!("{}/miniconda3/conda.env", project_root(team_id, project_id));
    std::fs::read_to_string(project_env_path)
        .unwrap_or_else(|_| "python39".to_string())
        .trim_end()
        .to_string()
}

fn project_tmp(team_id: TeamId, project_id: ProjectId) -> String {
    format!(
        "{}/{}",
        project_root(team_id, project_id),
        ProjectFolder::TMP.inner()
    )
}

pub fn user_extensions_path(team_id: TeamId, user_id: UserId) -> String {
    format!(
        "{}/{}/{user_id}",
        team_id_root(team_id).to_str().unwrap(),
        TeamFolder::EXTENSIONS.inner()
    )
}

pub fn recommended_extensions() -> PathBuf {
    if std::path::Path::new("/var/run/secrets/kubernetes.io").exists() {
        std::path::Path::new("/home/ray/extension-store").to_path_buf()
    } else {
        STORE_PARENT_DIR.join("store").join("extension-store")
    }
}

pub fn vars_file_path(team_id: TeamId, project_id: ProjectId, path: &str) -> String {
    format!(
        "{}/{}.vars",
        project_tmp(team_id, project_id),
        replace_path(path)
    )
}

pub fn session_file_path(team_id: TeamId, project_id: ProjectId, path: &str) -> String {
    // debug_assert!(nb_rel_path.starts_with('/'));
    format!(
        "{}/{}.session",
        project_tmp(team_id, project_id),
        replace_path(path)
    )
}

#[tracing::instrument]
pub fn get_store_path(team_id: TeamId, project_id: ProjectId, path_type: ProjectFolder) -> PathBuf {
    // tracing::debug!("--> get_store_path");
    let project_root_ = project_root(team_id, project_id);
    let base_path = std::path::Path::new(&project_root_);
    base_path.join(path_type.inner())
}

pub fn get_store_full_path<P: AsRef<Path>>(
    team_id: TeamId,
    project_id: ProjectId,
    relative_path: P,
) -> PathBuf {
    match relative_path.as_ref().strip_prefix("/") {
        Ok(path_without_leading_slash) => {
            get_store_path(team_id, project_id, ProjectFolder::NOTEBOOKS)
                .join(path_without_leading_slash)
        }
        Err(_) => get_store_path(team_id, project_id, ProjectFolder::NOTEBOOKS).join(relative_path),
    }
}

pub fn get_full_tmp_path<P: AsRef<Path>>(
    team_id: u64,
    project_id: u64,
    relative_path: P,
) -> PathBuf {
    get_store_path(team_id, project_id, ProjectFolder::TMP).join(relative_path)
}

pub fn escape_path_as_string(path_str: String) -> String {
    path_str.replace('/', "___")
}

#[cfg(not)]
pub fn get_relative_path_from_notebooks_full_path<P: AsRef<Path>>(
    team_id: TeamId,
    project_id: ProjectId,
    path: P,
) -> Option<String> {
    let store_notebooks_dir_path = get_store_path(team_id, project_id, ProjectFolder::NOTEBOOKS);
    tracing::debug!("{:?}", store_notebooks_dir_path);
    let path = path.as_ref();
    tracing::debug!("{:?}", path);
    if path.starts_with(&store_notebooks_dir_path) {
        let path = path.to_str().unwrap().to_string();
        let prefix = store_notebooks_dir_path
            .into_os_string()
            .into_string()
            .unwrap();

        return Some(path.strip_prefix(&prefix).unwrap().to_string());
    }
    None
}

#[cfg(not)]
pub fn get_pipeline_instance_workdir_path(
    team_id: TeamId,
    project_id: ProjectId,
    job_id: JobId,
    instance_id: InstanceId,
) -> PathBuf {
    //1.1 get pipeline root path via team_id and project_id using business path_tool
    // /store/1519596781073219584/projects/104/job/80/400/770_1.py.out
    let mut job_path = get_store_path(team_id, project_id, ProjectFolder::JOB);
    //1.2 splice task directory path
    job_path.push(job_id.to_string());
    job_path.push(instance_id.to_string());
    job_path
}

pub fn get_pipeline_output_path(
    team_id: u64,
    project_id: u64,
    path: &str,
    job_id: u64,
    job_instance_id: u64,
    task_instance_id: u64,
    ipynb_flag: bool,
) -> Result<PathBuf, String> {
    let mut job_path = get_store_path(team_id, project_id, ProjectFolder::JOB);
    job_path.push(job_id.to_string());
    job_path.push(job_instance_id.to_string());
    let path = replace_path(path);
    let mut file_name = format!("{}{}", task_instance_id, path);
    if !ipynb_flag {
        file_name.push_str(".out");
    }
    job_path.push(file_name);
    tracing::info!("##job_path={:?}", job_path);
    let output_dir = job_path.parent().unwrap();
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|err| format!("create pipeline output_dir {output_dir:?} {err}"))?;
    }
    Ok(job_path)
}

#[cfg(test)]
#[cfg(not)]
pub fn first_match_path(root: String, regex: String, depth: usize) -> Option<String> {
    if let Ok(walker) = globwalk::GlobWalkerBuilder::from_patterns(root, &[regex])
        .max_depth(depth)
        .follow_links(true)
        .build()
    {
        let mut matches = walker.into_iter().filter_map(Result::ok);
        if let Some(entry) = matches.next() {
            return Some(entry.path().to_str().unwrap().to_owned());
        }
        None
    } else {
        None
    }
}
#[cfg(unix)]
fn replace_path(path: &str) -> String {
    path.replace('/', "___")
}
#[cfg(windows)]
fn replace_path(path: &str) -> String {
    path.replace('\\', "___")
}
