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

pub async fn interrupt(ctx: AppContext, req: Request<Body>) -> Result<Resp<()>, Error> {
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
    let state = rx.await?;
    if !matches!(state, State::Running(_)) {
        return Err(Error::new(&format!(
            "only running kernel can interrupt, current state is {state:?}"
        )));
    }

    kernel
        .kernel_operate_tx
        .send(KernelOperate::Interrupt)
        .await?;
    Ok(Resp::success(()))
}
