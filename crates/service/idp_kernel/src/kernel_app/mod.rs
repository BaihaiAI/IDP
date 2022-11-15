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

mod handle_execute_req_impl;
mod main_loop;
mod output_sender_thread;

use kernel_common::Message;

use crate::execute_code::execute_code_context::ExecuteCodeTransportCtx;
use crate::kernel_init::init_python::PythonDefines;

pub struct KernelApp {
    pub ctx: ExecuteCodeTransportCtx,
    pub execution_count: u32,
    pub(crate) python_defines: PythonDefines,
}

impl KernelApp {
    pub fn new(
        output_tx: crossbeam_channel::Sender<Message>,
        input_reply_receiver: std::sync::mpsc::Receiver<String>,
        args: kernel_common::Header,
    ) -> Self {
        let (output_sender, output_receiver) = std::sync::mpsc::channel::<Message>();
        pyo3::prepare_freethreaded_python();
        let python_defines = pyo3::Python::with_gil(|py| {
            crate::kernel_init::init_python::init_python(
                output_sender.clone(),
                input_reply_receiver,
                &args,
                py,
            )
        });
        output_sender_thread::spawn_output_sender_thread(output_tx, output_receiver);

        KernelApp {
            ctx: ExecuteCodeTransportCtx {
                last_req_cell_id: "".to_string(),
                output_sender,
                header: args,
            },
            execution_count: 1,
            python_defines,
        }
    }
}
