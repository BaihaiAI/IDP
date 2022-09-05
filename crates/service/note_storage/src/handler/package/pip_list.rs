use axum::extract::Query;
use common_model::Rsp;

use crate::api_model::TeamIdProjectIdQueryString;

#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub package_name: String,
    pub version: String,
}

pub async fn pip_list(
    Query(TeamIdProjectIdQueryString {
        project_id,
        team_id,
    }): Query<TeamIdProjectIdQueryString>,
) -> Result<Rsp<Vec<PackageInfo>>, err::ErrorTrace> {
    let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
    let py_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);
    let package_list = pip_list_(py_path).await?;

    Ok(Rsp::success(package_list))
}

#[tracing::instrument]
pub async fn pip_list_(py_path: String) -> std::io::Result<Vec<PackageInfo>> {
    tracing::info!("--> pip_list");
    let start = std::time::Instant::now();
    let output = tokio::process::Command::new(py_path)
        .arg("-m")
        .arg("pip")
        .arg("list")
        .arg("--format")
        .arg("freeze")
        .output()
        .await?;
    tracing::debug!(
        "--- pip_list after pip list command, time cost = {:?}",
        start.elapsed()
    );

    if !output.status.success() {
        let stderr = unsafe { String::from_utf8_unchecked(output.stderr) };
        tracing::error!("{stderr}");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("pip list error: {}", stderr),
        ));
    }

    let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };

    let mut pip_list = vec![];

    for line_str in stdout.lines() {
        let (package_name, version) = match line_str.split_once("==") {
            // if split_once failed, it means that line_str is not a package line,break;
            // e.g. could not fetch URL https://pypi.douban.org
            None => {
                break;
            }
            Some((package_name, version)) => (package_name, version),
        };
        pip_list.push(PackageInfo {
            package_name: package_name.to_string(),
            version: version.to_string(),
        });
    }
    tracing::debug!("<-- pip_list, timing = {:?}", start.elapsed());
    Ok(pip_list)
}
