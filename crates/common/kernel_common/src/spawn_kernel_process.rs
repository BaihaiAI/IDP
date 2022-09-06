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

use err::ErrorTrace;

use crate::Header;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SpawnKernel {
    pub header: Header,
    pub resource: Resource, // pub conda_env_name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// unit GB
    pub memory: f64,
    /// core count of cpu, 0.25 means 25% of one core
    pub num_cpu: f64,
    /// device count of gpu
    pub num_gpu: f64,
    /// 1-100
    pub priority: u8,
}

// serde(default) not work with serde(flatten) https://github.com/serde-rs/serde/issues/1626
impl Default for Resource {
    fn default() -> Self {
        Self {
            memory: 1.0,
            num_cpu: 1.0,
            num_gpu: 0.0,
            priority: 50,
        }
    }
}

pub fn spawn_kernel_process(header: Header) -> Result<std::process::Child, ErrorTrace> {
    tracing::info!("--> spawn_kernel_process");
    let ipynb_abs_path = header.ipynb_abs_path();

    let working_directory = ipynb_abs_path.parent().unwrap().to_path_buf();
    #[cfg(unix)]
    let python_minor_version = get_python_minor_version("python3");
    #[cfg(windows)]
    let python_minor_version = get_python_minor_version("python");
    if python_minor_version < 8 {
        return Err(ErrorTrace::new("only support python version >= 3.8"));
    }

    // tokio::fs::File has no into_raw_fd
    // let log_fd = log_file.into_raw_fd();

    let cur_exe_path = std::env::current_exe().unwrap();
    let cur_exe_dir = cur_exe_path.parent().unwrap();
    let target_dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../target"));
    // let is_dev_env = target_dir.join("debug").exists();
    #[cfg(unix)]
    let kernel_name = format!("kernel_py3{python_minor_version}");
    #[cfg(windows)]
    let kernel_name = format!("kernel_py3{python_minor_version}.exe");
    let mut kernel_path = None;
    for path in [
        cur_exe_dir.join("bin"),
        cur_exe_dir.to_path_buf(),
        Path::new(".").to_path_buf(),
        target_dir.join("debug"),
        target_dir.join("release"),
    ] {
        if path.exists() {
            if path.join(&kernel_name).exists() {
                kernel_path = Some(path.join(&kernel_name).canonicalize().unwrap());
                break;
            }
            #[cfg(windows)]
            if path.join("idp_kernel.exe").exists() {
                kernel_path = Some(path.join("idp_kernel.exe").canonicalize().unwrap());
                break;
            }
            #[cfg(unix)]
            if path.join("idp_kernel").exists() {
                kernel_path = Some(path.join("idp_kernel").canonicalize().unwrap());
                break;
            }
        }
    }
    let idp_kernel_path = match kernel_path {
        Some(path) => path,
        None => {
            return Err(ErrorTrace::new(&format!(
                "kernel binary {kernel_name}/idp_kernel not found"
            )));
        }
    };
    // use tokio process to prevent process become defunct(zombie) when criu dump
    // use shell process invoke kernel prevent defunct after criu dump(ptrace then kill kernel)

    #[cfg(unix)]
    let python3_real_path = {
        extern "C" {
            // char *realpath(const char *restrict path, char *restrict resolved_path);
            fn realpath(path: *const i8, output: *mut i8) -> *mut i8;
        }

        let output = std::process::Command::new("which")
            .arg("python3")
            .output()
            .unwrap();
        assert!(output.status.success(), "which python3 err");
        let python3_real_path = String::from_utf8_lossy(&output.stdout);
        let python3_real_path = python3_real_path.trim_end();
        // let mut realpath_output = [0u8; libc::PATH_MAX];
        let mut realpath_output = [0u8; 4096];
        tracing::debug!("{python3_real_path}");
        let python3_real_path = format!("{python3_real_path}\0");
        unsafe {
            if realpath(
                python3_real_path.as_ptr().cast(),
                realpath_output.as_mut_ptr().cast(),
            )
            .is_null()
            {
                panic!("{}", std::io::Error::last_os_error());
            }
        }
        let ret = String::from_utf8(realpath_output.to_vec()).unwrap();
        ret.trim_end_matches('0').to_string()
    };
    #[cfg(unix)]
    let conda_env_name_root = business::path_tool::get_conda_env_name_root(
        header.team_id,
        business::path_tool::project_conda_env(header.team_id, header.project_id),
    );
    #[cfg(unix)]
    let ld_library_path = {
        let python_lib_path = std::path::Path::new(&python3_real_path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("lib");
        let python_lib_path = python_lib_path.to_str().unwrap();

        let gateway_exe_path = std::env::current_exe().unwrap();
        let exe_parent_dir = gateway_exe_path.parent().unwrap();
        let package_lib_path = exe_parent_dir.join("lib");
        let package_lib_path = package_lib_path.to_str().unwrap();

        let ld_library_path = format!(
            "{conda_env_name_root}/lib:{python_lib_path}:{package_lib_path}:{}",
            std::env::var("LD_LIBRARY_PATH").unwrap_or_default()
        );

        tracing::debug!(
            "which python3 = {python3_real_path}\nLD_LIBRARY_PATH = {ld_library_path:?}"
        );
        ld_library_path
    };
    #[cfg(target_os = "linux")]
    {
        let ldd_output = std::process::Command::new("ldd")
            .arg(&idp_kernel_path)
            .env("LD_LIBRARY_PATH", &ld_library_path)
            .output()?;
        if !ldd_output.status.success() {
            return Err(ErrorTrace::new("ldd exit code 1"));
        }
        let stdout = String::from_utf8_lossy(&ldd_output.stdout);
        if stdout.contains("not found") {
            return Err(ErrorTrace::new(&format!(
                "missing dylib, ldd output {stdout}"
            )));
        }
    }
    let mut command = std::process::Command::new(idp_kernel_path);
    command
        // bash fork+exec new process to start kernel, prevent defunct after ptrace by criu
        // .arg(format!("(sleep 0; {idp_kernel_path} $0) &"))
        // .stdout(unsafe { std::process::Stdio::from_raw_fd(log_fd) })
        .arg(serde_json::to_string(&header).unwrap())
        .arg("ray_id")
        .current_dir(working_directory);
    #[cfg(unix)]
    command
        .env(
            if cfg!(target_os = "linux") {
                "LD_LIBRARY_PATH"
            } else if cfg!(target_os = "macos") {
                "DYLD_LIBRARY_PATH"
            } else {
                unreachable!()
            },
            ld_library_path,
        )
        .env(
            "PATH",
            format!(
                "{conda_env_name_root}/bin:/usr/bin:/usr/sbin:{}",
                std::env::var("PATH").unwrap_or_default()
            ),
        );
    command
        .env("MPLBACKEND", "module://baihai_matplotlib_backend")
        .env("RUST_BACKTRACE", "1");
    #[cfg(target_os = "macos")]
    if python3_real_path.contains("conda") {
        command.env(
            "PYTHONHOME",
            std::path::Path::new(&python3_real_path)
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        );
    }

    tracing::info!("{command:?}");
    let child = command.spawn()?;
    tracing::debug!("<-- spawn_kernel_process pid = {}", child.id());

    Ok(child)
}

fn submitter_port() -> u16 {
    if let Ok(val) = std::env::var("SUBMITTER_PORT") {
        val.parse().unwrap()
    } else {
        9240
    }
}

#[cfg(feature = "tcp")]
pub async fn req_submitter_spawn_kernel(arg: SpawnKernel) -> Result<(), ErrorTrace> {
    tracing::info!("--> spawn_kernel_process_tcp");
    if !business::kubernetes::is_k8s() {
        spawn_kernel_process(arg.header)?;
        return Ok(());
    }
    // let hostname = kernel_common::cluster_header_hostname(header.team_id);
    let url = format!("http://127.0.0.1:{}/start_kernel", submitter_port());
    let resp = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(15))
        .build()?
        .post(&url)
        .json(&arg)
        .send()
        .await?;
    if resp.status().is_success() {
        // can't return pid 0, if use pid 0 then shutdown kernel would shutdown kernel_manage process group
        Ok(())
    } else {
        let status = resp.status();
        let resp = resp.text().await?;
        tracing::warn!("submitter resp = {resp}, status = {status}");
        let json = serde_json::from_str::<serde_json::Value>(&resp)?;
        let msg = json
            .get("message")
            .map(|x| x.as_str())
            .unwrap_or_default()
            .unwrap_or_default();
        Err(ErrorTrace::new(msg))
    }
}

