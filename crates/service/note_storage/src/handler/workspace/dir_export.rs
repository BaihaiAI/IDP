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

use axum::response::IntoResponse;
use axum::Json;
use business::path_tool;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use err::ErrorTrace;
use tracing::info;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDirExport {
    pub export_path: String,
    pub project_id: u64,
    pub project_name: Option<String>,
}

pub async fn dir_export(
    axum::TypedHeader(cookies): axum::TypedHeader<common_tools::cookies_tools::Cookies>,
    Json(payload): Json<WorkspaceDirExport>,
) -> impl IntoResponse {
    info!("access dir_export_test api");

    let (export_path, project_id, project_name_opt) = (
        payload.export_path,
        payload.project_id,
        payload.project_name,
    );
    info!("export_path: {:?}", export_path);
    info!("project_id: {:?}", project_id);
    info!("project_name_opt: {:?}", project_name_opt);

    let team_id = get_cookie_value_by_team_id(cookies);
    dir_export_(export_path, team_id, project_id, project_name_opt).await
}

#[tracing::instrument]
async fn dir_export_(
    export_path: String,
    team_id: u64,
    project_id: u64,
    project_name_opt: Option<String>,
) -> impl IntoResponse {
    info!("access dir_export_test function .......");
    let base_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    info!("base_path: {:?}", base_path);

    let tmp_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::TMP,
    );
    info!("tmp_path: {:?}", tmp_path);

    let mut abs_export_path = base_path.clone();
    abs_export_path.push(crate::business_::path_tool::get_relative_path(
        std::path::Path::new(&export_path),
    ));
    info!("export_path: {:?}", abs_export_path);

    let mut generate_filename;
    let download_file_name;
    if export_path.eq_ignore_ascii_case("/") {
        generate_filename = tmp_path.to_str().unwrap().to_string() + "/notebook.zip";
        info!("root export, generate_filename: {:?}", generate_filename);
        let project_path = path_tool::project_root(team_id, project_id);
        info!("root export, project_path: {:?}", project_path);

        if let Some(project_name) = project_name_opt {
            download_file_name = project_name;
        } else {
            download_file_name = "notebook".to_string();
        }
        info!("root export, download_file_name:{}", download_file_name);

        let export_path_start = "notebooks".to_string();
        let exclude_export_path = "notebooks/storage-service/*".to_string();
        info!("root export, exclude_export_path:{}", exclude_export_path);

        //must be "$exclude_export_path" it works well. $exclude_export_path does not work
        let mut cmd = std::process::Command::new("zip");
        cmd.current_dir(project_path)
            .arg("-q")
            .arg("-r")
            .arg(&generate_filename)
            .arg(export_path_start)
            .arg("-x")
            .arg(exclude_export_path);
        tracing::info!("cmd = {cmd:?}");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        }
    } else {
        let is_dir = abs_export_path.is_dir();
        if !is_dir {
            return Err(ErrorTrace::new("dir not exist"));
        }

        let pos = export_path.rfind('/').unwrap();
        let (_, file_name) = export_path.split_at(pos + 1);
        info!("file_name:{}", file_name);
        download_file_name = file_name.to_string();
        info!("download_file_name:{}", download_file_name);

        let (_, export_path_start) = export_path.split_at(1);
        info!("export_path_start:{}", export_path_start);

        generate_filename = tmp_path.display().to_string() + "/";
        let s2 = file_name.to_string();
        let ext = ".zip";
        generate_filename += &s2;
        generate_filename += ext;

        info!("generate_filename:{}", generate_filename);
        info!("file_name:{}", file_name);

        let org_export_path = abs_export_path.parent().unwrap();

        // (cd $org_export_path;zip -q -r $generate_filename $file_name) {
        let mut cmd = std::process::Command::new("zip");
        cmd.current_dir(&org_export_path)
            .arg("-q")
            .arg("-r")
            .arg(&generate_filename)
            .arg(file_name);
        tracing::info!("cmd = {cmd:#?}, current_dir = {org_export_path:?}");
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
        }
    }

    crate::handler::workspace::download_file(
        std::path::Path::new(&generate_filename).to_path_buf(),
        "application/zip",
    )
    .await
}
