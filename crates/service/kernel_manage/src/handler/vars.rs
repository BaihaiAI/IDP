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

use super::prelude::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Req {
    // #[serde(deserialize_with = "kernel_common::de_u64_from_str")]
    team_id: TeamId,
    project_id: ProjectId,
    file_path: String,
    // #[serde(deserialize_with = "kernel_common::de_u64_from_str")]
    // inode: u64,
}
pub fn vars(req: Request<Body>) -> Result<Resp<String>, Error> {
    let req = serde_urlencoded::from_str::<Req>(req.uri().query().unwrap_or_default())?;
    let vars_path =
        business::path_tool::vars_file_path(req.team_id, req.project_id, &req.file_path);
    let meta = match std::fs::metadata(&vars_path) {
        Ok(meta) => meta,
        Err(err) => {
            if !matches!(err.kind(), std::io::ErrorKind::NotFound) {
                tracing::error!("{vars_path} {err}");
            }
            return Ok(Resp::success("[]".to_string()));
        }
    };
    if meta.len() > 5 * 1024 * 1024 {
        return Err(Error::new("vars file too large skip loading").code(Error::CODE_WARNING));
    }

    Ok(Resp::success(std::fs::read_to_string(vars_path)?))
}
