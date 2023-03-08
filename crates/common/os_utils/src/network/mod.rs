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

#[cfg(not(windows))]
pub mod linux_impl;
#[cfg(windows)]
pub mod windows_impl;

/// kubernetes only support ipv4
pub fn dns_resolve(hostname: &str) -> std::net::Ipv4Addr {
    for ip in std::net::ToSocketAddrs::to_socket_addrs(&format!("{hostname}:0"))
        .ok()
        .unwrap()
    {
        if let std::net::SocketAddr::V4(ipv4) = ip {
            let ipv4 = *ipv4.ip();
            if ipv4 != std::net::Ipv4Addr::UNSPECIFIED && ipv4 != std::net::Ipv4Addr::LOCALHOST {
                return ipv4;
            }
        }
    }
    eprintln!("[dns_resolve]: DNS resolve {hostname} failed, use localhost",);
    std::net::Ipv4Addr::LOCALHOST
}
