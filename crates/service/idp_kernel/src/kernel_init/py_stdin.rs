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

// use std::sync::Arc;
// use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub static mut IS_WAITING_INPUT_REPLY: bool = false;

#[pyo3::pyclass]
pub struct PyStdin {
    pub input_request_sender: std::sync::mpsc::Sender<kernel_common::Message>,
    pub input_reply_receiver: std::sync::mpsc::Receiver<String>,
    pub header: kernel_common::Header,
}

#[pyo3::pymethods]
impl PyStdin {
    /// https://docs.python.org/3/library/io.html#io.IOBase.readline
    /// https://docs.python.org/3/library/io.html#io.TextIOBase.readline
    /*
    TODO support getpass lib
    ```python
    from getpass import getpass
    password = getpass()
    ```
    */
    fn readline(&mut self) -> String {
        self.input_request_sender
            .send(kernel_common::Message {
                header: self.header.clone(),
                content: kernel_common::Content::InputRequest {
                    prompt: "".to_string(),
                    password: false,
                },
            })
            .unwrap();
        unsafe {
            IS_WAITING_INPUT_REPLY = true;
        }
        let input = self.input_reply_receiver.recv().unwrap();
        unsafe {
            IS_WAITING_INPUT_REPLY = false;
        }
        input
    }
}
