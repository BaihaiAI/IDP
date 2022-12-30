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

use common_model::api_model::PartialUpdateCellReq;
use common_model::entity::cell::CellUpdate;
use common_model::entity::cell::Updates;
use tracing::error;

use super::prelude::*;
use crate::handler::prelude::State;

impl super::KernelCtx {
    pub async fn handle_kernel_execute_resp(&mut self, resp: kernel_common::Message) {
        let start = std::time::Instant::now();

        // clear cell output when receive ExecuteInput
        if matches!(resp.content, Content::ExecuteInput { .. }) {
            let team_id = resp.header.team_id;
            let req = PartialUpdateCellReq {
                path: resp.header.path.clone(),
                project_id: resp.header.project_id,
                cells: vec![CellUpdate {
                    id: resp.header.cell_id.clone(),
                    updates: Updates {
                        outputs: Some(Vec::new()),
                        ..Default::default()
                    },
                }],
            };
            tokio::spawn(async move {
                // info!("execute_input clear cell output: before send req to note_storage");
                put_cell_update_req(req, team_id).await;
                // info!("execute_input clear cell output: after  send req to note_storage");
            });
            _ = self.cell_update.remove(&resp.header.cell_id);
        }

        let cell = self
            .cell_update
            .entry(resp.header.cell_id.clone())
            .or_default();
        if let kernel_common::Content::ExecuteInput {
            execution_count, ..
        } = resp.content
        {
            cell.execution_count = Some(execution_count as _);
        }
        #[allow(unused_variables)]
        if let kernel_common::Content::Duration {
            duration,
            ref code,
            run_at,
        } = resp.content
        {
            cell.execution_time = Some(duration.to_string());

            #[cfg(not)]
            let data_source_list =
                common_model::api_model::get_develop_active_connect_data(resp.header.project_id)
                    .await
                    .map(|ret| ret.data)
                    .unwrap_or_default();
            #[cfg(not)]
            let run_record = crate::handler::execute_record::ExecuteRecord {
                code: code.clone(),
                data_source_list,
                duration_ms: duration,
                run_at: chrono::NaiveDateTime::from_timestamp_opt(run_at as _, 0)
                    .expect("chrono from_timestamp_opt"),
                header: resp.header.clone(),
            };
            #[cfg(not)]
            if let Err(err) = self.execute_record_db.insert(
                run_record.key().into_bytes(),
                serde_json::to_vec(&run_record).unwrap(),
            ) {
                error!("{err}");
            }
        }
        if let Some(output) = resp.content.warp_to_cell_output() {
            match cell.outputs {
                Some(ref mut arr) => {
                    arr.push(output);
                }
                None => {
                    cell.outputs = Some(vec![output]);
                }
            }
        }

        if resp.is_error() {
            tracing::debug!("cell_id execute error {}", resp.header.cell_id);
            self.err_cell_ids.insert(resp.header.cell_id.clone());
        }
        if resp.is_idle() {
            // pending.is_empty means all req send to kernel, does not mean kernel finish all req
            if self.pending_req.is_empty() {
                self.update(State::Idle);
            }
            if self.err_cell_ids.contains(&resp.header.cell_id) {
                tracing::debug!("pop all pending_req and reply stop on error");
                self.err_cell_ids.remove(&resp.header.cell_id);
                while let Some(req) = self.pending_req.pop_front() {
                    // req.parent_header.msg_id = req.header.msg_id.clone();
                    let mut rsp = req;
                    rsp.content = Content::ReplyOnStop {};
                    if let Err(err) = self.broadcast_output_to_all_ws_write.send(rsp) {
                        error!("{err}");
                    }
                }
                self.update(State::Idle);
            } else if let Some(req) = self.pending_req.pop_front() {
                self.send_req_to_kernel(req).await;
            }
        }
        if let Err(err) = self.broadcast_output_to_all_ws_write.send(resp.clone()) {
            error!("{err}");
        };

        if self.last_persist_output.elapsed() > std::time::Duration::from_secs(3) {
            if let Err(err) = self.persist_cell_output() {
                error!("{err}");
            }
            self.last_persist_output = std::time::Instant::now();
        }

        tracing::trace!(
            "<-- handle_kernel_execute_resp, time cost = {:?}",
            start.elapsed()
        );
    }

