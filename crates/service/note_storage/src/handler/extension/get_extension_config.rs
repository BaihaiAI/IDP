use super::{get_extensions_config, models::ExtensionResp};
const EXTENSION_DOWNLOAD_URL: &str = "us3://baihai/docker-build/extension-store";
const US3CLI_DEST: &str = "/home/ray/us3cli-linux64";

pub async fn get_extension() {
    loop {
        let extension_path = business::path_tool::recommended_extensions();
        let extension_resp = match get_extensions_config(&extension_path).await{
            Ok(data) => data,
            Err(err) => {tracing::error!("{}", err.to_string());continue},
        };

        let parent_path = match extension_path.parent(){
            Some(data) => data,
            None => {tracing::error!("wrong extension path");continue},
        };

        let dest_path = parent_path.join("extension_temp.json");
        let mut cmd = tokio::process::Command::new("curl");
        cmd.arg("-o")
            .arg("-L")
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
        loop{
            tracing::info!("resp_new: {:#?}",resp_new);
            tracing::info!("resp: {:#?}",resp);
            match resp_new.cmp(&resp){
                std::cmp::Ordering::Less => {
                    //us3下来 in and 在最后的第一个地方插入0，继续
                    //    ./us3cli-linux64 cp -r us3://baihai/docker-build/extension-store/folder_name ./
                    //TODO_hy 测试一下这个command可能有问题
                    let origin_path = format!("{}/{}",EXTENSION_DOWNLOAD_URL,resp_new.unwrap().name);
                    let mut cmd = tokio::process::Command::new(US3CLI_DEST);
                    cmd.arg("cp")
                        .arg("-r")
                        .arg(&origin_path)
                        .arg(&extension_path);
                    match cmd.spawn().expect("can't get current extension_config").wait().await{
                        Ok(_) => tracing::info!("successful cp folder: {:#?} to destpath",origin_path),
                        Err(_) => tracing::info!("fail to cp folder: {:#?}",origin_path),
                    }

                    resp_new = resp_new_iter.next();
                },
                std::cmp::Ordering::Equal => {resp_new = resp_new_iter.next();resp = resp_iter.next();},
                std::cmp::Ordering::Greater => {
                    // 把后面那个删掉，后一列往后移
                    let remove_path = format!("{}/{}",EXTENSION_DOWNLOAD_URL,resp.unwrap().name);
                    match tokio::fs::remove_dir_all(&remove_path).await{
                        Ok(_) => tracing::info!("successful remove extension: {:#?}",remove_path),
                        Err(err) => tracing::error!("fail to cp folder: {:#?},err:{:#?}",remove_path,err),
                    };
                    resp = resp_iter.next();
                },
            }
            
            if (resp_new_iter.next() == None)&&(resp_iter.next() == None){
                break;
            }
        }

        // 拉完了之后用进来的config替代原有的config
        if tokio::fs::copy(&dest_path, &extension_path).await.is_err(){
            tracing::error!("fail to overwrite past extension_config");
        };

        tokio::time::sleep(std::time::Duration::from_secs(24*3600)).await;
    }
}