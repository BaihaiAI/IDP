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



use crate::business_term::TeamId;

pub fn team_id_to_user_name(team_id: TeamId) -> String {
    format!("k{}", team_id_to_uid(team_id))
}

pub const fn team_id_to_uid(team_id: TeamId) -> libc::uid_t {
    let ret = (team_id % 10_0000) / 2 + 10000;
    ret as libc::uid_t
}

#[test]
fn test_uid() {
    assert_eq!(team_id_to_uid(8383737173688), 46844);
    assert_eq!(team_id_to_uid(838376363663621), 41810);
    assert_eq!(team_id_to_uid(838376363613621), 16810);
}

pub fn contains_team_linux_user(team_id: TeamId) -> bool {
    use std::io::BufRead;

    let team_id_user_name = team_id_to_user_name(team_id);
    for line in std::io::BufReader::new(std::fs::File::open("/etc/passwd").unwrap())
        .lines()
        .flatten()
    {
        let username = line.split_once(':').unwrap().0;
        if username == team_id_user_name {
            return true;
        }
    }
    false
}

/// e.g. `useradd -u 12345 -d /home/k12345 k12345`
/// e.g. `runuser -u k12345 -- /bin/ls -a`
/// e.g. `su k12345 -s /bin/ls -- -a`
pub fn create_team_linux_user_if_not_exist(team_id: TeamId) {
    if contains_team_linux_user(team_id) {
        if cfg!(debug_assertions) {
            tracing::info!(
                "<-- create_team_linux_user_if_not_exist: team_linux_user already exist"
            );
        }
        return;
    }

    let uid = team_id_to_uid(team_id);
    let linux_user_name = team_id_to_user_name(team_id);
    // useradd is native binary compiled with the system. But, adduser is a perl script which uses useradd binary in back-end.
    let mut cmd = std::process::Command::new("/usr/sbin/useradd");
    cmd.arg("-u")
        .arg(uid.to_string())
        .arg(linux_user_name)
        .arg("--create-home")
        .stdin(std::process::Stdio::null());
    let output = cmd.output().unwrap();
    if cfg!(debug_assertions) {
        tracing::info!("cmd = {cmd:?}");
        tracing::info!("output = {output:#?}");
    }
    debug_assert!(output.status.success());
}
