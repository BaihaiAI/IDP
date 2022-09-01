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

/// cpython C-API has breaking change between 3.9 and 3.10 so we use python ast module for compatibly
/**
```text
>>> print(ast.unparse(ast.parse('print(1);print(2)', mode='exec')))
print(1)
print(2)
>>> a=ast.parse('print(1);print(2)', mode='exec')
>>> a.body[1].col_offset
9
>>> a.body[0].end_col_offset
8
>>> a.body[0].col_offset
0
```
*/
type Span = (i32, i32, i32, i32, i32, i32);

impl<'py> super::execute_code_context::ExecuteCodeContext<'py> {
    pub fn split_code_to_exec_and_eval_part(
        &mut self,
    ) -> Result<(Option<String>, Option<String>), pyo3::PyErr> {
        let span = self
            .python_defines
            .func_ast_parse
            .call1(self.py, (&self.code,))?
            .extract::<Span>(self.py)?;
        Ok(split_code(
            &self.code,
            span,
            &mut self.eval_part_lieno_offset,
        ))
    }
}

/**
0. no code(all code is comment)
1. code only one expr
2. code only exec part
3. code exec part(e.g. func define) + one expr

eval_part use Option to make sure empty string also need eval
*/
fn split_code(
    code: &str,
    span: Span,
    eval_part_lineno_offset: &mut usize,
) -> (Option<String>, Option<String>) {
    // python lineno count from 1
    // python col offset count from 0
    let previous_stmt_end_lineno = span.0;
    let previous_stmt_end_col_offset = span.1;
    let last_expr_lineno = span.2;
    let last_expr_col_offset = span.3;
    let last_expr_end_lineno = span.4;
    let last_expr_end_col_offset = span.5;
    // #[cfg(debug_assertions)]
    // {
    //     println!("previous_stmt_end_lineno = {previous_stmt_end_lineno}");
    //     println!("previous_stmt_end_col_offset = {previous_stmt_end_col_offset}");
    //     println!("last_expr_lineno = {last_expr_lineno}");
    //     println!("last_expr_col_offset = {last_expr_col_offset}");
    //     println!("last_expr_end_lineno = {last_expr_end_lineno}");
    //     println!("last_expr_end_col_offset = {last_expr_end_col_offset}");
    // }

    // code is all comment
    if previous_stmt_end_lineno == -1 && last_expr_lineno == -1 {
        return (None, None);
    }

    // code is only stmt
    if last_expr_lineno == -1 {
        // eval part is None, must be situation `2. code only exec part`
        return (Some(code.to_string()), None);
    }

    // code is only one expr
    let no_exec_part = previous_stmt_end_lineno == -1;
    if no_exec_part {
        return (None, Some(code.to_string()));
    }

    // code is exec + eval
    let lines = code.split_inclusive('\n').collect::<Vec<_>>();
    // #[cfg(debug_assertions)]
    // {
    //     println!("{lines:#?}");
    // }
    let last_expr_lineno = last_expr_lineno as usize - 1;
    *eval_part_lineno_offset = last_expr_lineno;
    (
        Some(extract_code_from_lines(
            &lines,
            0,
            0,
            previous_stmt_end_lineno as usize - 1,
            previous_stmt_end_col_offset as usize,
        )),
        Some(extract_code_from_lines(
            &lines,
            last_expr_lineno,
            last_expr_col_offset as usize,
            last_expr_end_lineno as usize - 1,
            last_expr_end_col_offset as usize,
        )),
    )
}

fn extract_code_from_lines(
    lines: &[&str],
    lineno: usize,
    col_offset: usize,
    end_lineno: usize,
    end_col_offset: usize,
) -> String {
    #[cfg(debug_assertions)]
    {
        tracing::debug!("lineno, col_offset = {lineno}, {col_offset}");
        tracing::debug!("end_lineno, end_col_offset = {end_lineno}, {end_col_offset}");
    }
    if lineno == end_lineno {
        return lines[lineno]
            .chars()
            .skip(col_offset)
            .take(end_col_offset - col_offset)
            .collect::<String>();
    }
    let mut ret = lines[lineno].chars().skip(col_offset).collect::<String>();

    // middle part
    for line in lines
        .iter()
        .skip(lineno + 1)
        .take((end_lineno - lineno).saturating_sub(1))
    {
        ret.push_str(line);
    }

    if end_lineno > lineno {
        ret.push_str(
            &lines[end_lineno]
                .chars()
                .take(end_col_offset)
                .collect::<String>(),
        );
    }

    ret
}

#[test]
fn test_split_code() {
    let (exec_part, eval_part_opt) = split_code("print(1);print(2)", (1, 8, 1, 9, 1, 17), &mut 0);
    assert_eq!(exec_part.unwrap(), "print(1)");
    assert_eq!(eval_part_opt.unwrap(), "print(2)".to_string());
}
