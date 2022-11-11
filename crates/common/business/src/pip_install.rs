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

use std::path::Path;

use walkdir::WalkDir;

pub fn ensure_pth_file_exist(py_path: &String, install_dir: &String) -> Result<String, String> {
    let path = Path::new(py_path);
    let p1 = path
        .parent()
        .map_or(Err(format!("get {} parent failed", path.display())), |x| {
            Ok(x)
        })?;
    let p2 = p1
        .parent()
        .map_or(Err(format!("get {} parent 2 failed", path.display())), Ok)?;
    let aim_dir = p2.join("lib");

    if aim_dir.is_dir() {
        for entry in WalkDir::new(aim_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.starts_with("python") {
                let aim = entry.path().join("site-packages/0_idp.pth");
                if aim.exists() {
                    return Ok(aim.to_string_lossy().to_string());
                } else {
                    let p1 = aim
                        .parent()
                        .map_or(Err(format!("get {} parent failed", aim.display())), |x| {
                            Ok(x)
                        })?;
                    if !p1.exists() {
                        tracing::error!(
                            "python site-packages dir {} not exist, exit",
                            p1.display()
                        );
                        return Err(format!(
                            "python site-package dir {} not exist",
                            p1.display()
                        ));
                    } else {
                        return write_idp_pth_file(aim, install_dir);
                    }
                }
            }
        }
        // traval all file, not found python
        tracing::error!("not found python lib/pythonxxx dir");
        Err("not found python lib/pythonxxxx dir".to_string())
    } else {
        tracing::error!("not found python lib dir");
        Err("not found python lib dir".to_string())
    }
}

pub fn ensure_python2user_install_dir_exist(py_path: &String) -> Result<String, String> {
    let path = Path::new(py_path);
    let p1 = path
        .parent()
        .map_or(Err(format!("get {} parent failed", path.display())), |x| {
            Ok(x)
        })?;
    let p2 = p1
        .parent()
        .map_or(Err(format!("get {} parent 2 failed", path.display())), Ok)?;
    let aim = p2.join("pm_installed");
    std::fs::create_dir_all(&aim)
        .map_or(Err(format!("create dir {} failed", aim.display())), |_| {
            Ok("")
        })?;
    aim.into_os_string()
        .into_string()
        .map_or(Err("trans path XXX to string failed".to_string()), Ok)
}

fn write_idp_pth_file(aim: std::path::PathBuf, install_dir: &String) -> Result<String, String> {
    std::fs::write(&aim, install_dir)
        .map_or(Err(format!("write to {} failed", aim.display())), |_| {
            Ok(format!("{}", aim.display()))
        })
}
