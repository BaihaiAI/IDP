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
pub static ACCOUNT: Lazy<String> = Lazy::new(|| {
    if !is_k8s() {
        return "1".to_string();
    }
    let hostname = os_utils::get_hostname();
    let mut hostname_parts = hostname.split("-").skip(2).take(2);
    let _region = hostname_parts.next().expect("not found region in hostname");
    let account = hostname_parts
        .next()
        .expect("not found team_id/executor in hostname");
    account.to_string()
});

pub fn runtime_pod_svc(project_id: u64) -> String {
    let region = &*REGION;
    let account = &*ACCOUNT;
    let pod_id = format!("{account}-{project_id}-runtime");
    let platform = "idp-kernel";
    format!("{platform}-{region}-{pod_id}-svc")
}

pub fn runtime_pod_is_running(project_id: u64) -> bool {
    std::net::TcpStream::connect(format!("{}:8089", runtime_pod_svc(project_id))).is_ok()
}

#[allow(unreachable_code)]
pub fn cluster_header_k8s_svc() -> String {
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
