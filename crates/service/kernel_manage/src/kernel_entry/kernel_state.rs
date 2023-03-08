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

use kernel_common::typedef::CellId;
// use os_utils::resource_usage::sysinfo;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum KernelState {
    Idle,
    Running(CellId),
    /// paused running cell_id
    Paused(Option<CellId>),
}

impl KernelState {
    pub fn is_idle(&self) -> bool {
        matches!(self, KernelState::Idle)
    }
    pub fn is_busy(&self) -> bool {
        matches!(self, KernelState::Running(_))
    }
}

#[cfg(not)]
pub struct StateWrapper {
    pub notebook_path: String,
    pub inode: String,
    pub header: kernel_common::Header,
    pub state: KernelState,
    // pub sys: sysinfo::System,
    // pub gpu_device_count: u32,
}

#[cfg(not)]
impl StateWrapper {
    pub fn report_state_to_redis_and_pg(
        &mut self,
        _need_update_run_start: bool,
    ) -> Result<(), Error> {
        Ok(())
    }

    pub fn shutdown(&mut self) {}
}

#[cfg(not)]
impl StateWrapper {
    /*
    redis write req time cost 700us-900us
    pg write req time cost 12ms
    2022-06-16T03:51:02.753425Z DEBUG kernel_manage::kernel_entry::kernel_state: 105: after set_kernel_state redis 1.151068ms
    2022-06-16T03:51:02.764683Z DEBUG kernel_manage::kernel_entry::kernel_state: 135: after post to pg 12.411396ms
    */
    pub fn report_state_to_redis_and_pg(
        &mut self,
        need_update_run_start: bool,
    ) -> Result<(), Error> {
        use sysinfo::ProcessExt;
        use sysinfo::SystemExt;
        let is_busy = self.state.is_busy();
        let pid = <sysinfo::Pid as sysinfo::PidExt<_>>::from_u32(self.pid);
        if !sysinfo::SystemExt::refresh_process(&mut self.sys, pid)
            && !matches!(self.state, KernelState::Paused { .. })
        {
            tracing::warn!(
                "kernel not paused but {:?} pid not found in /proc",
                self.header
            );
            // FIXME after criu resume, the sys.refresh_process is wrong would failed
        }
        let (cpu_used, mem_used) = match self.sys.process(pid) {
            Some(proc) => {
                let cpu_used = proc.cpu_usage() / 100.0;
                let mem_used = (proc.memory() / 1024) as i32;
                (cpu_used, mem_used)
            }
            None => {
                tracing::warn!(
                    "kernel {:?} pid not found in /proc maybe paused",
                    self.header
                );
                (0.0, 0)
            }
        };
        tokio::spawn({
            use common_model::api_model::dashboard as dashboard_api_model;
            let header = self.header.clone();
            let region = business::region::REGION.clone();
            let url = dashboard_api_model::insert_or_update_api_url(region.clone());
            let client = reqwest::ClientBuilder::new()
                .connect_timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap();
            let state = self.state.clone();
            let inode = self.inode.clone();
            let now = dashboard_api_model::chrono::Local::now();
            let gpu_cards = self.gpu_device_count;
            async move {
                if let Err(err) = client
                    .post(&url)
                    .json(&dashboard_api_model::IdpTaskMonitorQto {
                        kernel_source: "workspace".to_string(),
                        team_id: header.team_id,
                        project_id: header.project_id as u32,
                        path: header.path.clone(),
                        region: Some(region.clone()),
                        inode: Some(inode),
                        status: Some(
                            match state {
                                KernelState::Idle => "idle",
                                KernelState::Running(_) => "busy",
                                KernelState::Paused { .. } => "paused",
                            }
                            .to_string(),
                        ),
                        cpu_used: Some(cpu_used),
                        mem_used: Some(mem_used),
                        gpu_used: Some(gpu_cards as _),
                        run_start: if need_update_run_start {
                            Some(now)
                        } else {
                            None
                        },
                        run_end: if is_busy { Some(now) } else { None },
                        ..Default::default()
                    })
                    .send()
                    .await
                {
                    tracing::error!("{err}");
                }
                // tracing::debug!("after post to pg {:?}", start.elapsed());
            }
        });
        if matches!(self.state, KernelState::Paused { .. }) {
            tracing::info!("state is paused");
        } else if cfg!(not(feature = "tcp")) {
            let mut retry = 0;
            loop {
                if unsafe { libc::kill(self.pid as _, 0) } == -1 {
                    retry += 1;
                    if retry == 3 {
                        return Err(Error::new("kernel crash"));
                    }
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}
