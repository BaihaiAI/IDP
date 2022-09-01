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

use tracing::debug;

use super::prelude::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Req {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    inode: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    project_id: ProjectId,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    team_id: TeamId,
    path: String,
    // serde(default) not work with serde(flatten) https://github.com/serde-rs/serde/issues/1626
    resource: Option<kernel_common::spawn_kernel_process::Resource>,
    restart: bool,
}

#[derive(serde::Serialize, Debug)]
pub struct Rsp {
    kernel: String,
    inode: String,
}

pub async fn post_shutdown_or_restart(
    ctx: AppContext,
    req: Request<Body>,
) -> Result<Resp<Rsp>, Error> {
    let req = de_hyper_body::<Req>(req).await?;
    shutdown_or_restart(ctx, req).await
}

pub async fn get_shutdown_or_restart(
    ctx: AppContext,
    req: Request<Body>,
) -> Result<Resp<Rsp>, Error> {
    let req = serde_urlencoded::from_str::<Req>(req.uri().query().unwrap_or_default())?;
    shutdown_or_restart(ctx, req).await
}

// deprecated
pub async fn shutdown_or_restart(ctx: AppContext, req: Req) -> Result<Resp<Rsp>, Error> {
    let team_id = req.team_id;
    let header = kernel_common::Header {
        path: req.path.clone(),
        project_id: req.project_id,
        team_id,
        ..Default::default()
    };
    let inode = req.inode;
    let kernel_opt = ctx.get_kernel_by_inode(inode).await?;
    let resource = match kernel_opt {
        Some(kernel) => {
            kernel
                .kernel_operate_tx
                .send(KernelOperate::Shutdown {
                    is_core_dump: false,
                })
                .await?;
            kernel.resource.clone()
        }
        None => {
            if !req.restart {
                return Err(Error::new("inode not found, maybe kernel has shutdown"));
            }
            req.resource.unwrap_or_default()
        }
    };

    if !req.restart {
        return Ok(Resp::success(Rsp {
            kernel: "".to_string(),
            inode: inode.to_string(),
        }));
    }

    // only restart would clean session/vars path
    let session_file_path =
        business::path_tool::session_file_path(header.team_id, header.project_id, &header.path);
    let vars_file_path =
        business::path_tool::vars_file_path(header.team_id, header.project_id, &header.path);
    _ = std::fs::remove_file(session_file_path);
    _ = std::fs::remove_file(vars_file_path);

    let kernel = KernelEntry::new(header, resource, &ctx).await?;
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::Insert(Box::new(kernel)))
        .await?;

    Ok(Resp::success(Rsp {
        kernel: "".to_string(),
        inode: inode.to_string(),
    }))
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ShutdownAllReq {
    project_id: ProjectId,
    #[serde(default)]
    path: String,
    state: Option<String>,
}

// - switch env shutdown kernel, path is ""
// - move/delete folder shutdown kernel, path is folder path
pub async fn shutdown_all(ctx: AppContext, req: Request<Body>) -> Result<Resp<()>, Error> {
    let req = serde_urlencoded::from_str::<ShutdownAllReq>(req.uri().query().unwrap_or_default())?;
    // serde_urlencoded lib contains url decode
    // req.path = urlencoding::decode(&req.path)?.to_string();
    tracing::info!("shutdown_all {req:#?}");
    let is_shutdown_all_idle_kernel = match req.state {
        Some(state) => {
            if state != "idle" {
                return Err(Error::new("only support shutdown_all idle kernel"));
            }
            true
        }
        None => false,
    };
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::GetAll(tx))
        .await?;
    let kernel_list = rx.await?;
    for kernel in kernel_list {
        // let inode = kernel.inode;
        if kernel.header.project_id != req.project_id {
            continue;
        }
        if !kernel.header.path.starts_with(&req.path) {
            continue;
        }
        if is_shutdown_all_idle_kernel {
            let (tx, rx) = tokio::sync::oneshot::channel();
            kernel
                .kernel_operate_tx
                .send(KernelOperate::GetState(tx))
                .await?;
            let kernel_state = rx.await.unwrap();
            if !kernel_state.is_idle() {
                continue;
            }
        }
        kernel
            .kernel_operate_tx
            .send(KernelOperate::Shutdown {
                is_core_dump: false,
            })
            .await?;
    }
    debug!("<-- shutdown_all");
    Ok(Resp::success(()).message("success"))
}

#[derive(serde::Deserialize)]
struct CoreDumpReportReq {
    ip: std::net::Ipv4Addr,
    pid: u32,
}

pub async fn core_dump_report(ctx: AppContext, req: Request<Body>) -> Result<Resp<()>, Error> {
    let req =
        serde_urlencoded::from_str::<CoreDumpReportReq>(req.uri().query().unwrap_or_default())?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::GetAll(tx))
        .await?;
    let kernel_list = rx.await?;
    for kernel in kernel_list {
        if kernel.kernel_info.ip == req.ip && kernel.kernel_info.pid == req.pid {
            tracing::info!("shutdown core dump kernel {} {}", req.ip, req.pid);
            let (tx, rx) = tokio::sync::oneshot::channel();
            kernel
                .kernel_operate_tx
                .send(KernelOperate::GetState(tx))
                .await?;
            let state = rx.await?;
            if matches!(state, State::Paused(_)) {
                continue;
            }
            kernel
                .kernel_operate_tx
                .send(KernelOperate::Shutdown { is_core_dump: true })
                .await?;
            return Ok(Resp::success(()).message("success"));
        }
    }
    Err(ErrorTrace::new("core dump kernel not found"))
}
