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

use std::time::Duration;

use common_model::entity::notebook::Notebook;
use common_model::enums::mime::Mimetype;
use redis::aio::Connection;
use redis::AsyncCommands;
use redis::RedisResult;
use tokio::time::sleep;
use tracing::debug;
use tracing::error;

use super::distributed_lock;
use crate::keys::ipynb_key;
use crate::RefreshDto;

///
///  refresh disk distributed_lock fetch failed,throw this error.
#[derive(Debug)]
pub struct LockError {}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    pub length: usize,
    pub last_modified: String,
    pub content: String,
}

///
/// TODO write file content need add distributed lock.
///
pub(crate) async fn spawn_refresh_disk(
    mut connection: Connection,
    mut refresh_receiver: tokio::sync::mpsc::Receiver<RefreshDto>,
) {
    debug!("refresh disk thread start!");
    loop {
        match refresh_receiver.recv().await {
            None => {
                error!("received none! maybe the channel has been closed.");
            }
            Some(refresh_dto) => {
                refresh_disk(refresh_dto, &mut connection).await;
            }
        }
    }
}
// debug!("lock error, sleep and retry.");
// sleep(Duration::from_secs(1)).await;
async fn write_file_to_disk(
    refresh_dto: RefreshDto,
    cache_result: RedisResult<String>,
    connection: &mut Connection,
) {
    match cache_result {
        Ok(file_json_content) => {
            match serde_json::from_str::<FileContent>(&file_json_content) {
                Ok(file_content_obj) => {
                    // in standalone version no data race no need to lock
                    // if !business::kubernetes::is_k8s() {
                    if let Err(err) = common_tools::file_tool::write_large_to_nfs(
                        refresh_dto.path.clone(),
                        file_json_content,
                        refresh_dto.file_type.clone(),
                    )
                    .await
                    {
                        tracing::error!("{err}");
                    };
                    // return;
                    // }

                    debug!("prepare invoke file_tool!");
                    let mut retry_times = 3;
                    loop {
                        if retry_times == 0 {
                            error!("used up retry times,write notebook to disk still failed!");
                            break;
                        }
                        match lock_write_file_content_to_disk(
                            connection,
                            refresh_dto.clone(),
                            file_content_obj.content.clone(),
                        )
                        .await
                        {
                            Ok(()) => {
                                break;
                            }
                            Err(_e) => {
                                retry_times -= 1;
                                debug!("lock failed,sleep and retry,retry number:{}", retry_times);
                                sleep(Duration::from_secs(1)).await;
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("write file content json format error:=>{:?}", err);
                }
            };
        }
        Err(err) => {
            error!("redis error : {:?}", err);
        }
    }
}
async fn write_cache_notebook_to_disk(
    connection: &mut Connection,
    cache_result: RedisResult<Vec<String>>,
    refresh_dto: RefreshDto,
) {
    match cache_result {
        Ok(val_vec) => match crate::vec_string_into_notebook(val_vec) {
            Ok(notebook) => {
                debug!("######write notebook to disk:{:?}", notebook);
                let mut retry_times = 3;
                loop {
                    if retry_times == 0 {
                        error!("used up retry times,write notebook to disk still failed!");
                        break;
                    }
                    match lock_write_notebook_to_disk(connection, refresh_dto.clone(), &notebook)
                        .await
                    {
                        Ok(()) => {
                            break;
                        }
                        Err(_e) => {
                            retry_times -= 1;
                            debug!("lock failed,sleep and retry.");
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
            }
            Err(err) => {
                error!("vec_string_into_notebook error: {}", err)
            }
        },
        Err(err) => {
            error!("redis error : {:?}", err);
        }
    }
}
async fn lock_write_notebook_to_disk(
    connection: &mut Connection,
    refresh_dto: RefreshDto,
    notebook: &Notebook,
) -> Result<(), LockError> {
    //FIXME temporary solution: writing notebook is forbidden when it's cells is empty.
    if notebook.cells.is_empty() {
        error!("notebook cells is empty, skip write notebook to disk");
        return Ok(());
    }
    let lock_result = distributed_lock::try_lock(connection, refresh_dto.key.clone(), 5).await;
    if let Ok(lock_value) = lock_result {
        debug!("lock notebook,path:{:?}", notebook.path());
        if let Err(ref e) =
            common_tools::file_tool::write_notebook_to_disk(refresh_dto.path, notebook).await
        {
            error!("write_notebook_to_disk error: {:?}", e)
        }
        // In any case the lock has to be release.
        if let Err(ref e) =
            distributed_lock::release_lock(connection, refresh_dto.key, lock_value).await
        {
            error!("release_lock error: {:?}", e);
        }
        Ok(())
    } else {
        Err(LockError {})
    }
}
async fn lock_write_file_content_to_disk(
    connection: &mut Connection,
    refresh_dto: RefreshDto,
    file_content: String,
) -> Result<(), LockError> {
    let lock_result = distributed_lock::try_lock(connection, refresh_dto.key.clone(), 3).await;
    if let Ok(lock_value) = lock_result {
        debug!("lock file ,path:{:?}", refresh_dto.path);

        if let Err(ref e) = common_tools::file_tool::write_large_to_nfs(
            refresh_dto.path,
            file_content,
            refresh_dto.file_type,
        )
        .await
        {
            error!("write_file_content_to_disk error: {:?}", e);
        };
        //unlock
        if let Err(ref e) =
            distributed_lock::release_lock(connection, refresh_dto.key, lock_value).await
        {
            error!("release_lock error: {:?}", e)
        }
        Ok(())
    } else {
        Err(LockError {})
    }
}

async fn refresh_disk(refresh_dto: RefreshDto, connection: &mut Connection) {
    debug!("receive a refresh request from {:?} ", refresh_dto);
    //fetch cache and refresh to disk
    // read cache

    match refresh_dto.file_type {
        Mimetype::Notebook => {
            // get cache
            let key = ipynb_key(&refresh_dto.path, refresh_dto.project_id);
            let cache_result: RedisResult<Vec<String>> = connection.hvals(key).await;
            write_cache_notebook_to_disk(connection, cache_result, refresh_dto).await;
        }
        _ => {
            tracing::warn!("redis->disk: normal_file/image should not use redis");
            let cache_result: RedisResult<String> = connection.get(&refresh_dto.key).await;
            write_file_to_disk(refresh_dto.clone(), cache_result, connection).await;
        }
    }
}
