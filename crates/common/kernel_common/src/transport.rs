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

// socketpair is better, but we are not parent-child relation after criu restore
fn make_pipe_return_filename(read_or_write_to_kernel: &str, pid: u32) -> String {
    let filename = if unsafe { libc::access("/run\0".as_ptr().cast(), libc::W_OK) } == -1 {
        format!("/store/run/pid_{pid}_inode_{read_or_write_to_kernel}.fifo")
    } else {
        format!("/run/pid_{pid}_inode_{read_or_write_to_kernel}.fifo")
    };
    let filename_with_nul_byte = format!("{filename}\0");
    let filename_with_nul_byte_ptr = filename_with_nul_byte.as_ptr().cast();
    // check fifo(pipe) file exist
    if unsafe { libc::access(filename_with_nul_byte_ptr, libc::F_OK) } == -1 {
        // let old_mask = unsafe { libc::umask(0) };
        let ret = unsafe {
            libc::mkfifo(
                filename_with_nul_byte_ptr,
                libc::S_IRUSR | libc::S_IWUSR | libc::S_IROTH | libc::S_IWOTH,
            )
        };
        if ret == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        // unsafe { libc::umask(old_mask) };
    }
    filename
}

pub fn kernel_rsp_pipe(pid: u32) -> String {
    make_pipe_return_filename("kernel_rsp_pipe", pid)
}

pub fn kernel_req_pipe(pid: u32) -> String {
    make_pipe_return_filename("kernel_req_pipe", pid)
}
