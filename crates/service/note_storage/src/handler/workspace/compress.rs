use std::path::Path;

use axum::Json;
use business::path_tool;
use common_model::Rsp;
use err::ErrorTrace;
use tracing::info;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspacePathRto {
    pub path: String,
    pub project_id: u64,
    pub team_id: String,
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
    if abs_export_path.is_dir() {
        let is_empty = abs_export_path.read_dir()?.next().is_none();
        if is_empty {
            return Err(ErrorTrace::new("directory is null"));
        }
    }
    let (_, export_path_start) = path.split_at(1);
    info!("export_path_start:{}", export_path_start);

    let abs_export_path_str = abs_export_path
        .to_str()
        .ok_or(ErrorTrace::new("path to str err"))?;
    info!("abs_export_path_str: {:?}", abs_export_path_str);

    let parent_path = &abs_export_path.parent().unwrap();
    info!("parent_path:{:?}", parent_path);

    info!("generate_filename:{:?}", generate_filename);
    info!("file_name:{:?}", file_name);

    let path = &parent_path.join(&generate_filename);
    if path.exists() {
        let file = generate_filename.file_stem().unwrap().to_string_lossy();
        let mut num = 1;
        loop {
            let a = format!("{}/{file}({num}).zip", parent_path.to_string_lossy());
            if std::path::Path::new(&a).exists() {
                num += 1;
                continue;
            }
            generate_filename = std::path::Path::new(&a).to_path_buf();
            break;
        }
    }

    info!("parent_path-->{:#?}", parent_path);
    info!("generate_filename-->{:#?}", generate_filename);
    info!("file_name-->{:#?}", file_name);

    info!("!!!!base_path: {:?}", base_path);
    //std: "zip" "-q" "-r" "demo.zip" "demo"
    let mut cmd = tokio::process::Command::new("zip");
    cmd.current_dir(parent_path)
        .arg("-q")
        .arg("-r")
        .arg(&generate_filename)
        .arg(file_name);
    tracing::info!("cmd = {cmd:?}");
    let output = cmd.output().await?;
    if !output.status.success() {
        return Err(ErrorTrace::new(&String::from_utf8_lossy(&output.stderr)));
    }

    Ok(Rsp::success(()))
}
