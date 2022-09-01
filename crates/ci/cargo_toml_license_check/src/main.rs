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

fn main() {
    check_workspace_license();
}

const LICENSE_NAME: &str = "Apache-2.0";

fn check_workspace_license() {
    let config = cargo::Config::default().unwrap();
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../Cargo.toml"));
    let path = path.canonicalize().unwrap();
    // alternative crate to parse workspace is rust-analyzer
    let workspace = cargo::core::Workspace::new(path.as_path(), &config).unwrap();
    // cargo::util::toml::read_manifest
    for package in workspace.members() {
        if let Some(ref license) = package.manifest().metadata().license {
            if license != LICENSE_NAME {
                panic!("{package} license {license} not {LICENSE_NAME}");
            }
        } else {
            panic!("{package} license not found");
        }
    }
}
