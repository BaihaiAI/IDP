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

pub mod graph_run_impl;
pub mod helper;
pub mod manual_run_job_instance;
pub mod prelude;
mod run_cancel;
pub mod run_log;
mod run_result;
mod run_status;
#[cfg(test)]
mod test;

use axum::routing::get;
use axum::routing::post;
use axum::Router;

pub fn routes() -> Router {
    // run_status::spawn_run_status_msg_queue_write_db_consumer();
    Router::new()
        .route(
            "/model-instance/run-once",
            post(manual_run_job_instance::manual_run_job_instance),
        )
        .route("/run/status", get(run_status::status))
        .route("/run/log", get(run_log::logs))
        .route("/run/cancel", post(run_cancel::cancel))
        .route("/run/result", get(run_result::result))
}
