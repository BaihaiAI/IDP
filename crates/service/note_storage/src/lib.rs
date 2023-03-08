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

#![deny(unused_crate_dependencies)]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner::run_tests)]

use chrono::Utc;
use tokio_schedule as _;
use tokio_schedule::every;
use tokio_schedule::Job;

use crate::handler::deploy_prod_service::check_model_batch_service_handler;
use crate::handler::deploy_prod_service::check_model_service;
use crate::handler::extension::get_extension;
use crate::service::schedule::schedule_service::check_visual_modeling_job_handler;

mod api;
pub(crate) mod api_model;
mod app_context;
pub(crate) mod business_;
pub(crate) mod common;
mod handler;
pub(crate) mod pojo;
mod route;
pub(crate) mod service;
pub(crate) mod status_code;

pub async fn main() {
    let reload_log_level_handle = logger::init_logger();
    // clap::Command::new(env!("CARGO_PKG_NAME")).version(env!("VERSION")).get_matches();
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }
    license_verify();

    if business::kubernetes::is_k8s() {
        let every_1_minutes = every(1).minutes().at(0).in_timezone(&Utc).perform(move || {
            tracing::info!("Every 1 minutes at 0'th second");
            let pg_para = crate::app_context::DB_POOL.clone();
            async move {
                check_visual_modeling_job_handler(&pg_para).await;
                check_model_batch_service_handler(&pg_para).await;
                check_model_service(&pg_para).await;
            }
        });
        tokio::spawn(every_1_minutes);
        tokio::spawn(get_extension::get_extension());
    }

    let app = route::init_router(reload_log_level_handle).await;
    let address = std::net::SocketAddr::from((
        std::net::Ipv4Addr::UNSPECIFIED,
        business::note_storage_port(),
    ));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn license_verify() {
    if !business::kubernetes::is_k8s() {
        return;
    }
    match license_generator::verify_license(
        license_generator::DEFAULT_LICENSE_PUBLIC_KEY_PATH,
        license_generator::DEFAULT_LICENSE_PATH,
    ) {
        Ok(license) => {
            let expire_timestamp = license.expire_timestamp;
            tokio::spawn(async move {
                loop {
                    let timestamp = tokio::task::spawn_blocking(|| {
                        license_generator::get_timestamp_from_internet()
                    })
                    .await
                    .unwrap();
                    if timestamp > expire_timestamp {
                        tracing::error!("license expire, exit...");
                        std::process::exit(1);
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                }
            });
        }
        Err(err) => {
            tracing::error!("verify_license fail, exit... {err}");
            std::process::exit(1);
        }
    }
}
