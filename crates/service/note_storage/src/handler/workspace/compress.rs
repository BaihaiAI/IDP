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

use axum::Json;
use business::path_tool;
use common_model::Rsp;
use err::ErrorTrace;
use tracing::info;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspacePathRto {
    pub team_id: String,
    pub project_id: u64,
    pub path: String,
}

pub async fn dir_zip(Json(payload): Json<WorkspacePathRto>) -> impl axum::response::IntoResponse {
    info!("access workspace dir_zip api");

    let (path, project_id, team_id) = (payload.path, payload.project_id, payload.team_id);
    info!("path: {:?}", path);
    info!("project_id: {:?}", project_id);
    tracing::info!("team_id: {:?}", team_id);

    dir_zip_(path, team_id, project_id).await
}

#[tracing::instrument]
async fn dir_zip_(path: String, team_id: String, project_id: u64) -> Result<Rsp<()>, ErrorTrace> {
    let team_id = team_id.parse::<u64>()?;
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let mut abs_export_path = base_path.clone();
    abs_export_path.push(crate::business_::path_tool::get_relative_path(Path::new(
        &path,
    )));
    info!("export_path: {:?}", abs_export_path);

    let generate_filename = abs_export_path.file_stem().unwrap();
    let mut generate_filename = Path::new(generate_filename).with_extension("zip");
    let file_name = abs_export_path.file_name().unwrap();

    let (_, export_path_start) = path.split_at(1);
    info!("export_path_start:{}", export_path_start);

    let parent_path = &abs_export_path.parent().unwrap();
    info!("parent_path:{:?}", parent_path);

    info!("generate_filename:{:?}", generate_filename);
    info!("file_name:{:?}", file_name);

    let path = &parent_path.join(&generate_filename);
    if path.exists() {
        generate_filename = super::decompress::rename_path_if_path_exist(path.clone());
    }

    info!("abs_path->{:#?}", abs_export_path);
    info!("parent_path-->{:#?}", parent_path);
    info!("generate_filename-->{:#?}", generate_filename);
    info!("file_name-->{:#?}", file_name);

    info!("!!!!base_path: {:?}", base_path);

    let output = if abs_export_path.is_dir() {
        let is_empty = abs_export_path.read_dir()?.next().is_none();
        if is_empty {
            return Err(ErrorTrace::new("directory is null"));
        }
        let zip_full_path = &parent_path.join(&generate_filename);
        let mut cmd = tokio::process::Command::new("zip");
        cmd.current_dir(&abs_export_path)
            .arg("-q")
            .arg("-r")
            .arg(&zip_full_path)
            .arg(".")
            .arg("-i")
            .arg("*");
        tracing::info!("cmd = {cmd:?}");
        cmd.output().await?
    } else {
        //std: "zip" "-q" "-r" "demo.zip" "demo"
        let mut cmd = tokio::process::Command::new("zip");
        cmd.current_dir(parent_path)
            .arg("-q")
            .arg("-r")
            .arg(&generate_filename)
            .arg(file_name);
        tracing::info!("cmd = {cmd:?}");
        cmd.output().await?
    };

    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }

    Ok(Rsp::success(()))
}
