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
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct Req {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub inode: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: ProjectId,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: TeamId,
    pub path: String,
    // serde(default) not work with serde(flatten) https://github.com/serde-rs/serde/issues/1626
    pub resource: Option<kernel_common::spawn_kernel_process::Resource>,
    pub restart: bool,
}

#[derive(serde::Serialize, Debug)]
pub struct ShutdownRsp {
    kernel: String,
    inode: String,
}

// #[axum_macros::debug_handler]
pub async fn post_shutdown_or_restart(
    State(ctx): State<AppContext>,
    Json(req): Json<Req>,
) -> Result<Rsp<ShutdownRsp>, Error> {
    // let req = de_hyper_body::<Req>(req).await?;
    shutdown_or_restart(ctx, req).await
}

#[cfg(not)]
pub async fn get_shutdown_or_restart(
    ctx: AppContext,
    req: Request<Body>,
) -> Result<Rsp<ShutdownRsp>, ErrorTrace> {
    let req = serde_urlencoded::from_str::<Req>(req.uri().query().unwrap_or_default())?;
    shutdown_or_restart(ctx, req).await
}

pub async fn shutdown_or_restart(ctx: AppContext, req: Req) -> Result<Rsp<ShutdownRsp>, Error> {
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
                    core_dumped_reason: None,
                })
                .await?;
            kernel.resource.clone()
        }
        None => {
            if !req.restart {
                return Err(ErrorTrace::new("already shutdown").code(ErrorTrace::CODE_WARNING));
            }
            req.resource.unwrap_or_default()
        }
    };

    if !req.restart {
        return Ok(Rsp::success(ShutdownRsp {
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

    let (tx, _rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::Insert {
            header: Box::new(header),
            resource,
            ctx: ctx.clone(),
            tx,
        })
        .await?;

    Ok(Rsp::success(ShutdownRsp {
        kernel: "".to_string(),
        inode: inode.to_string(),
    }))
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownAllReq {
    project_id: ProjectId,
    #[serde(default)]
    path: String,
    state: Option<String>,
}

// - switch env shutdown kernel, path is ""
// - move/delete folder shutdown kernel, path is folder path
pub async fn shutdown_all(
    State(ctx): State<AppContext>,
    Query(req): Query<ShutdownAllReq>,
) -> Result<Rsp<()>, Error> {
    tracing::debug!("shutdown_all {req:#?}");
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
                core_dumped_reason: None,
            })
            .await?;
    }
    debug!("<-- shutdown_all");
    Ok(Rsp::success(()).message("success"))
}

#[derive(serde::Deserialize)]
pub struct CoreDumpReportReq {
    // ip: std::net::Ipv4Addr,
    // pid: u32,
    pod_id: String,
    reason: String,
}

pub async fn core_dumped_report(
    State(ctx): State<AppContext>,
    Query(req): Query<CoreDumpReportReq>,
) -> Result<Rsp<()>, Error> {
    // let req = serde_urlencoded::from_str::<CoreDumpReportReq>(req.uri().query().unwrap_or_default())?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::GetAll(tx))
        .await?;
    let kernel_list = rx.await?;
    for kernel in kernel_list {
        if kernel.kernel_info.pod_id == req.pod_id {
            tracing::info!("shutdown core dump kernel {}", req.pod_id);
            let (tx, rx) = tokio::sync::oneshot::channel();
            kernel
                .kernel_operate_tx
                .send(KernelOperate::GetState(tx))
                .await?;
            let state = rx.await?;
            if matches!(state, KernelState::Paused(_)) {
                continue;
            }
            kernel
                .kernel_operate_tx
                .send(KernelOperate::Shutdown {
                    core_dumped_reason: Some(req.reason),
                })
                .await?;
            return Ok(Rsp::success(()).message("success"));
        }
    }
    Err(ErrorTrace::new("core dump kernel not found"))
}
