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

pub async fn resume(ctx: AppContext, req: Request<Body>) -> Result<Resp<()>, Error> {
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
    if !matches!(state, State::Paused { .. }) {
        return Err(Error::new("only paused kernel can resume"));
    }

    let ip = kernel.kernel_info.ip;
    let pid = kernel.kernel_info.pid;
    let resp = reqwest::get(format!("http://{ip}:9241/cr/resume?pid={pid}")).await?;
    if !resp.status().is_success() {
        return Err(Error::new("resume failed").code(500));
    }

    // let dir = super::pause::criu_dump_dir(ip, pid);
    // tracing::info!("prepare to criu restore at dir {dir}");
    // let mut cmd = tokio::process::Command::new("criu");
    // cmd.arg("restore")
    //     .arg("--shell-job")
    //     .arg("--tcp-established")
    //     .arg("--restore-detached")
    //     .current_dir(dir);
    // tracing::info!("cmd = {cmd:?}");
    // let output = cmd.output().await?;
    // if !output.status.success() {
    //     tracing::error!(
    //         "criu restore fail stdout:\n{}",
    //         String::from_utf8_lossy(&output.stdout)
    //     );
    //     tracing::error!(
    //         "criu restore fail stderr:\n{}",
    //         String::from_utf8_lossy(&output.stderr)
    //     );
    //     return Err(Error::new("resume failed").code(500));
    // }

    kernel.kernel_operate_tx.send(KernelOperate::Resume).await?;

    tracing::info!("criu restore success");

    let client = reqwest::ClientBuilder::new().build().unwrap();
    let req = serde_json::json!({
        "teamId": kernel.kernel_info.header.team_id.to_string(),
        "rayTaskId": kernel.kernel_info.ray_id.clone(),
        "status": "Running".to_string(),
    });

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
