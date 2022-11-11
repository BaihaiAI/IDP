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

pub static REGION: Lazy<String> = Lazy::new(|| get_region_from_hostname(&os_utils::get_hostname()));

/// e.g. idp-develop-b-executor-7b77cd4c6c-n866m -> b
/// e.g. idp-raycluster-a-executor-ray-head-type-rmlft -> a
fn get_region_from_hostname(hostname: &str) -> String {
    match hostname.split('-').nth(2) {
        Some(region) => region.to_string(),
        None => {
            tracing::error!("get region from hostname {hostname} err, use default region a");
            "a".to_string()
        }
    }
}

#[test]
fn test_split_hostname() {
    assert_eq!(
        get_region_from_hostname("idp-develop-b-executor-7b77cd4c6c-n866m"),
        "b"
    );
    assert_eq!(
        get_region_from_hostname("idp-raycluster-a-executor-ray-head-type-rmlft"),
        "a"
    );
}
