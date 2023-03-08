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
mod full;
pub mod hpopt;
pub mod refresh_disk;
pub mod snapshot;
use std::collections::HashMap;
use std::sync::Arc;

use err::ErrorTrace;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use redis::Value;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

use crate::RefreshDto;

#[derive(Clone)]
pub struct Pool {
    pool: ConnectionManager,
}

impl Pool {
    async fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).expect("redis::Client::open");
        Self {
            pool: ConnectionManager::new(client)
                .await
                .expect("redis ConnectionManager::new"),
        }
    }
    #[allow(clippy::unused_async)]
    async fn get(&self) -> Result<ConnectionManager, ErrorTrace> {
        Ok(self.pool.clone())
    }
}

#[derive(Clone)]
pub struct CacheService {
    // pub(crate) connection: redis::aio::MultiplexedConnection,
    // pub(crate) pool: bb8_redis::bb8::Pool<bb8_redis::RedisConnectionManager>,
    pub(crate) pool: Pool,
    pub(crate) refresh_sender: tokio::sync::mpsc::Sender<RefreshDto>,
    // (path, cell_id)
    #[allow(clippy::type_complexity)]
    pub(crate) lock_for_update: Arc<RwLock<HashMap<(std::path::PathBuf, String), Arc<Mutex<()>>>>>,
}

impl std::fmt::Debug for CacheService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CacheService").finish()
    }
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

        let pool = Pool::new(&redis_url).await;
        let (refresh_sender, refresh_receiver) = tokio::sync::mpsc::channel::<RefreshDto>(30);

        let mut conn = pool.get().await?;
        let pong = redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .expect("redis send ping fail");
        drop(conn);
        assert_eq!(pong, "PONG");
        let pool_ = pool.clone();
        tokio::spawn(crate::redis::refresh_disk::spawn_refresh_disk(
            pool_,
            refresh_receiver,
        ));

        Ok(CacheService {
            pool,
            refresh_sender,
            lock_for_update: Arc::new(RwLock::new(HashMap::new())), // handle
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
            std::env::set_var("IDP_REDIS_PORT", 6379.to_string());
            let cache_svc = CacheService::new().await.unwrap();
            let mut conn = cache_svc.pool.get().await.unwrap();
            let hvals = conn.hvals::<_, Vec<String>>("nil").await.unwrap();
            dbg!(hvals);
        });
}
