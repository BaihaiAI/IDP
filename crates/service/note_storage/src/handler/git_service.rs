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

use crate::api_model::project::GitInfoObj;
use crate::api_model::project::ProjectRet;
use crate::handler::project_handler::parse_return_success_code;
/// Copyright @baihai 2021
/// User: Zhang Qike
/// Date: 2022/4/14
/// Time: 16:07
/// To change this template use Preferences | Editor | File and Code Templates | Rust File

const PROTOCOLS_HTTPS: &str = "https://";
const PROTOCOLS_OAUTH2: &str = "oauth2";

static FULL_PROJECT_DELETE_URL: &str = "http://idp-resource-svc:10005/api/v1/project/delete";

pub async fn git_clone(
    url: String,
    git_info_obj: GitInfoObj,
    target_path: String,
    project_id: u64,
) -> Result<(), err::ErrorTrace> {
    tracing::info!("git_clone target_path= {:?}", target_path);

    if !git_info_obj.token.as_ref().unwrap().is_empty() {
        tracing::info!("git_info_obj.token is not none");
        run_git_clone(
            make_git_url_by_token(url.as_str(), git_info_obj.token.unwrap().as_str()),
            target_path,
            project_id.to_string(),
        )
        .await?
    } else if !git_info_obj.username.as_ref().unwrap().is_empty()
        && !git_info_obj.password.as_ref().unwrap().is_empty()
    {
        tracing::info!("git_info_obj.username, password is not none");
        run_git_clone(
            make_git_url_by_user_info(
                url.as_str(),
                git_info_obj.username.unwrap().as_str(),
                git_info_obj.password.unwrap().as_str(),
            ),
            target_path,
            project_id.to_string(),
        )
        .await?
    } else {
        tracing::info!("git_info_obj.username, password, token all are none ");
        run_git_clone(url, target_path, project_id.to_string()).await?
    }
    Ok(())
}

pub fn make_git_url_by_token<'a>(url: &'a str, token: &'a str) -> String {
    let last = format!(
        "{}{}{}{}{}{}",
        PROTOCOLS_HTTPS,
        PROTOCOLS_OAUTH2,
        ":",
        token,
        "@",
        split_url(url)
    );
    last
}

pub fn make_git_url_by_user_info(url: &str, username: &str, password: &str) -> String {
    let last = format!(
        "{}{}{}{}{}{}",
        PROTOCOLS_HTTPS,
        username,
        ":",
        password,
        "@",
        split_url(url)
    );
    last
}

pub fn split_url(url: &str) -> &str {
    url.split_once("//").unwrap().1
}

pub async fn run_git_clone(
    url: String,
    target_path: String,
    id: String,
) -> Result<(), err::ErrorTrace> {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone").arg(url).arg(target_path);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("{stderr}");

        let project_ret = reqwest::Client::new()
            .post(FULL_PROJECT_DELETE_URL)
            .json(&serde_json::json!({ "id": id }))
            .send()
            .await?
            .json::<ProjectRet>()
            .await?;
        tracing::debug!(
            "--> delete_project_from_db_by_api project_ret={:?}",
            project_ret
        );
        if !parse_return_success_code(project_ret.code) {
            tracing::debug!(
                "--> parse_return_success_code error ={:?} {:?}",
                project_ret.code,
                project_ret.clone().message.unwrap()
            );
            return Err(err::ErrorTrace::new(&project_ret.message.unwrap()).code(project_ret.code));
        }
        return Err(err::ErrorTrace::new("error on git clone"));
    }
    Ok(())
}
