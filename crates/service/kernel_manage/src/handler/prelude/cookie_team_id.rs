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

use hyper::Body;
use hyper::Request;
use kernel_common::typedef::TeamId;

use crate::Error;

pub fn team_id_from_cookie(req: &Request<Body>) -> Result<u64, Error> {
    let cookie = req.headers().get(hyper::header::COOKIE);
    let cookie = if let Some(cookie) = cookie {
        cookie
    } else {
        return Err(Error::new("no cookie").code(500));
    };
    parse_team_id_from_cookies(cookie.to_str()?)
}

fn parse_team_id_from_cookies(cookies: &str) -> Result<TeamId, Error> {
    let mut user_id_opt = None;
    for cookie in cookies.split(';') {
        let cookie = cookie.trim_start();
        match cookie.split_once('=') {
            Some((k, v)) => {
                if k == "teamId" {
                    user_id_opt = Some(v);
                    break;
                }
            }
            None => return Err(Error::new("no teamId key in cookie")),
        }
    }
    let user_id = match user_id_opt {
        Some(user_id) => user_id,
        None => return Err(Error::new("invalid cookie")),
    };
    Ok(user_id.parse::<TeamId>()?)
}
