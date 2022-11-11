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
use std::path::PathBuf;

pub fn name_convert(path_str: String, replacement: String) -> String {
    let re = regex::Regex::new(".[^.]+$").unwrap();
    format!("{}.{}", re.replace(path_str.as_str(), ""), replacement)
}

/*
## Why need this function
frontend input: "/a/b/c.txt"
need convert to: "a/b/c.txt"

running 1 test
[crates\common\business\src\path_tool.rs:199] pat.join("/") = "C:/"
[crates\common\business\src\path_tool.rs:200] pat.join("") = "C:\\Users\\Finn\\Downloads\\ttt\\"
[crates\common\business\src\path_tool.rs:201] pat.join("").join("") = "C:\\Users\\Finn\\Downloads\\ttt\\"
*/
pub fn get_relative_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    #[cfg(unix)]
    {
        if path.is_absolute() {
            return path.strip_prefix("/").unwrap().to_path_buf();
        }
        path.to_path_buf()
    }
    #[cfg(windows)]
    {
        if path.to_str().unwrap() == "/" {
            return Path::new("").to_path_buf();
        }
        path.strip_prefix("/").unwrap().to_path_buf()
    }
}

pub fn project_conda_env_file_path(team_id: u64, project_id: u64) -> String {
    format!(
        "{}/miniconda3/conda.env",
        business::path_tool::project_root(team_id, project_id)
    )
}
