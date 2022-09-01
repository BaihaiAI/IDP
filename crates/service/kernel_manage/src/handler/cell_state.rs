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

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct CellStateReq {
    team_id: TeamId,
    project_id: ProjectId,
    path: String,
}

pub type CellStateResp = Vec<CellStateItem>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CellStateItem {
    cell_id: CellId,
    state: CellState,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum CellState {
    Running,
    Pending,
    Paused,
}

#[test]
fn test_deserialize_req() {
    let req = serde_urlencoded::from_str::<CellStateReq>(
        "path=%2F%F0%9F%98%82.ipynb&projectId=1&teamId=123",
    )
    .unwrap();
    assert_eq!(req.path, "/😂.ipynb");
}

/// execute/state?path=/%E7%8E%A9%E8%BD%ACIDP/%E5%BF%AB%E9%80%9F%E4%B8%8A%E6%89%8B/helloworld.ipynb&projectId=1
pub async fn cell_state(ctx: AppContext, req: Request<Body>) -> Result<Resp<CellStateResp>, Error> {
    let req =
        serde_urlencoded::from_str::<CellStateReq>(req.uri().query().unwrap_or_default()).unwrap();
    let header = kernel_common::Header {
        project_id: req.project_id,
        path: req.path,
        team_id: req.team_id,
        ..Default::default()
    };
    let inode = header.inode();
    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.kernel_entry_ops_tx
        .send(KernelEntryOps::Get(inode, tx))
        .await?;
    let kernel_opt = rx.await?;
    let kernel = match kernel_opt {
        Some(kernel) => kernel,
        None => {
            // kernel not start but request cell_state on ipynb open
            return Ok(Resp::success(Vec::new()));
        }
    };

    let (tx, rx) = tokio::sync::oneshot::channel();
    kernel
        .kernel_operate_tx
        .send(KernelOperate::GetPendingReq(tx))
        .await?;
    let mut ret = rx
        .await
        .unwrap()
        .into_iter()
        .map(|cell_id| CellStateItem {
            cell_id,
            state: CellState::Pending,
        })
        .collect::<Vec<_>>();

    let (tx, rx) = tokio::sync::oneshot::channel();
    kernel
        .kernel_operate_tx
        .send(KernelOperate::GetState(tx))
        .await?;
    match rx.await.unwrap() {
        State::Running(cell_id) => {
            ret.push(CellStateItem {
                cell_id,
                state: CellState::Running,
            });
        }
        State::Paused(Some(cell_id)) => ret.push(CellStateItem {
            cell_id,
            state: CellState::Paused,
        }),
        _ => {}
    }
    Ok(Resp::success(ret))
}

#[cfg(FALSE)]
#[tokio::test]
async fn test_cell_state() {
    let ctx = AppContext::new("127.0.0.1".to_string()).await;
    let mut req = Request::<Body>::default();
    *req.uri_mut() = hyper::Uri::builder()
        .path_and_query(format!(
            "/?{}",
            serde_urlencoded::to_string(&IpynbReq {
                path: "%2F%F0%9F%98%82.ipynb".to_string(),
                project_id: 1,
                team_id: 0
            })
            .unwrap()
        ))
        .build()
        .unwrap();
    dbg!(cell_state(ctx, req).await.unwrap());
}
