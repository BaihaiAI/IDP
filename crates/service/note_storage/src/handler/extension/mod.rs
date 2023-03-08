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

mod detail;
pub(crate) mod get_extension;
mod init_install;
mod install;
mod installed_list;
mod load;
mod models;
mod recommended_list;
mod uninstall;
mod update;

#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::unix::prelude::RawFd;
use std::path::Path;

pub use detail::detail;
use err::ErrorTrace;
pub use init_install::init_install;
pub use install::install;
pub use installed_list::installed_list;
pub use load::load;
#[cfg(target_os = "linux")]
use nix::fcntl::FcntlArg;
pub use recommended_list::recommended_list;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
pub use uninstall::uninstall;
pub use update::update;

use self::models::ExtensionResp;

pub async fn get_extensions_config<P: AsRef<Path>>(
    extension_config_path: P,
) -> Result<Vec<ExtensionResp>, ErrorTrace> {
    let jdata = if !extension_config_path.as_ref().starts_with("/store") {
        //get recommended_content
        match std::fs::read_to_string(&extension_config_path) {
            Ok(jdata) => jdata,
            Err(err) => {
                let path = extension_config_path.as_ref();
                tracing::error!("{err},path:{:?}", path);
                return Err(ErrorTrace::new("extension config no exist"));
            }
        }
    } else {
        //get installed_content
        read_file_lock(&extension_config_path).await?
    };
    if jdata.is_empty() {
        return Err(ErrorTrace::new("extension_config.json is empty").code(400));
    }
    match serde_json::from_str::<Vec<ExtensionResp>>(&jdata) {
        Ok(mut content) => {
            for x in content.iter_mut() {
                x.visible = Some(x.is_visible())
            }
            content.sort();
            Ok(content)
        }
        Err(err) => {
            tracing::error!("{err}");
            Err(ErrorTrace::new("serde json extension path failed"))
        }
    }
}

async fn write_file_lock<P: AsRef<Path>>(path: P, data: String) -> Result<(), ErrorTrace> {
    let mut f = tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .await?;
    #[cfg(target_os = "linux")]
    let raw_fd: RawFd = f.as_raw_fd();

    //init file write lock
    let mut flock: libc::flock = unsafe { std::mem::zeroed() };
    flock.l_type = libc::F_WRLCK as libc::c_short;
    flock.l_whence = libc::SEEK_SET as libc::c_short;
    flock.l_start = 0;
    flock.l_len = 0;
    flock.l_pid = 0;
    //lock

    #[cfg(target_os = "linux")]
    {
        let mut i = 0;
        while nix::fcntl::fcntl(raw_fd, FcntlArg::F_OFD_SETLK(&flock)).is_err() && i <= 40 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            i += 1;
        }
        if i > 40 {
            let msg = format!("can not get F_WRLCK,path:{:?}", path.as_ref());
            return Err(ErrorTrace::new(&msg));
        }
    }

    #[cfg(target_os = "linux")]
    nix::fcntl::fcntl(raw_fd, FcntlArg::F_OFD_SETLK(&flock))?;

    //write
    f.write_all(data.as_bytes()).await?;
    //unlock
    flock.l_type = libc::F_UNLCK as libc::c_short;

    #[cfg(target_os = "linux")]
    nix::fcntl::fcntl(raw_fd, FcntlArg::F_OFD_SETLK(&flock))?;

    Ok(())
}

async fn read_file_lock<P: AsRef<Path>>(path: P) -> Result<String, ErrorTrace> {
    let mut f = tokio::fs::OpenOptions::new().read(true).open(&path).await?;
    //init file_readlock
    let mut flock: libc::flock = unsafe { std::mem::zeroed() };
    flock.l_type = libc::F_RDLCK as libc::c_short;
    flock.l_whence = libc::SEEK_SET as libc::c_short;
    flock.l_start = 0;
    flock.l_len = 0;
    flock.l_pid = 0;
    //lock
    #[cfg(target_os = "linux")]
    let raw_fd: RawFd = f.as_raw_fd();
    #[cfg(target_os = "linux")]
    {
        let mut i = 0;
        while nix::fcntl::fcntl(raw_fd, FcntlArg::F_OFD_SETLK(&flock)).is_err() && i <= 40 {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            i += 1;
        }
        if i > 40 {
            let msg = format!("can not open path{:?}", path.as_ref());
            return Err(ErrorTrace::new(&msg));
        }
    }

    //read file
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await?;
    //unlock
    flock.l_type = libc::F_UNLCK as libc::c_short;
    #[cfg(target_os = "linux")]
    nix::fcntl::fcntl(raw_fd, FcntlArg::F_OFD_SETLK(&flock))?;
    Ok(buffer)
}
