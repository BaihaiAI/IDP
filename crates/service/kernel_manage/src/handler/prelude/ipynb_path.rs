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

use super::Body;
use super::Deserialize;
use super::Inode;
use super::Request;

// team id from cookie
// ipynb path old arch: /store/idp-note/projects/$project_id/notebooks/$path
// ipynb path new arch: /store/$team_id/idp-note/projects/$project_id/notebooks/$path

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
// #[serde(rename_all = "camelCase")]
struct InodeReq {
    inode: Inode,
}

pub fn inode_from_query_string(req: Request<Body>) -> Result<Inode, err::ErrorTrace> {
    Ok(serde_urlencoded::from_str::<InodeReq>(req.uri().query().unwrap_or_default())?.inode)
}

// 1. read env by project_id from redis
// 2. read env by conda env file
// 3. default env is python39
