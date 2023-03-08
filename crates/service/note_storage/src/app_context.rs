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

use cache_io::CacheService;
use once_cell::sync::Lazy;

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigToml {
    pub db_host: String,
    pub db_port: u16,
    pub db_user_name: String,
    pub db_password: String,
    pub db_database_name: String,
    pub deploy_mode: String,
    pub net_domain: String,
    pub extension_url: String,
}

impl ConfigToml {
    pub fn db_url(&self) -> String {
        let db_config = self;
        let db_host = &db_config.db_host;
        let db_port = db_config.db_port;
        let db_user_name = &db_config.db_user_name;
        let db_password = urlencoding::encode(&db_config.db_password);
        let db_database_name = &db_config.db_database_name;

        format!("postgres://{db_user_name}:{db_password}@{db_host}:{db_port}/{db_database_name}")
    }
}

pub static CONFIG: Lazy<ConfigToml> =
    Lazy::new(
        || match std::fs::read_to_string("/opt/config/config.toml") {
            Ok(toml_str) => toml::de::from_str::<ConfigToml>(&toml_str).unwrap(),
            Err(err) => {
                tracing::error!("/opt/config/config.toml {err}");
                ConfigToml {
                    db_host: "localhost".to_string(),
                    db_port: 5432,
                    db_user_name: "foo".to_string(),
                    db_password: "bar".to_string(),
                    db_database_name: "foo".to_string(),
                    deploy_mode: "Host".to_string(),
                    net_domain: "localhost".to_string(),
                    extension_url: "".to_string(),
                }
            }
        },
    );

pub static DB_POOL: Lazy<sqlx::PgPool> = Lazy::new(|| {
    std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let db_url = crate::app_context::CONFIG.db_url();
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
                .unwrap()
        })
    })
    .join()
    .unwrap()
});

#[derive(Clone)]
pub struct AppContext {
    pub redis_cache: CacheService,
    // pub pg_pool: PgPool,
}

impl AppContext {
    pub async fn new() -> Self {
        let cache_service =
            tokio::time::timeout(std::time::Duration::from_secs(20), CacheService::new())
                .await
                .expect("redis pool init timeout")
                .expect("CacheService::new");
        Self {
            redis_cache: cache_service,
        }
    }
}
