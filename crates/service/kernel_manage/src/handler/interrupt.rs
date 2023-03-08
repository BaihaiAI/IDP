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

pub async fn interrupt(
    State(ctx): State<AppContext>,
    Query(req): Query<InodeReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let inode = req.inode;
    let kernel_opt = ctx.get_kernel_by_inode(inode).await?;
    let kernel = match kernel_opt {
        Some(kernel) => kernel,
        None => {
            // check runtime pod status first?
            // kernel not start but request cell_state on ipynb open
            if let Some(tx) = ctx.interrupt_creating_pod.read().await.get(&inode) {
                if let Err(err) = tx.send(()) {
                    tracing::error!("{err}");
                }
            }
            return Ok(Rsp::success(()).message("interrupt creating pod"));
        }
    };

    let (tx, rx) = tokio::sync::oneshot::channel();
    kernel
        .kernel_operate_tx
        .send(KernelOperate::GetState(tx))
        .await?;
    let state = rx.await?;
    if matches!(state, KernelState::Idle) {
        return Ok(Rsp::success(()));
    }
    if !matches!(state, KernelState::Running(_)) {
        return Err(Error::new(&format!(
            "only running kernel can interrupt, current state is {state:?}"
        )));
    }

    kernel
        .kernel_operate_tx
        .send(KernelOperate::Interrupt)
        .await?;
    Ok(Rsp::success(()))
}
