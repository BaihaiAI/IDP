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

#[derive(Debug)]
pub struct StdoutStderrMsg {
    pub string: String,
    pub stdout_or_stderr: &'static str,
}

#[pyo3::pyclass]
pub struct PyStdoutStderr {
    pub sender: std::sync::mpsc::Sender<kernel_common::Message>,
    pub header: kernel_common::Header,
    pub stdout_or_stderr: &'static str,
    pub buf: String,
    /// ray cluster worker flush stdout has a delay to header, frontend would receive stdout after idle/duration msg
    pub is_busy: bool,
}

#[pyo3::pymethods]
impl PyStdoutStderr {
    fn write(&mut self, s: String) {
        self.buf.push_str(&s);
        // EOF or newline(python newline sometime means flush stdout)
        // if s == "\n" || self.buf.len() > libc::BUFSIZ as _ {
        if s == "\n" {
            self.flush();
        }
        if s.is_empty() && self.buf.ends_with('\n') {
            self.flush();
        }
    }

    /// matplotlib require
    fn flush(&mut self) {
        if self.buf.is_empty() {
            return;
        }
        self.sender
            .send(kernel_common::Message {
                header: self.header.clone(),
                content: kernel_common::Content::Stream {
                    name: self.stdout_or_stderr.to_string(),
                    text: std::mem::take(&mut self.buf),
                    is_busy: self.is_busy,
                },
            })
            .unwrap();
    }

    /// `import ray` require isatty
    fn isatty(&self) -> bool {
        // tracing::info!("--> isatty");
        true
    }

    fn fileno(&self) -> i32 {
        if self.stdout_or_stderr == "stdout" {
            1
        } else {
            2
        }
    }

    fn set_idle(&mut self) {
        self.is_busy = false
    }

    fn set_busy(&mut self) {
        self.is_busy = true
    }

    // publish display_data or execute_result message
    fn publish_ipython_data(&self, dict_json: String) {
        let data = serde_json::from_str::<std::collections::HashMap<String, String>>(&dict_json)
            .expect(&dict_json);
        if data.contains_key("image/png") || data.contains_key("image/jpeg") {
            self.sender
                .send(kernel_common::Message {
                    header: self.header.clone(),
                    content: kernel_common::Content::DisplayData { data },
                })
                .unwrap();
        } else {
            self.sender
                .send(kernel_common::Message {
                    header: self.header.clone(),
                    content: kernel_common::Content::ExecuteResult { data },
                })
                .unwrap();
        }
    }
}
