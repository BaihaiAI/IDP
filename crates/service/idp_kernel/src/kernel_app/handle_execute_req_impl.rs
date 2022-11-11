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

use kernel_common::business;
use kernel_common::content::ExecuteReply;
use kernel_common::content::ExecutionState;
use kernel_common::content::ReplyStatus;
use kernel_common::Content;
use tracing::debug;

impl super::KernelApp {
    pub(crate) fn publish_content(&self, content: Content) {
        self.ctx.publish_content(content);
    }

    pub(crate) fn handle_execute_req(
        &mut self,
        req: kernel_common::content::ExecuteRequest,
        is_pipeline: bool,
    ) {
        let start = std::time::Instant::now();
        let run_at = unsafe { libc::time(std::ptr::null_mut()) };
        // self.publish_content(Content::Status {
        //     execution_state: ExecutionState::Busy,
        // });

        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();

        let sys = py.import("sys").unwrap();
        let stdout = sys.getattr("stdout").unwrap();
        let stderr = sys.getattr("stderr").unwrap();

        stdout.call_method0("set_busy").unwrap();
        stderr.call_method0("set_busy").unwrap();

        let session_file_path = business::path_tool::session_file_path(
            self.ctx.header.team_id,
            self.ctx.header.project_id,
            &self.ctx.header.path,
        );
        let vars_file_path = business::path_tool::vars_file_path(
            self.ctx.header.team_id,
            self.ctx.header.project_id,
            &self.ctx.header.path,
        );

        let enable_checkpoint = "false";
        if !is_pipeline {
            if let Err(err) = self.python_defines.load_or_skip.call1(
                py,
                pyo3::types::PyTuple::new(py, &[&session_file_path, enable_checkpoint]),
            ) {
                tracing::warn!("load_or_skip {err}");
            };
        }

        self.publish_content(Content::ExecuteInput {
            code: req.code.clone(),
            execution_count: self.execution_count,
        });
        debug!(
            "after publish executeInput, time cost {:?}",
            start.elapsed()
        );

        let mut execute_ctx = crate::execute_code::execute_code_context::ExecuteCodeContext::new(
            req.code.clone(),
            py,
            self.python_defines.clone(),
            self.ctx.clone(),
        );

        // first we need to escape some char which added by frontend when transfer
        match execute_ctx.execute() {
            Ok(()) => {
                self.publish_content(Content::ExecuteReply(ExecuteReply {
                    execution_count: self.execution_count,
                    reply_status: ReplyStatus::Ok,
                }));
            }
            Err(err) => {
                let eval_part_lineno_offset = if execute_ctx.is_running_eval_part {
                    Some(execute_ctx.eval_part_lieno_offset)
                } else {
                    None
                };
                let err = crate::execute_code::traceback::convert_pyerr(
                    err,
                    py,
                    execute_ctx.code,
                    eval_part_lineno_offset,
                );
                self.publish_content(Content::ExecuteReply(ExecuteReply {
                    execution_count: self.execution_count,
                    reply_status: ReplyStatus::Error(err.clone()),
                }));
                self.publish_content(Content::Error(err));
            }
        }

        // must send before idle and duration
        _ = stdout.call_method0("flush");
        _ = stderr.call_method0("flush");

        self.publish_content(Content::Status {
            execution_state: ExecutionState::Idle,
        });
        self.execution_count += 1;

        if !is_pipeline {
            if let Err(err) = self.python_defines.after_run.call1(
                py,
                pyo3::types::PyTuple::new(py, &[
                    &session_file_path,
                    &vars_file_path,
                    enable_checkpoint,
                ]),
            ) {
                tracing::error!("after_run {err}");
            };
        }

        self.publish_content(Content::Duration {
            run_at: run_at as u64,
            duration: start.elapsed().as_millis() as u32,
            code: req.code,
        });
        stdout.call_method0("set_idle").unwrap();
        stderr.call_method0("set_idle").unwrap();
    }
}
