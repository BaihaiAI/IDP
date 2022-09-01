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
pub type ReloadLogLevelHandle =
    tracing_subscriber::reload::Handle<tracing_subscriber::EnvFilter, tracing_subscriber::Registry>;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
pub use tracing_subscriber::EnvFilter;
pub fn init_logger() -> ReloadLogLevelHandle {
    // use std::sync::Once;
    // static INIT: Once = Once::new();
    // INIT.call_once(|| {});
    let default_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info,sqlx=warn,hyper_reverse_proxy=warn".into()),
    );
    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(default_filter);

    _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_line_number(true)
                .with_ansi(false),
        )
        .try_init();
    reload_handle
}

#[cfg(not)]
#[cfg(feature = "tokio_console")]
pub fn init_logger_with_tokio_console() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::Layer;
    let console_layer = console_subscriber::ConsoleLayer::builder()
        // .server_addr((std::net::Ipv4Addr::UNSPECIFIED, 33333))
        .spawn();
    tracing_subscriber::registry()
        .with(
            console_layer.with_filter(
                tracing_subscriber::filter::Targets::new()
                    .with_target("tokio", tracing::Level::TRACE)
                    .with_target("runtime", tracing::Level::TRACE),
            ),
        )
        .with(
            tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
            )),
        )
        .init();
}
