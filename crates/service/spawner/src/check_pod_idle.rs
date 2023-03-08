// Copyright 2023 BaihaiAI, Inc.
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

#[cfg(not(target_os = "linux"))]
const fn is_runtime_pod_idle() -> bool {
    false
}

#[cfg(target_os = "linux")]
fn is_runtime_pod_idle() -> bool {
    for proc in procfs::process::all_processes()
        .expect("procfs::process::all_processes()")
        .flatten()
    {
        if let Ok(args) = proc.cmdline() {
            if args.len() >= 2
                && (args == ["bash", "--login"]
                    || args[0].starts_with("kernel_py")
                    || args[0].starts_with("/usr/local/bin/kernel_py"))
            {
                return false;
            }
        }
    }
    true
}

pub(crate) fn spawn_check_pod_idle_thread() {
    tokio::spawn(async {
        let now = std::time::Instant::now();
        loop {
            // pod must at least run 20 minute
            if now.elapsed().as_secs() > 20 * 60 && is_runtime_pod_idle() {
                tracing::info!("shutdown idle runtime pod");
                shutdown_hook().await;
                tokio::process::Command::new("supervisorctl")
                    .arg("shutdown")
                    .spawn()
                    .unwrap()
                    .wait()
                    .await
                    .unwrap();
                std::process::exit(0);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10 * 60)).await;
        }
    });
}

pub async fn shutdown_hook() -> &'static str {
    tokio::process::Command::new("idp_hook")
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    ""
}
