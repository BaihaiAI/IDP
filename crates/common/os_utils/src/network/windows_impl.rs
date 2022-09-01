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

use std::io::Error;

pub fn get_hostname() -> String {
    use winapi::um::sysinfoapi::ComputerNamePhysicalDnsHostname;
    use winapi::um::sysinfoapi::GetComputerNameExW;
    let mut buffer_size: u32 = 128;
    const BUF_LEN: usize = 128;
    let mut buffer = [0u16; BUF_LEN];
    unsafe {
        if GetComputerNameExW(
            ComputerNamePhysicalDnsHostname,
            buffer.as_mut_ptr() as *mut u16,
            &mut buffer_size,
        ) == 0
        {
            panic!("{}", Error::last_os_error());
        }
        let len = buffer.iter().position(|&b| b == 0).unwrap();
        String::from_utf16(&buffer[..len]).unwrap()
    }
}
