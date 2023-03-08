// Copyright 2023 BaihaiAI, Inc.
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

use axum::routing::get;
use axum::routing::post;
use axum::Router;
use handler::execute_code::browser_ws_handler::browser_ws_handler;
use handler::execute_code::kernel_ws_handler::kernel_ws_handler;

use crate::handler;

pub fn route() -> Router {
    let ctx = crate::app_context::AppContext::new();
    let kernel_router = Router::new()
        .route("/shutdown", post(handler::post_shutdown_or_restart))
        .route("/core_dumped_report", post(handler::core_dumped_report))
        .route("/list", get(handler::kernel_list))
        .route("/interrupt", get(handler::interrupt))
        .route("/shutdown_all", get(handler::shutdown_all))
        .route("/pause", get(handler::pause))
        .route("/resume", get(handler::resume))
        .with_state(ctx.clone());

    Router::new().nest(
        "/api/v1/execute",
        Router::new()
            .nest("/kernel", kernel_router)
            .route("/ws/kernel/execute", get(browser_ws_handler))
            .route("/ws/kernel/connect", get(kernel_ws_handler))
            .route("/notebook/cell_state", get(handler::cell_state))
            .route("/notebook/vars", get(handler::vars))
            .with_state(ctx),
    )
}
