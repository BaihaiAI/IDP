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

pub mod escape_slash_from_frontend;
pub mod execute_code_context;
// pub(crate) mod handle_matplotlib_output;
mod handle_mime_output;
mod handle_output;
mod handle_plotly_output;
pub mod parse_last_expr;
pub(crate) mod traceback;

const PLOTLY_FILE: &str = "temp-plot.html";
