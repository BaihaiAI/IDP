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

use super::execute_req_model;
use super::sql_cell_wrapper;
use super::ExecuteCodeReq;
use crate::app_context::KernelEntryOps;
use crate::AppContext;
use crate::Error;

pub(crate) async fn add_req_to_pending(ctx: &AppContext, req: ExecuteCodeReq) -> Result<(), Error> {
    if req.resource.num_cpu == 0.0 {
        return Err(Error::new("cpu must > 0"));
    }
    if req.resource.memory == 0.0 {
        return Err(Error::new("memory must > 0"));
    }
    if !matches!(req.resource.priority, 1..=99) {
        return Err(Error::new("priority must between 1 to 99"));
    }

    let header = req.header.clone();
    if cfg!(debug_assertions) && req.header.pipeline_opt.is_some() {
        tracing::info!("ws req header = {:?}", req.header);
    }
    let req_msg = if let Some(input_reply) = req.input_reply {
        kernel_common::Message {
            header: header.clone(),
            content: kernel_common::Content::InputReply { value: input_reply },
            ..Default::default()
        }
    } else {
        let code = match &req.cell_type {
            execute_req_model::CellTypeMeta::Code {} => req.code,
            execute_req_model::CellTypeMeta::Sql(sql_cell) => {
                sql_cell_wrapper::sql2python(sql_cell, &req)
            }
            execute_req_model::CellTypeMeta::Visualization(req) => {
                crate::handler::execute_code::visual_cell_wrapper::vis2python(&req)
            }
            execute_req_model::CellTypeMeta::Visualization2 { df_name, chart } => {
                format!(
                    "__import__('baihai_aid').draw_dataframe({df_name}, '{}')",
                    serde_json::to_string(chart).unwrap()
                )
            }
        };
        kernel_common::Message {
            header: header.clone(),
            content: kernel_common::Content::ExecuteRequest(
                kernel_common::content::ExecuteRequest {
                    code,
                    ..Default::default()
                },
            ),
            ..Default::default()
        }
    };

    let inode = req.header.inode();
    tracing::debug!(
        "receive ws execute code req, path: {}, inode = {inode}",
        req.header.path
    );

    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::Get(inode, tx))
        .await?;
    let kernel_opt = rx.await?;
    match kernel_opt {
        Some(kernel) => {
            kernel.req_sender.send(req_msg).await?;
        }
        None => {
            let (tx, rx) = tokio::sync::oneshot::channel();
            let resource = req.resource;
            ctx.kernel_entry_ops_tx
                .send(KernelEntryOps::Insert {
                    header,
                    resource,
                    ctx: ctx.clone(),
                    tx,
                })
                .await?;
            let kernel = rx.await??;
            kernel.req_sender.send(req_msg).await?;
        }
    }

    Ok(())
}

// fn req_context
