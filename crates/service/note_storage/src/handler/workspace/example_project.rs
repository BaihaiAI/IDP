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

use std::path::PathBuf;

use business::path_tool;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;

#[derive(Debug, serde::Serialize)]
pub struct UpdateReq {
    pub version: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateDto {
    pub code: u32,
}

pub async fn example_project(
    team_id: u64,
    project_id: u64,
    version: &str,
) -> Result<Rsp<()>, ErrorTrace> {
    tracing::info!("example_project api run...");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    tracing::info!(
        "team_id: {:?},project_id: {:?},version: {:?}",
        team_id,
        project_id,
        version
    );

    let client = reqwest::ClientBuilder::new().build().unwrap();
    let update_dto = client
        .get("http://idp-admin-rs-svc:9092/api/v1/admin-rs/example_project/compare-version")
        .query(&UpdateReq {
            version: version.to_string(),
        })
        .send()
        .await?
        .json::<UpdateDto>()
        .await?;

    if update_dto.code != 21_000_200 {
        return Ok(Rsp {
            data: (),
            code: update_dto.code,
            message: "The sample project is up to date".to_string(),
        });
    }

    tokio::spawn(async {
        if let Err(err) = update_example_project(base_path).await {
            tracing::error!("{err:#?}");
        }
    });

    Ok(Rsp {
        data: (),
        code: update_dto.code,
        message: "Sample project update in progress".to_string(),
    })
}

pub async fn update_example_project(base_path: PathBuf) -> Result<(), ErrorTrace> {
    // rm -rf /store/tmp/update
    let store_tmp = business::path_tool::store_parent_dir()
        .join("store")
        .join("tmp");
    let store_tmp_update = store_tmp.join("update");
    if let Err(err) = tokio::fs::remove_file(&store_tmp_update).await {
        tracing::warn!("{err}");
    }

    let url = "http://idp-admin-rs-svc:9092/api/v1/admin-rs/example_project/update";
    let mut cmd = tokio::process::Command::new("curl");
    cmd.arg(url).arg("-o").arg(&store_tmp_update);
    tracing::debug!("cmd = {cmd:?}");
    cmd.spawn()?.wait().await?;
    let file_path = base_path.clone();

    let file_path = file_path.join(include_str!("example_project_repo_name.txt"));
    if let Err(err) = tokio::fs::remove_dir_all(file_path).await {
        tracing::warn!("{err}");
    }

    // tar -zxf /store/tmp/update -C /store/1546774368495616000/projects/109/notebooks
    let mut cmd = tokio::process::Command::new("tar");
    cmd.arg("-zxf")
        .arg(store_tmp_update)
        .arg("-C")
        .arg(base_path);
    tracing::debug!("cmd = {cmd:?}");
    cmd.spawn()?.wait().await?;

    Ok(())
}
