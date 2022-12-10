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

use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

lazy_static! {
    static ref INVISABLE_EXTENSION: HashSet<&'static str> = {
        let a = std::fs::read_to_string("/opt/extension_config.json").unwrap();
        let extension_config: ExtensionConfig = serde_json::from_str(&a).unwrap();
        let mut m: HashSet<&'static str> = HashSet::new();
        extension_config.invisible.into_iter().for_each(|x| {
            m.insert(Box::leak(x.into_boxed_str()));
        });
        m
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionConfig {
    pub init: Vec<String>,
    pub invisible: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub user_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub user_id: u64,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionResp {
    pub name: String,
    pub version: String,
    pub url: Option<String>,
    pub entry: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub visible: Option<bool>,
}

impl PartialEq for ExtensionResp {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}
impl Eq for ExtensionResp {}

impl PartialOrd for ExtensionResp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.version.partial_cmp(&other.version)
    }
}
impl Ord for ExtensionResp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        let self_version = Version::get_version(&self.version);
        let other_version = Version::get_version(&other.version);
        self_version.cmp(&other_version)
    }
}

impl ExtensionResp {
    pub fn is_visible(&self) -> bool {
        !INVISABLE_EXTENSION.contains(self.name.as_str())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstalledExtensionResp {
    pub name: String,
    pub version: String,
    pub optional_version: Option<Vec<String>>,
    pub url: Option<String>,
    pub entry: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
    pub visible: Option<bool>,
}

impl PartialEq for InstalledExtensionResp {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}
impl Eq for InstalledExtensionResp {}

impl PartialOrd for InstalledExtensionResp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.version.partial_cmp(&other.version)
    }
}

impl Ord for InstalledExtensionResp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.cmp(&other.name) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        let self_version = Version::get_version(&self.version);
        let other_version = Version::get_version(&other.version);
        self_version.cmp(&other_version)
    }
}

impl InstalledExtensionResp {
    pub fn is_visible(&self) -> bool {
        !INVISABLE_EXTENSION.contains(self.name.as_str())
    }
}

pub struct Version {
    pub version: String,
}

impl Version {
    pub fn get_version(version: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9]+(\.[0-9]+)*").unwrap();
        }
        if !RE.is_match(version) {
            tracing::error!("Invalid version format,version:{}", version);
            return Version {
                version: "".to_owned(),
            };
        }
        Version {
            version: version.to_owned(),
        }
    }
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_parts: Vec<u32> = self
            .version
            .split('.')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let other_parts: Vec<u32> = other
            .version
            .split('.')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let self_len = self_parts.len();
        let other_len = other_parts.len();
        let length = self_len.max(other_len);
        for i in 0..=length - 1 {
            let self_part = if i < self_len { self_parts[i] } else { 0 };
            let other_part = if i < other_len { other_parts[i] } else { 0 };
            match self_part.cmp(&other_part) {
                std::cmp::Ordering::Equal => {}
                order => return order,
            }
        }
        std::cmp::Ordering::Equal
    }
}

#[test]
fn test_version_cmp() {
    let a = "1.0.9".to_owned();
    let b = "1.0".to_owned();
    let version_a = Version::get_version(&a);
    let version_b = Version::get_version(&b);
    println!("{:?}", version_a.cmp(&version_b))
}
