use business::business_term::ProjectId;
use business::business_term::TeamId;
use common_model::Rsp;
use tokio::time;

use crate::common::error::IdpGlobalError;

pub async fn start_hpopt_backend(
    db_url: String,
    team_id: TeamId,
    project_id: ProjectId,
) -> Result<Rsp<()>, IdpGlobalError> {
    // get python bin path from team_id,project_id
    // let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
    // let py_path = business::path_tool::get_conda_env_python_path(team_id, conda_env_name);
    //dashboard bin path /Users/huangjin/miniconda3/bin/optuna-dashboard
    let dashboard_bin_path = business::path_tool::get_optuna_dashboard_bin_path();
    println!("dashboard_bin_path: {}", dashboard_bin_path);

    // get a random unused TCP port.
    if let Some(port) = get_available_port() {
        // start optuna-dashboard
        let start = std::time::Instant::now();
        let child_opt = tokio::process::Command::new(dashboard_bin_path)
            .arg(db_url)
            .arg("--port")
            .arg(port.to_string())
            .spawn();
        // tracing::debug!(
        //     "--- start_hpopt_backend after optuna-dashboard command, time cost = {:?}",
        //     start.elapsed()
        // );

        if child_opt.is_err() {
            //print log on console
            // tracing::error!("{stderr}");
            // TODO need defind code and err msg.
            return Err(IdpGlobalError::NoteError("undefind error".to_string()));
        }
    }

    Ok(Rsp::success(()))
}
fn port_is_available(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}
fn get_available_port() -> Option<u16> {
    (10000..49151).find(|port| port_is_available(*port))
}
pub fn get_dburl_by_db_file_name(
    team_id: TeamId,
    project_id: ProjectId,
    db_file_name: &str,
) -> String {
    let db_file_fullpath =
        business::path_tool::get_hpopt_db_fullpath(team_id, project_id, db_file_name);
    let db_url = db_rul(db_file_fullpath);

    db_url
}
#[inline]
fn db_rul(db_file_name: String) -> String {
    format!("sqlite:///{}", db_file_name)
}

#[tokio::test]
async fn test_start_hpopt_backend() {
    let db_file_name = "111.db";
    let team_id = 19980923;
    let project_id = 1001;
    let db_url = get_dburl_by_db_file_name(team_id, project_id, db_file_name);
    println!("db_url: {}", db_url);
    let rsp = start_hpopt_backend(db_url, team_id, project_id).await;
    println!("{:?}", rsp);
    time::sleep(time::Duration::from_secs(10)).await;
    assert!(rsp.is_ok());
}
