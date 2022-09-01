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



use crate::AppContext;

// handle OOM killed or coredump
// multi kernel pod will receive same dmesg
pub async fn dmesg_watcher(ctx: AppContext) {
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;
    let mut dmesg = BufReader::new(tokio::fs::File::open("/dev/kmsg").await.unwrap()).lines();
    while let Ok(Some(line)) = dmesg.next_line().await {
        // [4253897.732535] Memory cgroup out of memory: Kill process 24820 (idp_kernel_pyth) score 1450 or sacrifice child
        if line.starts_with("Memory cgroup out of memory") {
            // let re = regex::Regex::new(r"Kill process \d*").unwrap();
            // let re_match = &re.captures_iter(&line).next().unwrap()[0];
            // let pid = re_match
            //     .trim_start_matches("Kill process ")
            //     .parse::<u32>()
            //     .unwrap();
            shutdown_oom_or_coredump_kernel_by_pid(&ctx, &line).await;
        }
        // [22981.296701] kernel_py39[631363]: segfault at ffffffffffffffff ip 00007ffcfc772fc0
        if line.contains("kernel_py[") {
            // let re = regex::Regex::new(r"\[\d*\]").unwrap();
            // let re_match = &re.captures_iter(&line).next().unwrap()[0];
            // let pid = re_match
            //     .trim_start_matches('[')
            //     .trim_end_matches(']')
            //     .parse::<u32>()
            //     .unwrap();
            tracing::warn!("some kernel was coredump");
            // dmesg's pid is host PID, not container pid
            shutdown_oom_or_coredump_kernel_by_pid(&ctx, &line).await;
        }
    }
}

/*
from ctypes import CDLL
libc = CDLL("libc.so.6")
libc.time(-1)

#include <signal.h>
#include <unistd.h>
#include <stdio.h>
int main() {
while (1) {
int ret = kill(620, 0);
printf("ret = %d\n", ret);
sleep(1);
}
return 0;
}
*/
async fn shutdown_oom_or_coredump_kernel_by_pid(ctx: &AppContext, error_reason: &str) {
    // dmesg coredump has a delay until pod pid shutdown
    tracing::error!("{error_reason}");
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    let mapping = ctx.inode_kernel_mapping.read().await;
    let mut inode_opt = None;
    for (inode, kernel) in mapping.iter() {
        tracing::debug!(
            "kernel pid={}, path={}, inode={inode}, project_id={}",
            kernel.pid,
            kernel.header.path,
            kernel.header.project_id
        );
        if unsafe { libc::kill(kernel.pid as _, 0) } == -1 {
            // if kernel.pid == pid {
            tracing::warn!(
                "prepare to shutdown core dumped kernel inode={inode}, path={}",
                kernel.header.path
            );
            inode_opt = Some(*inode);
            break;
        }
    }
    drop(mapping);
    let inode = match inode_opt {
        Some(inode) => inode,
        None => {
            tracing::info!("core dumped kernel is not started in my pod, skipping");
            return;
        }
    };
    let mut mapping = ctx.inode_kernel_mapping.write().await;
    let kernel = mapping.remove(&inode).unwrap();
    let (tx, rx) = tokio::sync::oneshot::channel();
    kernel.shutdown.send(tx).await.unwrap();
    let cell_id_opt = rx.await.unwrap();
    let mut header = kernel.header.clone();
    if let Some(cell_id) = cell_id_opt {
        header.cell_id = cell_id;
    }
    let msg = kernel_common::Message {
        header,
        content: kernel_common::Content::RuntimeError {
            message: error_reason.to_string(),
        },
        ..Default::default()
    };
    if let Err(err) = ctx
        .cache_svc
        .extend_cell_output(
            msg.header.ipynb_abs_path(),
            msg.header.cell_id.clone(),
            vec![msg.content.warp_to_cell_output().unwrap()],
        )
        .await
    {
        tracing::error!("update_cell_output {err}");
    }
    ctx.output_to_ws_sender.send(msg).unwrap();
}
