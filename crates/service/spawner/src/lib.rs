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

mod handler;
mod route;

pub async fn main() {
    logger::init_logger();
    let app = route::init_router();
    let address =
        std::net::SocketAddr::from((std::net::Ipv4Addr::UNSPECIFIED, business::spawner_port()));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
