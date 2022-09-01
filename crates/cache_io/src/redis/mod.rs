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

mod cell;
pub mod conda_env;
mod disk_read_write;
pub mod distributed_lock;
mod full;
pub mod refresh_disk;
pub mod snapshot;
use bb8_redis::bb8;
use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use err::ErrorTrace;
use redis::AsyncCommands;
use redis::Value;

use crate::RefreshDto;

#[derive(Clone)]
pub struct CacheService {
    // pub(crate) connection: redis::aio::MultiplexedConnection,
    pub(crate) pool: Pool<RedisConnectionManager>,
    pub(crate) refresh_sender: tokio::sync::mpsc::Sender<RefreshDto>,
}

impl CacheService {
    pub async fn new() -> Result<CacheService, ErrorTrace> {
        let redis_url = if business::kubernetes::is_k8s() {
            "redis://redis-svc".to_string()
        } else {
            let redis_port = business::idp_redis_port();
            format!("redis://127.0.0.1:{redis_port}")
        };
        if cfg!(debug_assertions) {
            tracing::debug!("redis_url = {redis_url}");
        }
        let mut retry = 0;
        let manager = loop {
            match RedisConnectionManager::new(&*redis_url) {
                Ok(manager) => break manager,
                Err(err) => {
                    tracing::error!("{err}");
                    retry += 1;
                    if retry > 10 {
                        panic!("{err}");
                    }
                }
            }
        };
        let pool = bb8::Pool::builder()
            .connection_timeout(std::time::Duration::from_secs(35))
            .max_size(200)
            .build(manager)
            .await
            .unwrap();

        let (refresh_sender, refresh_receiver) = tokio::sync::mpsc::channel::<RefreshDto>(30);
        // if need_refresh_thread {
        let client = redis::Client::open(redis_url)?;
        let mut refresh_con = client.get_tokio_connection().await?;
        let pong = redis::cmd("PING")
            .query_async::<_, String>(&mut refresh_con)
            .await
            .unwrap();
        assert_eq!(pong, "PONG");
        let _handle = tokio::spawn(crate::redis::refresh_disk::spawn_refresh_disk(
            refresh_con,
            refresh_receiver,
        ));
        // }

        Ok(CacheService {
            pool,
            refresh_sender,
            // handle
        })
    }

    pub async fn del_file_cache_key(&self, key: &str) -> Result<(), ErrorTrace> {
        if let Err(err) = self.pool.get().await?.del::<_, Value>(key).await {
            tracing::error!("{err}");
        }
        Ok(())
    }
}

#[test]
#[ignore = "need redis-server"]
fn test_redis_connect() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            CacheService::new().await.unwrap();
        });
}
