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
pub mod network;
// #[cfg(feature = "usage")]
#[cfg(not)]
pub mod resource_usage;
#[cfg(not(windows))]
pub use network::linux_impl::get_hostname;
#[cfg(windows)]
pub use network::windows_impl::get_hostname;

pub fn get_unused_port() -> u16 {
    std::net::TcpListener::bind(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::LOCALHOST,
        0,
    ))
    .unwrap()
    .local_addr()
    .unwrap()
    .port()
}

#[cfg(not)]
pub fn get_unused_port() -> libc::in_port_t {
    let listener = std::net::TcpListener::bind(std::net::SocketAddrV4::new(
        std::net::Ipv4Addr::LOCALHOST,
        0,
    ))
    .unwrap();
    let sockfd = std::os::unix::prelude::IntoRawFd::into_raw_fd(listener);
    unsafe {
        let mut addr = std::mem::zeroed();
        let mut addr_len = std::mem::size_of::<libc::sockaddr>() as libc::socklen_t;
        let res = libc::getsockname(sockfd, &mut addr, &mut addr_len);
        if res == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        let addr = *(&addr as *const libc::sockaddr as *const libc::sockaddr_in);
        addr.sin_port
    }
}
