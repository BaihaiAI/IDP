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

use crate::execute_code::PLOTLY_FILE;

impl<'py> super::execute_code_context::ExecuteCodeContext<'py> {
    /// matplotlib plt.show return None
    pub fn handle_output(&self, eval_output: &pyo3::PyAny) {
        let output_type_name = match eval_output.get_type().name() {
            Ok(type_) => type_,
            Err(_) => {
                tracing::error!("eval_output.get_type().name()");
                return self.handle_mime_output(eval_output);
            }
        };

        tracing::debug!("output_type_name = {output_type_name}");
        match output_type_name {
            // "GraphicObj" => {
            //     if self.flush_matplotlib_flag {
            //         self.handle_matplotlib_output(eval_output);
            //     }
            // }
            "str" => {
                // handle plotly
                let str_result = eval_output.extract::<String>().unwrap();
                if str_result.eq(PLOTLY_FILE) {
                    self.handle_plotly_file_output();
                } else if str_result.starts_with("<div>") && str_result.ends_with("</div>") {
                    self.handle_plotly_div_output(eval_output);
                } else {
                    self.handle_mime_output(eval_output);
                }
            }
            // "list" => {
            //     self.handle_mpl_fig_list(eval_output);
            // }
            // "wandb.sdk.wandb_run.Run"
            "Run" => {
                tracing::debug!("--> is a wandb type?");
                // if let Ok(wandb) = self.py.import("wandb") {
                //     let type_ = wandb
                //         .getattr("sdk")
                //         .unwrap()
                //         .getattr("wandb_run")
                //         .unwrap()
                //         .getattr("Run")
                //         .unwrap()
                //         .get_type();
                //     if let Ok(bool_) = eval_output.is_instance(type_) {
                //         if !bool_ {
                //             tracing::error!("not a wandb.sdk.wandb_run.Run");
                //             return;
                //         }
                //     }
                // }
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "text/plain".to_string(),
                    eval_output.repr().unwrap().to_string(),
                );
                let url = match eval_output.getattr("url") {
                    Ok(url) => url,
                    Err(err) => {
                        tracing::error!("{err}");
                        return;
                    }
                };
                let url = url.extract::<String>().unwrap();
                map.insert(
                    "text/html".to_string(),
                    format!(r#"<button onClick="this.nextSibling.style.display='block';this.style.display='none';">Display W&B run</button><iframe src="{url}" style="border:none;width:100%;height:420px;display:none;"></iframe>"#),
                );
                self.transport_ctx.execute_result(map);
            }
            _ => {
                #[cfg(not)]
                if eval_output.getattr("figure").is_ok() {
                    self.send_repr_to_execute_result_for_figure_object(eval_output);

                    let graphic_obj = self
                        .python_defines
                        .func_cvt_figs_to_graphic_obj
                        .call1(self.py, (vec![eval_output],))
                        .unwrap()
                        .into_ref(self.py);
                    return self.handle_matplotlib_output(graphic_obj);
                }
                self.handle_mime_output(eval_output);
            }
        }
    }

    #[cfg(not)]
    fn handle_mpl_fig_list(&self, eval_output: &pyo3::PyAny) {
        // handle adtk or matplotlib plot
        if let Ok(first_item) = eval_output.get_item::<u32>(0) {
            if first_item.getattr("figure").is_ok() {
                // send repr to execute result first
                self.send_repr_to_execute_result_for_figure_object(eval_output);

                let graphic_obj = self
                    .python_defines
                    .func_cvt_figs_to_graphic_obj
                    .call1(self.py, (eval_output,))
                    .unwrap()
                    .into_ref(self.py);
                return self.handle_matplotlib_output(graphic_obj);
            }
        }
        self.handle_mime_output(eval_output);
    }

    #[cfg(not)]
    fn send_repr_to_execute_result_for_figure_object(&self, figure_obj: &pyo3::PyAny) {
        let mut data_map = std::collections::HashMap::new();
        data_map.insert(
            "text/plain".to_string(),
            figure_obj.repr().unwrap().to_string(),
        );
        self.transport_ctx.execute_result(data_map);
    }
}
