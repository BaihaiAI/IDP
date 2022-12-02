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
    pub pod_id: Option<String>,
}

// serde(default) not work with serde(flatten) https://github.com/serde-rs/serde/issues/1626
impl Default for Resource {
    fn default() -> Self {
        Self {
            memory: 0.9,
            num_cpu: 0.9,
            num_gpu: 0.0,
            priority: 3,
            pod_id: None,
        }
    }
}

fn spawn_kernel_process(header: Header) -> Result<(), ErrorTrace> {
    tracing::info!("--> spawn_kernel_process");
    let ipynb_abs_path = header.ipynb_abs_path();

    let working_directory = ipynb_abs_path.parent().unwrap().to_path_buf();
    #[cfg(unix)]
    let python_minor_version = get_python_minor_version("python3");
    #[cfg(windows)]
    let python_minor_version = get_python_minor_version("python");
    if python_minor_version < 7 {
        return Err(ErrorTrace::new("only support python version >= 3.7"));
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
                "kernel binary {kernel_name} or idp_kernel not found"
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
            .expect("which python3");
        assert!(output.status.success(), "which python3 err");
        let which_python_output = String::from_utf8_lossy(&output.stdout);
        let python3_real_path = which_python_output.trim_end();
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
    tracing::info!("python3_real_path = {python3_real_path}");
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

        let default_ld_lib_path = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
        let ld_library_path = if default_ld_lib_path.is_empty() {
            format!("{conda_env_name_root}/lib:{python_lib_path}:{package_lib_path}",)
        } else {
            format!(
                "{conda_env_name_root}/lib:{python_lib_path}:{package_lib_path}:{}",
                default_ld_lib_path
            )
        };

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
    let pod_id = header.inode();
    let mut command = std::process::Command::new(idp_kernel_path);
    command
        // bash fork+exec new process to start kernel, prevent defunct after ptrace by criu
        // .arg(format!("(sleep 0; {idp_kernel_path} $0) &"))
        // .stdout(unsafe { std::process::Stdio::from_raw_fd(log_fd) })
        .arg(base64::encode(serde_json::to_string(&header).unwrap()))
        .arg(pod_id.to_string())
        .current_dir(working_directory);
    let mut env = std::collections::HashMap::new();
    env.insert(
        "MPLBACKEND",
        "module://baihai_matplotlib_backend".to_string(),
    );
    #[cfg(unix)]
    env.insert(
        if cfg!(target_os = "linux") {
            "LD_LIBRARY_PATH"
        } else if cfg!(target_os = "macos") {
            "DYLD_LIBRARY_PATH"
        } else {
            unreachable!()
        },
        ld_library_path.to_string(),
    );
    #[cfg(unix)]
    env.insert(
        "PATH",
        format!(
            "{conda_env_name_root}/bin:/usr/bin:/usr/sbin:{}",
            std::env::var("PATH").unwrap_or_default()
        ),
    );
    #[cfg(unix)]
    if !python3_real_path.starts_with("/usr/bin") {
        env.insert(
            "PYTHONHOME",
            std::path::Path::new(&python3_real_path)
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    tracing::info!("{command:#?}\nenv={env:#?}");
    for (k, v) in env {
        command.env(k, v);
    }
    let mut child = command.spawn()?;
    std::thread::Builder::new().name("wait_kernel".to_string()).spawn(move || {
        /*
        let Ok(exit_status) = child.wait() else {
            panic!("");
        };
        */
        let exit_status = child.wait().expect("wait kernel child process");
        #[cfg(unix)]
        if let Some(signal) = std::os::unix::process::ExitStatusExt::signal(&exit_status) {
            let err_msg = format!("kernel was kill by signal {signal}");
            #[cfg(not)]
            if signal == 9 {
                // /sys/fs/cgroup/user.slice/user-1000.slice/memory.current
                dbg!(std::fs::read_to_string("/sys/fs/cgroup/memory/memory.limit_in_bytes").unwrap());
            }
            tracing::error!("{err_msg}");
            let rsp = reqwest::blocking::Client::builder().build().unwrap().post(format!("http://127.0.0.1:9007/api/v1/execute/kernel/core_dumped_report?pod_id={pod_id}&reason={err_msg}")).send().unwrap();
            dbg!(rsp.status());
            return;
        }
        #[cfg(unix)]
        if std::os::unix::process::ExitStatusExt::core_dumped(&exit_status) {
            panic!("kernel core dumped! please check log in coredumpctl");
        }
        if !exit_status.success() {
            panic!("kernel exit code {:?}", exit_status.code());
        }
    }).unwrap();

    Ok(())
}

pub async fn req_submitter_spawn_kernel(arg: SpawnKernel) -> Result<(), ErrorTrace> {
    tracing::info!("--> spawn_kernel_process_tcp");
    if !business::kubernetes::is_k8s() {
        spawn_kernel_process(arg.header)?;
        return Ok(());
    }
    // let hostname = kernel_common::cluster_header_hostname(header.team_id);
    let url = format!("http://127.0.0.1:{}/start_kernel", 9240);
    let timeout_secs = 75;
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .build()?;
    let resp = match client.post(&url).json(&arg).send().await {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                return Err(ErrorTrace::new("request to submitter timeout"));
            }
            return Err(ErrorTrace::new(&err.to_string()));
        }
    };

    let http_status_code = resp.status();
    if http_status_code.is_success() {
        // can't return pid 0, if use pid 0 then shutdown kernel would shutdown kernel_manage process group
        return Ok(());
    }

    let status = resp.status();
    tracing::warn!("submitter resp status = {status}");
    if status.as_u16() != reqwest::StatusCode::TOO_MANY_REQUESTS {
        // if server response please retry
        let err_msg = if status.as_u16() == 500 {
            "submitter raise Exception".to_string()
        } else {
            resp.text().await?
        };
        return Err(ErrorTrace::new(&format!("kernel start fail {err_msg}")));
    }

    Err(ErrorTrace::new("kernel start max retry exceed"))
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

#[test]
#[ignore]
fn test_submitter_client_start_kernel() {
    let host = std::env::var("HOST").unwrap();
    for region in ["ga", "a", "b"] {
        let url = format!("http://{host}/{region}/api/v1/cluster");
        let body = SpawnKernel {
            header: Header {
                ..Default::default()
            },
            resource: Resource::default(),
        };
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();
        let rsp = client
            .get(url)
            .header("Cookie", format!("region={region}"))
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .unwrap()
            .text()
            .unwrap();
        dbg!(rsp);
    }

    dbg!(host);
}
