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

use super::get_extensions_config;
const EXTENSION_HTTP_URL: &str = "http://baihai.cn-bj.ufileos.com/docker-build/extension-store/";
const US3CLI_DEST: &str = "/home/ray/us3cli-linux64";
const US3_URL: &str = "us3://baihai/docker-build/extension-store/";

pub async fn get_extension() {
    loop {
        let store_path = business::path_tool::recommended_extensions();
        let extension_path = store_path.join("extensions_config.json");
        let extension_resp = match get_extensions_config(&extension_path).await {
            Ok(data) => data,
            Err(err) => {
                tracing::error!("{}", err.to_string());
                continue;
            }
        };

        let dest_path = store_path.join("extension_temp.json");
        let origin_url = format!("{EXTENSION_HTTP_URL}extensions_config.json");
        let mut cmd = tokio::process::Command::new("curl");
        cmd.arg("-L").arg("-o").arg(&dest_path).arg(&origin_url);
        let extension_resp_new = match cmd
            .spawn()
            .expect("can't get current extension_config")
            .wait()
            .await
        {
            Ok(_) => match get_extensions_config(&dest_path).await {
                Ok(extension_data) => extension_data,
                Err(err) => {
                    tracing::error!("{}", err.to_string());
                    continue;
                }
            },
            Err(err) => {
                tracing::error!("{}", err.to_string());
                continue;
            }
        };
        let mut resp_new_iter = extension_resp_new.iter();
        let mut resp_iter = extension_resp.iter();
        let mut resp_new = resp_new_iter.next();
        let mut resp = resp_iter.next();
        while (resp != None) || (resp_new != None) {
            tracing::debug!("resp_new: {:#?}", resp_new);
            tracing::debug!("resp: {:#?}", resp);
            if resp.is_none() || (resp_new.cmp(&resp).is_lt() && resp_new.is_some()) {
                tracing::info!("get_remote_extension");
                let origin_name = match resp_new {
                    Some(data) => &data.name,
                    None => "",
                };
                get_remote_extension(origin_name).await;
                resp_new = resp_new_iter.next();
            } else if resp_new.is_none() || (resp_new.cmp(&resp).is_gt() && resp.is_some()) {
                tracing::info!("remove extension");
                let origin_name = match resp {
                    Some(data) => &data.name,
                    None => "",
                };
                let remove_path = store_path.join(origin_name);
                match tokio::fs::remove_dir_all(&remove_path).await {
                    Ok(_) => tracing::info!("successful remove extension: {:#?}", remove_path),
                    Err(err) => {
                        tracing::error!("fail to cp folder: {:#?},err:{:#?}", remove_path, err)
                    }
                };
                resp = resp_iter.next();
            } else {
                resp_new = resp_new_iter.next();
                resp = resp_iter.next();
            };
        }
        tracing::info!("over");
        if tokio::fs::copy(&dest_path, &extension_path).await.is_err() {
            tracing::error!("fail to overwrite past extension_config");
        };

        tokio::time::sleep(std::time::Duration::from_secs(500)).await;
    }
}

pub async fn get_remote_extension(name: &str) {
    let store_path = business::path_tool::recommended_extensions();
    let dest_path = store_path.join(name);
    let origin_url = format!("{}{}", US3_URL, name);
    if dest_path.exists() {
        match tokio::fs::remove_dir_all(&dest_path).await {
            Ok(_) => tracing::debug!("successful overwrite extension: {:#?}", dest_path),
            Err(err) => {
                tracing::error!("fail to cp folder: {:#?},err:{:#?}", dest_path, err);
                return;
            }
        };
    }
    let mut cmd = tokio::process::Command::new(US3CLI_DEST);
    cmd.arg("cp").arg("-r").arg(&origin_url).arg(&dest_path);
    match cmd
        .spawn()
        .expect("can't get current extension_config")
        .wait()
        .await
    {
        Ok(_) => tracing::info!(
            "successful cp folder: {:#?} to destpath:{:#?}",
            origin_url,
            dest_path
        ),
        Err(_) => tracing::info!(
            "fail to cp folder: {:#?} to destpath:{:#?}",
            origin_url,
            dest_path
        ),
    }
}
