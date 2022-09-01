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

use super::prelude::*;

pub(super) fn criu_dump_dir(ip: std::net::Ipv4Addr, pid: u32) -> String {
    format!("/store/run/criu_dump_dir_{ip}_pid_{pid}")
}

/**
use SIGSTOP/SIGCONT in standalone/non_k8s env?

criu dump tips:
1. can't run in vscode terminal
2. can't use unix domain socket
3. recommend disable tty, stdin/stdout to null
4. can't hold

```text
/root/criu: /lib/x86_64-linux-gnu/libnl-3.so.200: no version information available (required by /root/criu)
Warn  (compel/arch/x86/src/lib/infect.c:340): Will restore 1854621 with interrupted system call
Error (criu/sk-unix.c:871): unix: Can't dump half of stream unix connection.
Error (criu/cr-dump.c:1788): Dumping FAILED.
```
*/
pub async fn pause(ctx: AppContext, req: Request<Body>) -> Result<Resp<()>, Error> {
    let inode = inode_from_query_string(req)?;
    let kernel_opt = ctx.get_kernel_by_inode(inode).await?;
    let kernel = match kernel_opt {
        Some(kernel) => kernel,
        None => {
            // kernel not start but request cell_state on ipynb open
            return Err(Error::new("kernel not found"));
        }
    };

    let (tx, rx) = tokio::sync::oneshot::channel();
    kernel
        .kernel_operate_tx
        .send(KernelOperate::GetState(tx))
        .await?;
    let state = rx.await.unwrap();
    if !matches!(state, State::Running(_) | State::Idle) {
        return Err(Error::new("only running or idle kernel can pause"));
    }

    let ip = kernel.kernel_info.ip;
    let pid = kernel.kernel_info.pid;
    let dir = criu_dump_dir(ip, pid);
    _ = std::fs::create_dir_all(&dir);

    let resp = reqwest::get(format!("http://{ip}:9241/cr/pause?pid={pid}")).await?;
    if !resp.status().is_success() {
        return Err(Error::new("pause failed").code(500));
    }

    // tracing::info!("prepare to criu dump pid {} to dir {}", pid, dir);
    // let mut cmd = tokio::process::Command::new("criu");
    // cmd.arg("dump")
    //     .arg("--shell-job")
    //     .arg("--tcp-established")
    //     .arg("-t")
    //     .arg(pid.to_string())
    //     .current_dir(dir);
    // tracing::info!("cmd = {cmd:?}");
    // let output = cmd.output().await?;
    // if !output.status.success() {
    //     tracing::error!(
    //         "criu dump fail stdout:\n{}",
    //         String::from_utf8_lossy(&output.stdout)
    //     );
    //     tracing::error!(
    //         "criu dump fail stderr:\n{}",
    //         String::from_utf8_lossy(&output.stderr)
    //     );
    //     return Err(Error::new("pause failed").code(500));
    // }

    kernel.kernel_operate_tx.send(KernelOperate::Pause).await?;

    tracing::info!("criu dump success");

    let req = serde_json::json!({
        "teamId": kernel.kernel_info.header.team_id.to_string(),
        "rayTaskId": kernel.kernel_info.ray_id.clone(),
        "status": "Paused".to_string(),
        "cpuUsed": 0,
        "gpuUsed": 0,
        "memUsed": 0,
    });

    let client = reqwest::ClientBuilder::new().build().unwrap();

    tracing::info!(
        "post modify-task-monitor rayTaskId={:?}",
        kernel.kernel_info.ray_id.clone()
    );

    match client
        .post("http://idp-admin-rs-svc:9092/api/v1/admin-rs/dashboard/modify-task-monitor")
        .json(&req)
        .send()
        .await
    {
        Ok(resp) => {
            tracing::info!("post modify-task-monitor resp={:?}", resp);
        }
        Err(err) => {
            tracing::error!("post modify-task-monitor error={:?}", err);
        }
    }

    Ok(Resp::success(()))
}
