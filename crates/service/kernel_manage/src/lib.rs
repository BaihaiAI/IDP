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
#![deny(clippy::unused_async)]
#[macro_use]
mod macros;
pub(crate) mod app_context;
// pub mod dmesg_monitor;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod kernel_entry;
pub(crate) mod resp;
pub(crate) mod route;
pub(crate) use app_context::AppContext;
pub(crate) use error::Error;
#[cfg(test)]
mod ws_integration_tests;

pub async fn main() {
    logger::init_logger();
    // clap::Command::new(env!("CARGO_PKG_NAME"))
    //     .version(env!("VERSION"))
    //     .get_matches();
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }

    let ctx = app_context::AppContext::new().await;
    // kernel_manage::app_context::restore_ctx_from_disk(&ctx).await;
    // tokio::spawn(kernel_manage::app_context::dump_ctx_to_disk_task(ctx.clone(),));
    // tokio::spawn(kernel_manage::dmesg_monitor::dmesg_watcher(ctx.clone()));
    let service = hyper::service::make_service_fn(move |addr: &hyper::server::conn::AddrStream| {
        let remote_addr = addr.remote_addr();
        // clone once to service
        let ctx = ctx.clone();
        async move {
            Ok::<_, std::convert::Infallible>(hyper::service::service_fn(move |req| {
                // clone again to let closure outlive main thread
                crate::route::service_route(ctx.clone(), req, remote_addr)
            }))
        }
    });
    let server = hyper::Server::bind(
        &(
            std::net::Ipv4Addr::UNSPECIFIED,
            business::kernel_manage_port(),
        )
            .into(),
    )
    .serve(service);
    server.await.unwrap();
}
