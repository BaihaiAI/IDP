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
pub mod business_term;
pub mod kubernetes;
pub mod path_tool;
#[cfg(not)]
#[cfg(feature = "pip_install")]
pub mod pip_install;
pub mod region;
pub use os_utils;

fn env_or_default<T: std::str::FromStr>(env_key: &str, default: T) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    if let Ok(val) = std::env::var(env_key) {
        match val.parse() {
            Ok(val) => val,
            Err(err) => {
                tracing::error!("{err:?}");
                default
            }
        }
    } else {
        default
    }
}

pub fn note_storage_port() -> u16 {
    env_or_default("NOTE_STORAGE_PORT", 8082)
}

pub fn kernel_manage_port() -> u16 {
    env_or_default("KERNEL_MANAGE_PORT", 9007)
}

pub fn idp_redis_port() -> u16 {
    env_or_default("IDP_REDIS_PORT", 16379)
}
