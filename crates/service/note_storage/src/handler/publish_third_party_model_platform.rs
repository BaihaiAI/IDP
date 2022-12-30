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

use axum::extract::Json;
use common_model::Rsp;
use err::ErrorTrace;

use super::workspace::compress::WorkspacePathRto;

// TODO this API upload file to sftp, maybe 504 gateway timeout
pub async fn publish_model(Json(payload): Json<WorkspacePathRto>) -> Result<Rsp<()>, ErrorTrace> {
    // crate::business_::path_tool:
    let abs_path = business::path_tool::get_store_full_path(
        payload.team_id.parse().unwrap(),
        payload.project_id,
        payload.path,
    );
    if !abs_path.exists() {
        return Err(ErrorTrace::new("path not exist"));
    }

    let config_str = std::fs::read_to_string("/etc/publish_third_party_model_platform.toml")?;
    let config = toml::de::from_str::<ThirdPartyPlatformConfig>(&config_str)?;

    // step 1. upload file to sftp, get sftp path
    let config_ = config.clone();
    let sftp_path = tokio::task::spawn_blocking(move || {
        upload_file_to_sftp(abs_path.to_str().unwrap(), config_)
    })
    .await??;

    // step 2. request model notify API with sftp path
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    let api_base_url = config.api_base_url;
    let token_rsp = client
        .post(format!("{api_base_url}/token"))
        .json(&serde_json::json!({
            "apiKey": config.api_key,
            "apiSecret": config.api_secret
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    // assert!(token_rsp["code"], 0);
    let token = token_rsp["result"]["token"].as_str().unwrap();
    client
        .post(format!("{api_base_url}/token"))
        .header("Authorization", token)
        .json(&serde_json::json!({
            "code": config.api_code,
            "type": 3,
            "path": sftp_path,
            "tag": config.api_image_tag,
            "protocolType": 1
        }))
        .send()
        .await?;

    Ok(Rsp::success_without_data())
}

#[derive(serde::Deserialize, Clone)]
struct ThirdPartyPlatformConfig {
    sftp_username: String,
    sftp_password: String,
    sftp_host: String,
    api_base_url: String,
    api_key: String,
    api_secret: String,
    api_code: String,
    api_image_tag: String,
}

// TODO tokio spawn_blocking
/// assert input abs_path exist
fn upload_file_to_sftp(
    abs_path: &str,
    config: ThirdPartyPlatformConfig,
) -> Result<String, ErrorTrace> {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;

    // Connect to the local SSH server
    let tcp = TcpStream::connect(format!("{}:22", config.sftp_host))?;
    let mut sess = ssh2::Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    // openssh/openssh_sftp_client async ssh by Jon Gjengset not support password base auth
    // so we use alexcrichton's ssh2
    sess.userauth_password(&config.sftp_username, &config.sftp_password)?;
    let sftp = sess.sftp()?;

    let filename = std::path::Path::new(abs_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let mut file_content_to_upload = Vec::new();
    std::fs::File::open(abs_path)?.read_to_end(&mut file_content_to_upload)?;
    let mut f = sftp.create(std::path::Path::new(filename))?;
    f.write_all(&file_content_to_upload)?;
    let sftp_abs_path = sftp.realpath(std::path::Path::new(filename))?;
    Ok(sftp_abs_path.to_str().unwrap().to_string())
}

#[test]
#[ignore]
fn test_upload_file_to_sftp() {
    let config_str =
        std::fs::read_to_string("/etc/publish_third_party_model_platform.toml").unwrap();
    let config = toml::de::from_str::<ThirdPartyPlatformConfig>(&config_str).unwrap();
    dbg!(upload_file_to_sftp("/etc/os-release", config).unwrap());
}
