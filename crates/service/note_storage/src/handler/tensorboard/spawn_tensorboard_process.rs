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

use err::ErrorTrace;
use tracing::debug;
use tracing::info;

pub async fn spawn_tensorboard_process(
    log_dir: &std::path::PathBuf,
    port: u16,
) -> Result<tokio::process::Child, ErrorTrace> {
    const RETRY_TIMES: usize = 25;
    const RETRY_INTERVAL_MS: u64 = 200;
    let mut cmd = tokio::process::Command::new("tensorboard");
    cmd.arg("--bind_all")
        .arg("--port")
        .arg(port.to_string())
        .arg("--logdir")
        .arg(&log_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true);
    info!("prepare to start tensorboard cmd {cmd:?}");
    let child = cmd.spawn()?;
    let pid = child.id().unwrap_or_default();
    for retry in 1..=RETRY_TIMES {
        match tokio::time::timeout(
            std::time::Duration::from_millis(5000),
            tokio::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)),
        )
        .await
        {
            Ok(res) => {
                if res.is_ok() {
                    break;
                }
            }
            Err(err) => {
                tracing::error!("{err}");
                // return Err(IdpGlobalError::UnDefinedError(err));
            }
        }
        if retry == RETRY_TIMES {
            return Err(ErrorTrace::new("tensorboard tcp bind fail"));
        }
        tokio::time::sleep(std::time::Duration::from_millis(RETRY_INTERVAL_MS)).await;
        debug!("tensorboard bind not success, retry={retry}, pid={pid}");
    }
    #[cfg(unix)]
    if unsafe { libc::kill(pid as _, 0) } == -1 {
        return Err(ErrorTrace::new("tensorboard start failed, pid not exist"));
    }
    #[cfg(windows)]
    std::process::Command::new("taskkill")
        .arg("/F")
        .arg("/PID")
        .arg(pid.to_string())
        .spawn()?;

    info!("tensorboard command spawn success");
    Ok(child)
}
