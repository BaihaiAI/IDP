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

#[derive(serde::Deserialize)]
struct Ctx {
    team_id: u64,
    domain: String,
    id_token: String,
}

#[test]
#[ignore]
fn x() {
    let ctx = toml::from_str::<Ctx>(&std::fs::read_to_string("target/test_config.toml").unwrap())
        .unwrap();
    let Ctx {
        team_id,
        domain,
        id_token,
    } = ctx;

    let mut headers = reqwest::header::HeaderMap::new();
    // cookie missing mode,teamId,id_token: 404
    // cookie id_token invalid: Jwt issuer is not configured
    headers.insert(
        reqwest::header::COOKIE,
        format!("mode=private; teamId={team_id}; id_token={id_token}")
            .parse()
            .unwrap(),
    );
    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .connect_timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap();

    let rsp = client.get(format!("http://{domain}/b/api/v2/idp-note-rs/visual-modeling/run/status?teamId={team_id}&jobInstanceId=1")).send().unwrap().text().unwrap();
    dbg!(rsp);
}
