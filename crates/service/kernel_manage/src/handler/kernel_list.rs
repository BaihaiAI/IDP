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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct KernelListReq {
    project_id: ProjectId,
}

#[derive(serde::Serialize, Debug)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct KernelListItem {
    pub state: String,
    notebook_path: String,
    pub inode: String,
}

pub async fn kernel_list(
    ctx: AppContext,
    req: Request<Body>,
) -> Result<Resp<Vec<KernelListItem>>, Error> {
    let req = serde_urlencoded::from_str::<KernelListReq>(req.uri().query().unwrap_or_default())?;

    let mut ret = Vec::new();
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::GetAll(tx))
        .await?;
    let kernel_list = rx.await?;
    for kernel in kernel_list {
        let inode = kernel.inode;
        if kernel.header.project_id != req.project_id {
            continue;
        }
        let (tx, rx) = tokio::sync::oneshot::channel();
        kernel
            .kernel_operate_tx
            .send(KernelOperate::GetState(tx))
            .await?;
        let kernel_state = rx.await.unwrap();
        let state = match kernel_state {
            State::Idle => "idle",
            State::Running(_) => "busy",
            State::Paused { .. } => "pause",
        };
        let kernel_state = KernelListItem {
            state: state.to_string(),
            notebook_path: kernel.header.path.clone(),
            inode: inode.to_string(),
        };
        ret.push(kernel_state);
    }

    Ok(Resp::success(ret))
}
