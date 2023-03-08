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

mod pip_install;
mod pip_list;
mod pip_uninstall;
mod search;
use std::collections::HashMap;
use std::sync::Arc;

use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tokio::sync::Mutex;

pub fn routes() -> Router {
    let project_info_map = if business::kubernetes::is_k8s() {
        Arc::new(Mutex::new(HashMap::<String, HashMap<String, String>>::new()))
    } else {
        let mut map = HashMap::<String, HashMap<String, String>>::new();
        map.insert("1+1".to_string(), HashMap::new());
        Arc::new(Mutex::new(map))
    };
    let pg_option = if business::kubernetes::is_k8s() {
        tokio::spawn(search::get_package_map(project_info_map.clone(), true));
        Some(crate::app_context::DB_POOL.clone())
    } else {
        None
    };
    Router::new()
        .route("/list", get(pip_list::pip_list))
        .route("/search", get(search::search))
        .route("/install", post(pip_install::pip_install))
        .route("/uninstall", post(pip_uninstall::pip_uninstall))
        .with_state((pg_option, project_info_map))
}
