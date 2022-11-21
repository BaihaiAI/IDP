use business::business_term::ProjectId;
use business::business_term::TeamId;

use crate::common::error::IdpGlobalError;

pub async fn stop_hpopt_backend(db_url: String) -> Result<(), IdpGlobalError> {
    let backend_pid = get_pid_by_name(&db_url).await?;
    println!("backend_pid: {}", backend_pid);
    if backend_pid > 0 {
        println!("kill backend_pid: {}", backend_pid);
        let _ = tokio::process::Command::new("kill")
            .arg("-9")
            .arg(backend_pid.to_string())
            .output()
            .await?;
    }
    Ok(())
}

async fn get_pid_by_name(db_url: &str) -> Result<u32, IdpGlobalError> {
    let output = tokio::process::Command::new("ps")
        .arg("-ef")
        .output()
        .await
        .map_err(|e| IdpGlobalError::ErrorCodeMsg(500, format!("get_pid_by_name error: {}", e)))?;

    let output_str = String::from_utf8(output.stdout).unwrap();
    let lines = output_str.lines();
    for line in lines {
        if line.contains(db_url) {
            let pid = line.split_whitespace().nth(1).unwrap();
            return Ok(pid.parse::<u32>().unwrap());
        }
    }

    Err(IdpGlobalError::ErrorCodeMsg(
        500,
        "get_pid_by_name error: not found".to_string(),
    ))
}

pub async fn start_hpopt_backend(
    db_url: String,
    _team_id: TeamId,
    _project_id: ProjectId,
) -> Result<u16, IdpGlobalError> {
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
        println!(
            "--- start_hpopt_backend after optuna-dashboard command, time cost = {:?}",
            start.elapsed()
        );

        if child_opt.is_err() {
            //print log on console
            // tracing::error!("{stderr}");
            // TODO need defind code and err msg.
            return Err(IdpGlobalError::NoteError("undefind error".to_string()));
        }
        return Ok(port);
    }
    Err(IdpGlobalError::NoteError("no available port.".to_string()))
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
    

    db_rul(db_file_fullpath)
}

#[inline]
#[cfg(not)]
pub fn get_db_full_file_name_by_dburl(db_url: &str) -> String {
    //substring by prefix "sqlite:///"
    let db_file_fullpath = &db_url[10..];
    db_file_fullpath.to_string()
}

#[inline]
fn db_rul(db_file_name: String) -> String {
    format!("sqlite:///{}", db_file_name)
}

#[cfg(test)]
mod control_tests {
    use crate::handler::hpopt::control::get_dburl_by_db_file_name;
    use crate::handler::hpopt::control::start_hpopt_backend;
    use crate::handler::hpopt::control::stop_hpopt_backend;
    #[tokio::test]
    // #[cfg(not)]
    async fn test_start_hpopt_backend() {
        let db_file_name = "111.db";
        let team_id = 19980923;
        let project_id = 1001;
        let db_url = get_dburl_by_db_file_name(team_id, project_id, db_file_name);
        println!("db_url: {}", db_url);
        let rsp = start_hpopt_backend(db_url, team_id, project_id).await;
        println!("{:?}", rsp);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        assert!(rsp.is_ok());
    }

    fn get_db_full_file_name_by_dburl(db_url: &str) -> String {
        //substring by prefix "sqlite:///"
        let db_file_fullpath = &db_url[10..];
        db_file_fullpath.to_string()
    }

    #[tokio::test]
    async fn test_get_file_name_by_dburl() {
        let db_url = "sqlite:////store/19980923/projects/1001/hpopt_datasource/111.db";
        let file_name = get_db_full_file_name_by_dburl(db_url);
        println!("file_name: {}", file_name);
        // assert_eq!(file_name, "111.db");
    }
    #[tokio::test]
    async fn test_stop_hpopt_backend() {
        let db_url = "sqlite:////store/19980923/projects/1001/hpopt_datasource/111.db";
        let rsp = stop_hpopt_backend(db_url.to_string()).await;
        println!("{:?}", rsp);
        assert!(rsp.is_ok());
    }
}
