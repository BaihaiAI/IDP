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

use kernel_common::Content;
use kernel_common::Message;
use tracing::debug;
use tracing::info;

use super::escape_slash_from_frontend::escape_slash_from_frontend;
use crate::kernel_init::init_python::PythonDefines;

pub(crate) struct ExecuteCodeContext<'py> {
    pub code: String,
    // remove cell_type because cell_type always code
    // cell_type sql would convert to cell_type code from upstream
    // pub cell_type: String,
    pub py: pyo3::Python<'py>,
    // jupyter context
    // zmq context
    pub transport_ctx: ExecuteCodeTransportCtx,
    pub python_defines: PythonDefines,
    // used in traceback
    pub is_running_eval_part: bool,
    pub eval_part_lieno_offset: usize,
    /// better alternative add stdout.publish_display_data?
    pub flush_matplotlib_flag: bool,
}

#[derive(Clone)]
pub struct ExecuteCodeTransportCtx {
    pub last_req_cell_id: String,
    pub header: kernel_common::Header,
    pub output_sender: std::sync::mpsc::Sender<Message>,
}

impl ExecuteCodeTransportCtx {
    pub(crate) fn make_msg(&self, content: Content) -> Message {
        Message {
            header: self.header.clone(),
            content,
        }
    }
    pub(crate) fn publish(&self, message: Message) {
        if let Err(err) = self.output_sender.send(message) {
            tracing::error!("publish err: {err}");
        }
    }
    pub(crate) fn publish_content(&self, content: Content) {
        let msg = self.make_msg(content);
        self.publish(msg);
    }
    pub(crate) fn display_data(&self, data: std::collections::HashMap<String, String>) {
        self.publish_content(Content::DisplayData { data });
    }
    pub(crate) fn execute_result(&self, data: std::collections::HashMap<String, String>) {
        self.publish_content(Content::ExecuteResult { data });
    }
}

impl<'py> ExecuteCodeContext<'py> {
    pub fn new(
        code: String,
        // execution_count: u32,
        py: pyo3::Python<'py>,
        python_defines: PythonDefines,
        ctx: ExecuteCodeTransportCtx,
    ) -> Self {
        let code = escape_slash_from_frontend(code);
        let args = pyo3::types::PyTuple::new(py, &[&code]);
        // AttributeError: module 'IPython' has no attribute 'utils'
        let code = match python_defines.cvt_magic_code.call1(py, args) {
            Ok(code_after_convert) => code_after_convert.to_string(),
            Err(err) => {
                tracing::error!("{err}");
                wrap_shell_if_starts_with_exclamation_mark(code)
            }
        };
        Self {
            // code: code.lines().map(ToString::to_string).collect::<Vec<_>>(),
            code,
            py,
            // execution_count,
            transport_ctx: ctx,
            python_defines,
            is_running_eval_part: false,
            eval_part_lieno_offset: 0,
            flush_matplotlib_flag: false,
        }
    }
    pub fn execute(&mut self) -> Result<(), pyo3::PyErr> {
        let start = std::time::Instant::now();
        let (exec_part, last_expr_opt) = self.split_code_to_exec_and_eval_part()?;
        debug!("after split code, time cost {:?}", start.elapsed());
        #[cfg(debug_assertions)]
        {
            if let Some(ref exec_part) = exec_part {
                debug!("==== start of exec_part: ");
                debug!("{exec_part}");
                debug!("==== end of exec_part: ");
            }
            if let Some(ref last_expr_opt) = last_expr_opt {
                debug!("==== start of last_expr_opt: ");
                debug!("{last_expr_opt}");
                debug!("==== end of last_expr_opt: ");
            }
        }

        if let Some(exec_part) = exec_part {
            self.py.run(&exec_part, None, None)?;
            debug!("after py.run, time cost {:?}", start.elapsed());
        }

        let last_expr = match last_expr_opt {
            Some(last_expr) => last_expr,
            None => return Ok(()),
        };
        self.is_running_eval_part = true;
        let last_expr_output = self.py.eval(&last_expr, None, None)?;
        debug!("after py.eval, time cost {:?}", start.elapsed());

        // e.g. print would return None
        if last_expr_output.is_none() {
            return Ok(());
        }

        self.handle_output(last_expr_output);
        info!("end execute, time cost {:?}", start.elapsed());
        Ok(())
    }
}

fn wrap_shell_if_starts_with_exclamation_mark(code: String) -> String {
    dbg!(line!());
    if !code.trim_start().starts_with('!') {
        return code;
    }
    // unwrap or /usr/bin/true
    let shell = code
        .trim_start()
        .trim_start_matches('!')
        .split('\n')
        .take(1)
        .next()
        .unwrap_or("true");
    let args = shell
        .split_whitespace()
        .map(|arg| format!("'''{arg}'''"))
        .collect::<Vec<_>>()
        .join(",");
    format!("__import__('baihai_aid').fake_shell([{args}])")
}
