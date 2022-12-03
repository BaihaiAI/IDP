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

use common_model::Rsp;
use err::ErrorTrace;

pub async fn runtime_pod_status() -> Result<Rsp<bool>, ErrorTrace> {
    if !business::kubernetes::is_k8s() {
        return Ok(Rsp::success(true));
    }
    let region = &*business::kubernetes::REGION;
    let account = &*business::kubernetes::ACCOUNT;
    let platform = "idp-kernel";
    let svc = format!("{platform}-{region}-{account}-svc");
    let pod_is_running = tokio::net::TcpStream::connect(format!("{svc}:8089"))
        .await
        .is_ok();
    Ok(Rsp::success(pod_is_running))
}
