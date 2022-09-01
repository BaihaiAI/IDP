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

/*!
## get k8s/docker cgroup resource limit

|env(from k8s yaml)|default|comment|
|---|---|---|
|IDP_EXECUTE_MEM_LIMIT_K|2097152|
|IDP_EXECUTE_STORE_LIMIT_B|21474836480|/store/$team_id store limit|
|IDP_EXECUTE_CPU_LIMIT_M|2000|???|

mem_limit_k:   ${HALO_EXECUTE_MEM_LIMIT_K:16777216}
store_limit_b: ${HALO_EXECUTE_STORE_LIMIT_B:53687091200}
*/
pub use sysinfo;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;

pub fn gpu_device_count() -> Result<u32, nvml_wrapper::error::NvmlError> {
    let nvml = nvml_wrapper::Nvml::init()?;
    nvml.device_count()
}

// mem is read from /proc/pid/stat rss field
#[cfg(not)]
pub fn get_pid_mem_and_cpu_used(pid: u32) -> Option<(i32, f32)> {
    use sysinfo::PidExt;
    use sysinfo::ProcessExt;
    use sysinfo::SystemExt;
    let sysinfo_pid = sysinfo::Pid::from_u32(pid);
    let mut sys = sysinfo::System::new();
    if !sys.refresh_process(sysinfo_pid) {
        return None;
    }
    unsafe { libc::sleep(1) };
    sys.refresh_process(sysinfo_pid);
    let p = sys.process(sysinfo_pid)?;
    Some(((p.memory() / 1024) as i32, p.cpu_usage() / 100.0))
}

#[cfg(not)]
#[test]
fn test_get_pid_mem_and_cpu_used() {
    dbg!(get_pid_mem_and_cpu_used(std::process::id()).unwrap());
}

#[cfg(not)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceUsage {
    pub cpu: f32,
    /// mem_used / mem_total
    pub mem: f32,
    /// avg of all graphics card mem usage
    pub gpu: f32,
}
#[cfg(not)]
impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu: 0.0,
            mem: 0.0,
            gpu: 0.0,
        }
    }
}

pub struct ResourceUsageFetcher {
    sys: sysinfo::System,
    // root_partition_index: usize,
    pid: sysinfo::Pid,

    nvml_opt: Option<nvml_wrapper::Nvml>,
    pub gpu_device_count: u32,

    /// in bytes
    last_gpu_used_mem: u64,
    gpu_total_mem: u64,

    // team_id: u64,
    // pub last_team_id_folder_size: u64,
    last_pod_mem_used_kb: u64,
    mem_limit_kb: u64,

    pub last_pod_cpu_usage: f32,
    cpu_cores: u32,
}

