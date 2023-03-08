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

mod check_pod_idle;
mod handler;
mod route;

pub async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }
    logger::init_logger();
    tracing::info!("--> spawner::main");
    check_pod_idle::spawn_check_pod_idle_thread();
    let app = route::init_router();
    let addr =
        std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, business::spawner_port()));
    let listener = std::net::TcpListener::bind(addr).unwrap_or_else(|_| panic!("bind {addr} fail"));
    tracing::info!("after spawner bind {addr}");
    axum::Server::from_tcp(listener)
        .expect("axum::Server::from_tcp")
        .serve(app.into_make_service())
        .await
        .expect("axum serve");
}
