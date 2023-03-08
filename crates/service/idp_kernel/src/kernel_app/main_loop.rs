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

impl super::KernelApp {
    #[cfg(feature = "tcp")]
    pub fn handle_req(
        &mut self,
        req: kernel_common::content::ExecuteRequest,
        header: kernel_common::Header,
    ) {
        let is_pipeline = header.pipeline_opt.is_some();

        self.ctx.last_req_cell_id = header.cell_id.clone();
        self.ctx.header.cell_id = header.cell_id;
        // msg_id is in parent_header field so we send None
        self.publish_content(Content::UpdateLastReq {});

        tracing::info!("kernel recv execute_request req");
        pyo3::Python::with_gil(|py| {
            self.handle_execute_req(req, is_pipeline, py);
        });
    }

    #[cfg(feature = "fifo")]
    #[cfg(not)]
    pub fn main_loop(mut self) {
        tracing::info!("--> main_loop");
        let filename = kernel_common::transport::kernel_req_pipe(std::process::id());
        let stream = std::fs::File::open(filename).unwrap();
        for msg_res in
            serde_json::Deserializer::from_reader(stream).into_iter::<kernel_common::Message>()
        {
            let msg = match msg_res {
                Ok(msg) => msg,
                Err(err) => {
                    tracing::error!("json from_reader de error {err}");
                    self.publish_content(kernel_common::Content::RuntimeError {
                        message: err.to_string(),
                    });
                    continue;
                }
            };
            let req = match msg.content {
                Content::ExecuteRequest(ref req) => req.clone(),
                _ => {
                    tracing::warn!("only support execute request");
                    self.publish_content(kernel_common::Content::RuntimeError {
                        message: "only support execute request".to_string(),
                    });
                    continue;
                }
            };

            self.ctx.last_req_cell_id = msg.header.cell_id;
            // msg_id is in parent_header field so we send None
            self.publish_content(Content::UpdateLastReq {});

            tracing::info!("kernel recv execute_request req");
            self.handle_execute_req(req, msg.header.pipeline_opt.is_some());
        }
    }
}