    pub fn update(&mut self, new_state: State) {
        if self.state.is_idle() && new_state.is_busy() {
            tracing::info!("kernel_state: idle->busy");
        }
        if self.state.is_busy() && new_state.is_idle() {
            tracing::info!("kernel_state: busy->idle");
            self.kernel_shutdown_time = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                + self.shutdown_idle_interval_duration;

            if !self.cell_update.is_empty() {
                if let Err(err) = self.persist_cell_output() {
                    tracing::error!("{err:#?}");
                }
            }
        }
        self.state = new_state;
    }

    fn persist_cell_output(&mut self) -> Result<(), crate::Error> {
        use common_model::entity::notebook::Notebook;
        tracing::debug!("--> persist_cell_output");

        let mut req = PartialUpdateCellReq {
            path: self.header.path.clone(),
            project_id: self.header.project_id,
            cells: Vec::new(),
        };
        for (cell_id, update) in &self.cell_update {
            req.cells.push(CellUpdate {
                id: cell_id.clone(),
                updates: update.clone(),
            });
        }
        if self.header.pipeline_opt.is_some() {
            let dst_path = self.header.ipynb_abs_path();
            tracing::info!("persist_cell_output pipeline dst_path={dst_path:?}");
            let nb = std::fs::read_to_string(&dst_path)?;
            let mut nb = serde_json::from_str::<Notebook>(&nb)?;
            let cell_id_index_map = nb
                .cells
                .iter()
                // first cell index can't be 0
                // .zip(1..)
                .enumerate()
                .filter_map(|(index, cell)| cell.id().map(|id| (id, index)))
                .collect::<std::collections::HashMap<_, _>>();
            tracing::debug!("cell_id_index_map = {cell_id_index_map:#?}");
            for cell_update in req.cells {
                if let Some(index) = cell_id_index_map.get(&cell_update.id) {
                    let cell = &mut nb.cells[*index];
                    cell.execution_count = cell_update.updates.execution_count;
                    cell.execution_time = cell_update.updates.execution_time;
                    if let Some(outputs) = cell_update.updates.outputs {
                        cell.outputs = outputs.clone();
                    }
                } else {
                    tracing::warn!(
                        "cell_update.id = {} not found in ipynb, skip update output",
                        cell_update.id
                    );
                }
            }
            std::fs::write(dst_path, serde_json::to_string(&nb)?)?;
        } else {
            let team_id = self.header.team_id;
            tokio::spawn(async move {
                put_cell_update_req(req, team_id).await;
            });
        }
        tracing::debug!("<-- persist_cell_output");
        Ok(())
    }
}

async fn put_cell_update_req(req: PartialUpdateCellReq, team_id: u64) {
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap();
    let cell_ids = req.cells.iter().map(|x| x.id.clone()).collect::<Vec<_>>();
    tracing::debug!(
        "req->note_storage[partial_update_cell]: path={}, project_id={}, cell_ids={:?}",
        req.path,
        req.project_id,
        cell_ids
    );
    let resp = client
        .put(format!(
            "http://{}:{}/api/v2/idp-note-rs/content/cell?teamId={team_id}",
            business::kubernetes::tenant_cluster_header_k8s_svc(),
            business::note_storage_port()
        ))
        .json(&req)
        .send()
        .await;
    match resp {
        Ok(resp) => {
            if !resp.status().is_success() {
                tracing::warn!("{}", resp.status());
                if let Ok(text) = resp.text().await {
                    tracing::error!("{text}");
                }
            }
        }
        Err(err) => {
            tracing::error!("{err}");
        }
    }
}
