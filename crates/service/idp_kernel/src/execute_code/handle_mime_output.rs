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

/*
>>> type(df_0)
<class 'pandas.core.frame.DataFrame'>
>>> type(type(df_0))
<class 'type'>
>>> type(df_0)._repr_html_()
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: _repr_html_() missing 1 required positional argument: 'self'
*/
fn handle_mime_output(eval_output: &pyo3::PyAny) -> HashMap<String, String> {
    const REPR_MIME_LIST: [(&str, &str); 9] = [
        ("_repr_png_", "image/png"),
        ("_repr_jpeg_", "image/jpeg"),
        ("_repr_html_", "text/html"),
        ("_repr_markdown_", "text/markdown"),
        ("_repr_svg_", "image/svg+xml"),
        ("_repr_latex_", "text/latex"),
        ("_repr_json_", "application/json"),
        ("_repr_javascript_", "application/javascript"),
        ("_repr_pdf_", "application/pdf"),
    ];

    let mut data_map = HashMap::new();
    // text/plain msg is from obj repr
    match eval_output.repr() {
        Ok(s) => {
            data_map.insert("text/plain".to_string(), s.to_string());
        }
        Err(err) => {
            tracing::error!("handle_mime_output panicked {err}");
            return data_map;
        }
    }

    for (attr, mimetype) in REPR_MIME_LIST {
        if eval_output.hasattr(attr).unwrap() {
            let func = eval_output.getattr(attr).unwrap();
            if let Ok(output) = func.call0() {
                let result_str = output.str().unwrap().to_string();
                data_map.insert(mimetype.to_string(), result_str);
            }
        }
    }
    data_map
}

impl<'py> super::execute_code_context::ExecuteCodeContext<'py> {
    pub fn handle_mime_output(&self, eval_output: &pyo3::PyAny) {
        let data_map = handle_mime_output(eval_output);
        self.transport_ctx.execute_result(data_map);
    }
}
