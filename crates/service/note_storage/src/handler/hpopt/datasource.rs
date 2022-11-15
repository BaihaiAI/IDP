use business::business_term::{TeamId, ProjectId};

///
/// /store/{team_id}/projects/project_id/hp[opt_datasource]
pub async fn datasource_list(team_id:TeamId,project_id:ProjectId) -> Vec<String> {
    // get datasource dir path
    let datasource_path = business::path_tool::get_hpopt_datasource_path(team_id, project_id);
    // create file struct by path and get all file name.
    let mut datasource_list = Vec::new();
    if let Ok(dir) = std::fs::read_dir(datasource_path.clone()) {
        dir.for_each(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            datasource_list.push(file_name.to_string());
                        }
                    }
                }
            }
        });
    }else{
        // this dir not exist, create it.
        // print log on console. todo:need change to log crate.
        println!("datasource dir not exist, create it. path: {}",datasource_path);
        // log::info!("datasource dir not exist, create it. path: {}",datasource_path);
        std::fs::create_dir_all(datasource_path).unwrap();
    }
    datasource_list
}

#[cfg(not)]
#[tokio::test]
async fn test_datasource_list() {
    let team_id = 19980923;
    let project_id = 1001;
    let datasource_list = datasource_list(team_id, project_id).await;
    println!("{:?}", datasource_list);
}