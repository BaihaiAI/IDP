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

// #![deny(warnings)]
#![deny(unused_crate_dependencies)]
#![deny(non_ascii_idents)]
#![deny(noop_method_call)]
pub mod message;
// #[cfg(feature = "spawn")]
pub mod spawn_kernel_process;
// #[cfg(feature = "fifo")]
// pub mod transport;
pub mod kubernetes_client;
pub mod runtime_pod_status;
pub mod typedef;

pub use business;
pub use message::content;
pub use message::content::Content;
pub use message::header::Header;
pub use message::Message;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KernelInfo {
    pub ip: std::net::Ipv4Addr,
    pub pid: u32,
    // current is team_id+project_id
    pub pod_id: String,
    pub header: Header,
}

impl KernelInfo {
    pub const HTTP_HEADER: &'static str = "Kernel-Info";
}
