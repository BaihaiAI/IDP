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

#[cfg(not)]
fn add_workspace_license() {
    let config = cargo::Config::default().unwrap();
    let path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../Cargo.toml"));
    let path = path.canonicalize().unwrap();
    // alternative use rust-analyzer
    let workspace = cargo::core::Workspace::new(path.as_path(), &config).unwrap();
    for package in workspace.members() {
        if package.manifest().metadata().license.is_none() {
            let cargo_toml_str = std::fs::read_to_string(package.manifest_path()).unwrap();
            let mut cargo_toml = toml::de::from_str::<toml::value::Table>(&cargo_toml_str).unwrap();
            let mut value = toml::value::Map::new();
            value.insert(
                "license".to_string(),
                toml::Value::String(LICENSE_NAME.to_string()),
            );
            _ = cargo_toml["package"].as_table_mut().insert(&mut value);
            std::fs::write(
                package.manifest_path(),
                toml::ser::to_string_pretty(&cargo_toml).unwrap(),
            )
            .unwrap();
        }
    }
}

#[cfg(not)]
#[test]
fn run_add_workspace_license() {
    add_workspace_license()
}