impl ResourceUsageFetcher {
    pub fn new(_team_id: u64) -> Self {
        // let mut sys = sysinfo::System::new_with_specifics(
        // sysinfo::RefreshKind::new().with_cpu().with_memory().with_processes(sysinfo::ProcessRefreshKind::new().with_cpu()),
        // );
        let mut sys = sysinfo::System::new();
        sys.refresh_disks_list();
        // let root_partition_index = sys
        //     .disks()
        //     .iter()
        //     .position(|disk| disk.mount_point() == std::path::Path::new("/"))
        //     .unwrap();

        let (nvml_opt, gpu_device_count, gpu_total_mem) = (|| {
            let nvml = nvml_wrapper::Nvml::init()?;
            let gpu_device_count = nvml.device_count()?;
            let mut gpu_total_mem = 0;
            for i in 0..gpu_device_count {
                let device = nvml.device_by_index(i)?;
                let memory_info = device.memory_info()?;
                gpu_total_mem += memory_info.total;
            }
            Ok::<_, nvml_wrapper::error::NvmlError>((
                Some(nvml),
                Some(gpu_device_count),
                Some(gpu_total_mem),
            ))
        })()
        .unwrap_or_default();
        let gpu_device_count = gpu_device_count.unwrap_or_default();
        let gpu_total_mem = gpu_total_mem.unwrap_or_default();

        let mut self_ = Self {
            sys,
            // root_partition_index,
            pid: sysinfo::Pid::from(std::process::id() as libc::pid_t),
            nvml_opt,
            last_gpu_used_mem: 0,
            gpu_device_count,
            gpu_total_mem,
            // store_limit_b: {
            //     match std::env::var("IDP_EXECUTE_STORE_LIMIT_B") {
            //         Ok(val) => val.parse().unwrap(),
            //         Err(_) => 16777216,
            //     }
            // },
            mem_limit_kb: {
                match std::env::var("IDP_EXECUTE_MEM_LIMIT_K") {
                    Ok(val) => val.parse().unwrap(),
                    Err(_) => 53687091200,
                }
            },
            last_pod_mem_used_kb: 0,
            last_pod_cpu_usage: 0.0,
            cpu_cores: {
                match std::env::var("IDP_CPU_LIMIT_CORE") {
                    Ok(val) => val.parse().unwrap(),
                    Err(_) => 1,
                }
            },
        };
        self_.refresh();
        self_
    }
    pub fn refresh(&mut self) {
        // self.sys.refresh_process(self.pid);
        // let start = std::time::Instant::now();
        self.sys.refresh_processes();
        // dbg!(start.elapsed());
        let mut pod_mem_used = 0;
        let mut pod_cpu_usage = 0.0;
        for proc in self.sys.processes().values() {
            pod_mem_used += proc.memory();
            pod_cpu_usage += proc.cpu_usage();
        }
        pod_cpu_usage /= 100.0;
        self.last_pod_mem_used_kb = pod_mem_used;
        self.last_pod_cpu_usage = pod_cpu_usage;
        // only way to calculate pod's cpu/mem usage is accumulate /proc/$pid usage
        // self.sys.refresh_cpu();
        // self.sys.refresh_memory();
        // self.sys.disks_mut()[self.root_partition_index].refresh();

        // self.last_team_id_folder_size =
        //     crate::count_dir_sizes_in_bytes(&format!("/store/{}", self.team_id))
        //         .unwrap_or_default();
        // dbg!(start.elapsed());

        let nvml = match self.nvml_opt {
            Some(ref nvml) => nvml,
            None => return,
        };
        let gpu_used_mem = (|| {
            let mut gpu_used_mem = 0;
            for i in 0..self.gpu_device_count {
                let device = nvml.device_by_index(i)?;
                let memory_info = device.memory_info()?;
                gpu_used_mem += memory_info.used;
            }
            Ok::<_, nvml_wrapper::error::NvmlError>(gpu_used_mem)
        })()
        .unwrap_or_default();
        self.last_gpu_used_mem = gpu_used_mem;
    }
    #[cfg(not)]
    pub fn resource_usage(&self) -> ResourceUsage {
        ResourceUsage {
            cpu: self.cpu_usage(),
            mem: self.mem_usage(),
            gpu: self.last_gpu_used_mem as f32 / self.gpu_total_mem as f32,
            // store_limit_b: self.store_limit_b,
        }
    }
    pub fn current_process_cpu_usage(&self) -> f32 {
        self.sys.process(self.pid).unwrap().cpu_usage() / 100.0
    }
    pub fn current_process_mem_used_in_mb(&self) -> i32 {
        (self.sys.process(self.pid).unwrap().memory() / 1024) as _
    }
    /// sample /proc/stat twice and calc diff as cpu usage in duration
    /// /proc store total cpu usage after machine boot
    #[cfg(not)]
    fn cpu_usage(&self) -> f32 {
        // these two line is calculate host machine cpu usage not pod itself cpu usage
        // let processor = self.sys.processors();
        // self.cpu_usage_total_cores() / processor.len() as f32
        self.last_pod_cpu_usage / self.cpu_cores as f32
    }
    /// libc::fstatvfs
    // fn disk_usage(&self) -> f32 {
    //     self.last_team_id_folder_size as f32 / self.store_limit_b as f32
    // }
    #[cfg(not)]
    fn mem_usage(&self) -> f32 {
        // let mem_usage = self.sys.used_memory() as f64 / self.sys.total_memory() as f64;
        // mem_usage as f32
        self.last_pod_mem_used_kb as f32 / self.mem_limit_kb as f32
    }
}
