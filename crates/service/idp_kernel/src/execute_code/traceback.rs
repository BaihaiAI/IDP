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

//! IPython/core/interactiveshell.py:3319 self.showbacktrace()

/*
reproduce step: run input() code and interrupt


2022-08-02T08:55:47.790457Z  INFO idp_kernel: 151: recv  InterruptRequest
2022-08-02T08:55:47.790523Z  INFO idp_kernel: 159: after InterruptRequest
[crates/service/idp_kernel/src/execute_code/traceback.rs:10] ename = "EOFError"
thread 'main' panicked at 'a Display implementation returned an error unexpectedly: Error', /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/alloc/src/string.rs:2490:14
stack backtrace:
   0: rust_begin_unwind
             at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/panicking.rs:142:14
   2: core::result::unwrap_failed
             at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/result.rs:1805:5
   3: core::result::Result<T,E>::expect
             at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/core/src/result.rs:1055:23
   4: <T as alloc::string::ToString>::to_string
             at /rustc/ddcbba036aee08f0709f98a92a342a278eae5c05/library/alloc/src/string.rs:2489:9
   5: idp_kernel::execute_code::traceback::convert_pyerr
             at /home/w/repos/baihai/idp-note/crates/service/idp_kernel/src/execute_code/traceback.rs:11:18
   6: idp_kernel::kernel_app::handle_execute_req_impl::<impl idp_kernel::kernel_app::KernelApp>::handle_execute_req
             at /home/w/repos/baihai/idp-note/crates/service/idp_kernel/src/kernel_app/handle_execute_req_impl.rs:88:27
   7: idp_kernel::kernel_app::main_loop::<impl idp_kernel::kernel_app::KernelApp>::handle_req
             at /home/w/repos/baihai/idp-note/crates/service/idp_kernel/src/kernel_app/main_loop.rs:20:9
*/
pub fn convert_pyerr(
    err: pyo3::PyErr,
    py: pyo3::Python,
    code: String,
    eval_part_lineno_offset: Option<usize>,
) -> kernel_common::content::Error {
    let ename = err.get_type(py).name().unwrap();
    let evalue = err.value(py).to_string();
    #[cfg(debug_assertions)]
    {
        tracing::info!("ename = {ename}, evalue = {evalue}");
    }
    let mut renderded_lines = match ename {
        "SyntaxError" | "IndentationError" => {
            let syntax_error = err
                .value(py)
                .cast_as::<pyo3::exceptions::PySyntaxError>()
                .unwrap();
            let lineno = syntax_error
                .getattr("lineno")
                .unwrap()
                .extract::<usize>()
                .unwrap();
            let offset = syntax_error
                .getattr("offset")
                .unwrap()
                .extract::<usize>()
                .unwrap();
            // let code = syntax_error
            //     .getattr("text")
            //     .unwrap()
            //     .extract::<String>()
            //     .unwrap();
            render_code_location(code, lineno, Some(offset))
        }
        _ => match err.traceback(py) {
            Some(traceback) => match traceback.format() {
                Ok(traceback) => {
                    render_runtime_error(traceback, ename, code, eval_part_lineno_offset)
                }
                Err(err) => {
                    tracing::error!("traceback.format() err: {err}");
                    Vec::new()
                }
            },
            None => {
                tracing::error!("err.traceback(py) is None");
                Vec::new()
            }
        },
    };
    renderded_lines.push(format!("\u{001b}[0;31m{}\u{001b}[0m:  {}", ename, evalue));
    #[cfg(debug_assertions)]
    {
        for line in &renderded_lines {
            println!("{}", line);
        }
    }
    kernel_common::content::Error {
        ename: ename.to_string(),
        evalue,
        traceback: renderded_lines,
    }
}

fn render_runtime_error(
    traceback: String,
    ename: &str,
    code: String,
    eval_part_lineno_offset: Option<usize>,
) -> Vec<String> {
    let traceback = traceback
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    #[cfg(debug_assertions)]
    {
        tracing::debug!("traceback = {traceback:#?}");
    }
    let last_line = match traceback.last() {
        Some(last_line) => last_line,
        None => {
            return traceback
                .into_iter()
                .map(|mut line| {
                    line.push('\n');
                    line
                })
                .collect();
        }
    };
    let last_line = match last_line.split(',').nth(1) {
        Some(last_line) => last_line,
        None => {
            return traceback
                .into_iter()
                .map(|mut line| {
                    line.push('\n');
                    line
                })
                .collect();
        }
    };
    let lineno = match last_line.trim_start_matches(" line ").parse::<usize>() {
        Ok(lineno) => {
            if let Some(eval_part_lineno_offset) = eval_part_lineno_offset {
                lineno + eval_part_lineno_offset
            } else {
                lineno
            }
        }
        Err(err) => {
            tracing::error!("last_line is `{last_line}`, parse lineno failed: {err}");
            return Vec::new();
        }
    };

    // render
    let mut rendered_lines = vec![format!(
        "\u{001b}[0;31m{}\u{001b}[0m                           Traceback (most recent call last)\n",
        ename
    )];
    rendered_lines.extend(render_code_location(code, lineno, None));
    rendered_lines
}

fn render_code_location(code: String, lineno: usize, col_opt: Option<usize>) -> Vec<String> {
    if cfg!(debug_assertions) {
        tracing::info!("code = {code}, lineno = {lineno}, col_opt = {col_opt:?}");
    }
    let mut rendered_lines = Vec::new();
    for (i, line) in code
        .lines()
        .into_iter()
        .enumerate()
        .skip((lineno - 1).saturating_sub(5 / 2))
        .take(5)
    {
        let cur_lineno = i + 1;
        if cur_lineno == lineno {
            rendered_lines.push(format!(
                "\u{001b}[0;32m----> {cur_lineno}\u{001b}[0m  \u{001b}[38;5;28;01m{line}\u{001b}[39;00m\n",
            ));
            if let Some(col) = col_opt {
                rendered_lines.push(format!(
                    "\u{001b}[0;32m{}^\u{001b}[0m\n",
                    " ".repeat(
                        col - 1 + "----> ".len() + cur_lineno.to_string().len() + "  ".len()
                    )
                ));
            }
        } else {
            rendered_lines.push(format!(
                "\u{001b}[0;32m      {cur_lineno}\u{001b}[0m  {line}\n",
            ));
        }
    }
    rendered_lines
}
