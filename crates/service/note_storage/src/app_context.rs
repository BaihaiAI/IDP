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

#[derive(Clone)]
pub struct AppContext {
    pub redis_cache: CacheService,
    // pub pg_pool: PgPool,
}
pub struct Config {
    pub sqlite_path: String,
    pub redis_url: String,
}

impl AppContext {
    pub async fn new(_config: Config) -> Self {
        let cache_service = CacheService::new().await.unwrap();
        Self {
            redis_cache: cache_service,
            // pg_pool: todo!(),
        }
    }
}
