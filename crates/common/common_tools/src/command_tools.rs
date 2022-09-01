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

use err::ErrorTrace;

#[cfg(unix)]
pub fn copy(from_path_str: &str, to_path_str: &str) -> Result<(), ErrorTrace> {
    let mut cmd = std::process::Command::new("cp");
    cmd.arg("-r").arg(from_path_str).arg(to_path_str);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output().unwrap();
    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

#[cfg(windows)]
pub fn copy(from_path_str: &str, to_path_str: &str) -> Result<(), ErrorTrace> {
    let options = fs_extra::dir::CopyOptions {
        copy_inside: true,
        ..Default::default()
    };
    if let Err(err) = fs_extra::dir::copy(from_path_str, to_path_str, &options) {
        return Err(ErrorTrace::new(&err.to_string()));
    };
    Ok(())
}
