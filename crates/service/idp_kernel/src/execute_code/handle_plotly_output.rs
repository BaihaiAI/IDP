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

use std::collections::HashMap;
use std::io::Error;

use crate::execute_code::PLOTLY_FILE;

fn handle_plotly_file_output() -> Result<HashMap<String, String>, Error> {
    let plotly_str = std::fs::read_to_string(PLOTLY_FILE)?;

    let mut data_map = HashMap::new();
    data_map.insert("text/html".to_string(), plotly_str);

    Ok(data_map)
}
fn handle_plotly_div_output(eval_output: &pyo3::PyAny) -> HashMap<String, String> {
    let str_result: String = eval_output.extract().unwrap();

    let mut data_map = HashMap::new();
    data_map.insert("text/html".to_string(), str_result);

    data_map
}

impl<'py> super::execute_code_context::ExecuteCodeContext<'py> {
    pub fn handle_plotly_file_output(&self) {
        tracing::info!("handle_plotly_output>>>");
        match handle_plotly_file_output() {
            Ok(data_map) => {
                self.transport_ctx.display_data(data_map);
            }
            Err(err) => {
                println!("handle_plotly_output error");
                dbg!(err);
            }
        }
        tracing::info!("<<<handle_plotly_output");
    }
    pub fn handle_plotly_div_output(&self, eval_output: &pyo3::PyAny) {
        let data_map = handle_plotly_div_output(eval_output);
        self.transport_ctx.display_data(data_map);
    }
}
