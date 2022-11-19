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

pub type Cookies = axum::headers::Cookie;

pub fn get_cookie_value_by_key(cookies: &Cookies, key: &str) -> String {
    let cookie_opt = cookies.get(key);
    if let Some(val) = cookie_opt {
        tracing::debug!("-->cookie key = {:?}, value = {:?} ", key, val);
        val.to_string()
    } else {
        tracing::warn!("cookie key {key} not found");
        String::default()
    }
}

pub fn get_cookie_value_by_team_id(cookies: Cookies) -> u64 {
    let team_id_string = get_cookie_value_by_key(&cookies, "teamId");
    if team_id_string.is_empty() {
        tracing::warn!("cookie teamId not found");
        0u64
    } else {
        match team_id_string.parse::<u64>() {
            Ok(team_id_ret) => team_id_ret, // Err(err) => Err(ErrorTrace::NoteError(err.to_string())),
            Err(_err) => 0,
        }
    }
}
