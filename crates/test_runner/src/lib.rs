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

#![feature(test)]

extern crate test;
use test::TestDescAndFn;
use test::TestFn;

/// test function name with `_it` suffix means require docker
pub fn run_tests(cases: &[&TestDescAndFn]) {
    let is_integration_test = std::env::var("INTEGRATION_TEST").is_ok();
    let cases = cases
        .iter()
        .filter_map(|case| {
            let test_fn_name = case.desc.name.as_slice().to_owned();
            if test_fn_name.ends_with("_it") {
                if is_integration_test {
                    Some(make_owned_test(case))
                } else {
                    None
                }
            } else if is_integration_test {
                None
            } else {
                Some(make_owned_test(case))
            }
        })
        .collect::<Vec<_>>();
    let args = std::env::args().collect::<Vec<_>>();
    test::test_main(&args, cases, None)
}
/// /home/w/.rustup/toolchains/nightly-2022-08-11-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/test/src/lib.rs
fn make_owned_test(test: &&TestDescAndFn) -> TestDescAndFn {
    match test.testfn {
        TestFn::StaticTestFn(f) => TestDescAndFn {
            testfn: TestFn::StaticTestFn(f),
            desc: test.desc.clone(),
        },
        TestFn::StaticBenchFn(f) => TestDescAndFn {
            testfn: TestFn::StaticBenchFn(f),
            desc: test.desc.clone(),
        },
        _ => panic!("non-static tests passed to test::test_main_static"),
    }
}

pub struct IntegrationTestCtx {
    pub port: u16,
    pub client: reqwest::blocking::Client,

    pub team_id: u64,
    pub project_id: u64,
    pub region: &'static str,
}

impl IntegrationTestCtx {
    pub fn get() -> Self {
        let team_id = 12345;
        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::COOKIE,
                    format!("teamId={team_id}").parse().unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self {
            port: std::env::var("GATEWAY_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap(),
            client,
            team_id,
            project_id: 6789,
            region: "a",
        }
    }
    pub fn note_storage_api_url(&self, api_path: &'static str) -> String {
        format!(
            "http://127.0.0.1:{}/{}/api/v2/idp-note-rs{api_path}?teamId={}&projectId={}",
            self.port, self.region, self.team_id, self.project_id
        )
    }
}
