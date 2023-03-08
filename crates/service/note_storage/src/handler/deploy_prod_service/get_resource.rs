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

use super::*;

pub async fn get_resource_handler(team_id: i64, region: &str) -> Result<ResourceRsp, ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let namespace = &*business::kubernetes::NAMESPACE;

    let req = GetResourceBody {
        action: "resource",
        service: "volcano",
        account: team_id.to_string(),
        region: region.to_string(),
        namespace: namespace.to_string(),
    };
    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/resource"))
        .json(&req)
        .send()
        .await?;

    let resp_body = resp.json::<RspBody>().await?;
    let resp_code = resp_body.code.to_string();
    if !resp_code.starts_with('2') {
        tracing::error!(
            "code:{resp_code},message:{},data:{:?}",
            resp_body.message,
            resp_body.data
        );
        return Err(ErrorTrace::new(&resp_body.message));
    }

    let resp = serde_json::from_value::<ResourceData>(resp_body.data)?;
    let mut resp = resp.info[0].clone();
    resp.modify_unit();
    Ok(resp)
}
