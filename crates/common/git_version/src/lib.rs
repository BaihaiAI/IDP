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
pub fn version() -> String {
    #[cfg(unix)]
    extern "C" {
        fn time(timep: *mut i64) -> i64;
        /// char *ctime_r(const time_t *restrict timep, char *restrict buf);
        fn ctime_r(timep: *const i64, buf: *mut std::os::raw::c_char) -> *mut std::os::raw::c_char;
        // FIXME windows ctime_s() link error on binary
        #[cfg(not)]
        #[cfg(windows)]
        fn ctime_s(
            buffer: *mut std::os::raw::c_char,
            bufsize: usize,
            timep: *const i64,
        ) -> std::os::raw::c_int;
    }
    #[cfg(unix)]
    let built_at = unsafe {
        let mut buf = [0_u8; 32];
        ctime_r(&time(std::ptr::null_mut()), buf.as_mut_ptr().cast());
        String::from_utf8_unchecked(buf.to_vec())
    };
    #[cfg(windows)]
    let built_at = "".to_string();

    // alternative: git rev-parse --short HEAD
    let output = std::process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%h %ci %s")
        .output()
        .unwrap();
    if !output.status.success() {
        return "no .git".to_string();
    }
    let version = unsafe { String::from_utf8_unchecked(output.stdout) };
    format!("{}, built_at: {built_at}", version.trim_end())
}
