use super::get_extensions_config;
const EXTENSION_DOWNLOAD_URL: &str = "http://baihai.cn-bj.ufileos.com/docker-build/extension-store/extensions_config.json";
const US3CLI_DEST: &str = "/home/ray/us3cli-linux64";

pub async fn get_extension() {
    loop {
        let extension_path = business::path_tool::recommended_extensions();
        let extension_path = extension_path.join("extensions_config.json");
        let extension_resp = match get_extensions_config(&extension_path).await{
            Ok(data) => data,
            Err(err) => {tracing::error!("{}", err.to_string());continue},
        };

        let parent_path = match extension_path.parent(){
            Some(data) => data,
            None => {tracing::error!("wrong extension path");continue},
        };

        let dest_path = parent_path.join("extension_temp.json");
        //curl -L -o /home/ray/extension-store/
        let mut cmd = tokio::process::Command::new("curl");
        cmd.arg("-L")
            .arg("-o")
            .arg(&dest_path)
            .arg(EXTENSION_DOWNLOAD_URL);
        let extension_resp_new = match cmd.spawn().expect("can't get current extension_config").wait().await{
            Ok(_) => {
                match get_extensions_config(&dest_path).await{
                    Ok(extension_data) => extension_data,
                    Err(err) => {
                        tracing::error!("{}", err.to_string());continue
                    },
                }
            },
            Err(err) =>{
                tracing::error!("{}", err.to_string());continue
            }
        };

        let mut resp_new_iter = extension_resp_new.iter();
        let mut resp_iter = extension_resp.iter();
        let mut resp_new = resp_new_iter.next();
        let mut resp = resp_iter.next();
        while(resp_new_iter.next() != None)||(resp_iter.next() != None){

            tracing::info!("resp_new: {:#?}",resp_new);
            tracing::info!("resp: {:#?}",resp);
            if resp.is_none() || resp_new.cmp(&resp).is_lt(){
                tracing::info!("get_remote_extension");
                let origin_name = match resp_new{
                    Some(data) => &data.name,
                    None => "",
                };
                get_remote_extension(origin_name).await;
                resp_new = resp_new_iter.next();
            }else if resp_new.is_none() || resp_new.cmp(&resp).is_gt(){
                tracing::info!("remove extension");
                let origin_name = match resp_new{
                    Some(data) => &data.name,
                    None => "",
                };
                let remove_path = format!("{}/{}",EXTENSION_DOWNLOAD_URL,origin_name);
                match tokio::fs::remove_dir_all(&remove_path).await{
                    Ok(_) => tracing::info!("successful remove extension: {:#?}",remove_path),
                    Err(err) => tracing::error!("fail to cp folder: {:#?},err:{:#?}",remove_path,err),
                };
                resp = resp_iter.next();
            }else {
                resp_new = resp_new_iter.next();
                resp = resp_iter.next();
            };
            
        }
        tracing::info!("over");
        if tokio::fs::copy(&dest_path, &extension_path).await.is_err(){
            tracing::error!("fail to overwrite past extension_config");
        };

        tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    }
}

pub async fn get_remote_extension(name:&str){
    let extension_path = business::path_tool::recommended_extensions();
    let origin_path = format!("{}/{}",EXTENSION_DOWNLOAD_URL,name);
    let mut cmd = tokio::process::Command::new(US3CLI_DEST);
    cmd.arg("cp")
        .arg("-r")
        .arg(&origin_path)
        .arg(&extension_path);
    match cmd.spawn().expect("can't get current extension_config").wait().await{
        Ok(_) => tracing::info!("successful cp folder: {:#?} to destpath",origin_path),
        Err(_) => tracing::info!("fail to cp folder: {:#?}",origin_path),
    }
}

