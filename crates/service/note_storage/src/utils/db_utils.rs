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

use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use tracing::error;
use urlencoding::encode;

use crate::utils::db_config::DbConfig;

pub async fn init_pg_connect_pool() -> Pool<Postgres> {
    tracing::debug!("init_pg_connect_pool start ");
    let database_info = std::fs::read_to_string("/opt/config/config.toml");
    let database_info_tmp = if database_info.is_err() {
        error!("read /opt/config.toml failed");
        std::fs::read_to_string("/etc/db_config.toml").unwrap()
    } else {
        database_info.unwrap()
    };

    let toml_str = database_info_tmp.as_str();
    let db_config: DbConfig = toml::from_str(toml_str).unwrap();

    let db_host = db_config.db_host;
    let db_port = db_config.db_port;
    let db_user_name = db_config.db_user_name;
    let db_password = encode(&db_config.db_password);
    let db_database_name = db_config.db_database_name;

    let db_url =
        format!("postgres://{db_user_name}:{db_password}@{db_host}:{db_port}/{db_database_name}");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}
