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

fn handle_matplotlib_output(graphic_obj: &pyo3::PyAny) -> Vec<HashMap<String, String>> {
    let graphic_obj_data = graphic_obj.getattr("data").unwrap();

    let display_outputs: Vec<(String, String)> = graphic_obj_data.extract().unwrap();

    let mut ret = Vec::new();
    for (text_plain, png_data) in display_outputs {
        let mut data_map = HashMap::new();
        data_map.insert("text/plain".to_string(), text_plain);
        // let mut output_repr_vec = vec![("text/plain".to_string(), text_plain)];
        data_map.insert("image/png".to_string(), png_data);
        ret.push(data_map);
    }
    ret
}

impl<'py> super::execute_code_context::ExecuteCodeContext<'py> {
    pub fn handle_matplotlib_output(&self, eval_output: &pyo3::PyAny) {
        let data_map_list = handle_matplotlib_output(eval_output);
        for data_map in data_map_list {
            self.transport_ctx.display_data(data_map);
        }
    }
}