fn get_python_minor_version(python_path: &str) -> u8 {
    let mut cmd = std::process::Command::new(python_path);
    cmd.arg("--version");
    let output = cmd.output().unwrap_or_else(|_| panic!("{cmd:?}"));
    if !output.status.success() {
        panic!("{cmd:?} {output:?}");
    }
    let stdout = output.stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    let minor_version = stdout
        .trim_start_matches("Python 3.")
        .split_once('.')
        .expect(&stdout)
        .0;
    minor_version.parse().unwrap()
}

#[cfg(test)]
#[cfg(unix)]
fn get_python_minor_version_by_sys(python_path: &str) -> u8 {
    let stdout = std::process::Command::new(python_path)
        .arg("-c")
        .arg("print(__import__('sys').version_info.minor,end='')")
        .output()
        .unwrap()
        .stdout;
    let stdout = String::from_utf8_lossy(&stdout);
    stdout.parse().unwrap()
}

// on SSD debug build:
// [crates/service/kernel_manage/src/kernel_entry/get_python_minor_version.rs:31] start.elapsed() = 2.741944ms
// [crates/service/kernel_manage/src/kernel_entry/get_python_minor_version.rs:34] start.elapsed() = 23.407171ms
// on NFS, python -c maybe cost 500ms - 2000ms\
#[test]
#[cfg(unix)]
fn test_get_python_version() {
    let start = std::time::Instant::now();
    #[cfg(unix)]
    get_python_minor_version("python3");
    dbg!(start.elapsed());
    let start = std::time::Instant::now();
    get_python_minor_version_by_sys("python3");
    dbg!(start.elapsed());
}
