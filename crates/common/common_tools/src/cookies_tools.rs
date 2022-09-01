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

use tower_cookies::Cookies;

fn get_cookie_value_by_name(cookies: &Cookies, name: &str) -> String {
    let cookie_opt = cookies.get(name);
    if let Some(cookie) = cookie_opt {
        let value = cookie.value();
        tracing::debug!("-->cookie name = {:?}, value = {:?} ", cookie.name(), value);
        value.to_string()
    } else {
        tracing::warn!("cookie key {name} not found");
        String::default()
    }
}

pub fn get_cookie_value_by_team_id(cookies: Cookies) -> u64 {
    let team_id_string = get_cookie_value_by_name(&cookies, "teamId");
    if !team_id_string.is_empty() {
        match team_id_string.parse::<u64>() {
            Ok(team_id_ret) => team_id_ret, // Err(err) => Err(ErrorTrace::NoteError(err.to_string())),
            Err(_err) => 0,
        }
    } else {
        tracing::warn!("cookie teamId not found");
        0u64
    }
}

#[cfg(not)]
pub fn get_cookie_value_region(cookies: Cookies) -> String {
    let region = get_cookie_value_by_name(&cookies, "region");
    if !region.is_empty() {
        region
    } else {
        tracing::warn!("cookie region not found");
        "a".to_string()
    }
}
