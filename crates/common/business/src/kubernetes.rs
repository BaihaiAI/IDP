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

use once_cell::sync::Lazy;

pub use crate::region::REGION;

/*
def resource_account() -> str:
    """
    public  user: idp-develop-a-executor-job-0 -> executor
    private user:
    """
    return socket.gethostname().split("-")[3]
*/
/// team_id or executor(public user)
pub static ACCOUNT: Lazy<String> = Lazy::new(|| {
    if !is_k8s() {
        return "1".to_string();
    }
    let hostname = os_utils::get_hostname();
    let mut hostname_parts = hostname.split('-').skip(2).take(2);
    let _region = hostname_parts.next().expect("not found region in hostname");
    let account = hostname_parts
        .next()
        .expect("not found team_id/executor in hostname");
    account.to_string()
});

pub static NAMESPACE: Lazy<String> = Lazy::new(|| {
    if !is_k8s() {
        return "default_k8s_namespace".to_string();
    }
    std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace").unwrap()
});

fn runtime_pod_id(project_id: u64) -> String {
    let account = &*ACCOUNT;
    format!("{account}-{project_id}-runtime")
}

/*
svc: idp-kernel-b-12345-100-runtime-svc
pod: idp-kernel-b-12345-100-runtime-job-0
container: idp-kernel-b-12345-100-runtime
*/
pub fn runtime_pod_container(project_id: u64) -> String {
    let pod_id = runtime_pod_id(project_id);
    let platform = "idp-kernel";
    let region = &*REGION;
    format!("{platform}-{region}-{pod_id}")
}

pub fn runtime_pod_svc(project_id: u64) -> String {
    // these svc format DNS query is slow
    // format!("{}-svc", runtime_pod_container(project_id))
    format!(
        "{}-svc.{}.svc.cluster.local",
        runtime_pod_container(project_id),
        &*NAMESPACE
    )
}

pub fn runtime_pod_name(project_id: u64) -> String {
    format!("{}-job-0", runtime_pod_container(project_id))
}

#[cfg(not)]
pub async fn runtime_pod_is_running_by_health_check(project_id: u64) -> bool {
    const TIMEOUT_MS: u64 = 500;
    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(TIMEOUT_MS))
            .build()
            .unwrap()
    });

    let host = runtime_pod_svc(project_id);
    let port = crate::spawner_port();
    let url = format!("http://{host}:{port}/health_check");
    match CLIENT.get(&url).send().await {
        Ok(rsp) => {
            if rsp.status() != 200 {
                tracing::debug!("rsp.status() {}", rsp.status());
                false
            } else {
                true
            }
        }
        Err(err) => {
            tracing::debug!("{url} timeout_ms={TIMEOUT_MS} {err}");
            false
        }
    }
}

#[cfg(not)]
pub fn runtime_pod_is_running_sync(project_id: u64) -> bool {
    const TIMEOUT_MS: u64 = 2000;
    let host = runtime_pod_svc(project_id);
    let port = crate::spawner_port();
    let url = format!("http://{host}:{port}/health_check");
    match ureq::get(&url)
        .timeout(std::time::Duration::from_millis(TIMEOUT_MS))
        .call()
    {
        Ok(rsp) => {
            if rsp.status() != 200 {
                tracing::warn!("rsp.status() {}", rsp.status());
                false
            } else {
                true
            }
        }
        Err(err) => {
            tracing::warn!("{url} timeout_ms={TIMEOUT_MS} {err}");
            false
        }
    }
}

pub fn tenant_cluster_header_k8s_svc() -> String {
    if !is_k8s() {
        return "127.0.0.1".to_string();
    }
    let hostname = os_utils::get_hostname();
    let region = &*REGION;
    let team_id = &*ACCOUNT;
    if hostname.contains("raycluster") {
        format!("idp-raycluster-{region}-{team_id}-ray-head")
    } else {
        format!("idp-develop-{region}-{team_id}-svc")
    }
}

#[cfg(test)]
fn get_region_team_id_from_hostname(hostname: &str) -> (String, String) {
    let mut parts = hostname.split('-').skip(2);
    let region = parts.next().unwrap();
    let team_id = parts.next().unwrap();
    (region.to_string(), team_id.to_string())
}

#[test]
fn test_get_region_team_id_from_hostname() {
    assert_eq!(
        get_region_team_id_from_hostname("idp-kernel-b-1586198890356670464-22227-job-0"),
        ("b".to_string(), "1586198890356670464".to_string())
    );
    assert_eq!(
        get_region_team_id_from_hostname("idp-kernel-a-executor-22227-job-0"),
        ("a".to_string(), "executor".to_string())
    );
}

#[cfg(test)]
fn cluster_header_k8s_svc_inner(hostname: &str) -> String {
    let mut parts = hostname.rsplit('-').skip(3).collect::<Vec<_>>();
    parts.reverse();
    parts.push("head");
    parts.join("-")
}

#[test]
fn test_cluster_header_k8s_svc_inner() {
    assert_eq!(
        cluster_header_k8s_svc_inner("idp-raycluster-b-1546774368495616000-ray-worker-type-tswnj"),
        "idp-raycluster-b-1546774368495616000-ray-head"
    )
}
pub fn is_k8s() -> bool {
    // FIXME why our supervisor in k8s pod not contains this env but kubectl exec /bin/bash contains this env
    // std::env::var("KUBERNETES_SERVICE_HOST").is_ok()
    std::path::Path::new("/var/run/secrets/kubernetes.io").exists()
}
