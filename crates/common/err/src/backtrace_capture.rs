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

pub fn backtrace_capture() -> Vec<String> {
    let mut frames = Vec::new();
    backtrace::trace(|frame| {
        // let ip = frame.ip();
        // let symbol_address = frame.symbol_address();

        // Resolve this instruction pointer to a symbol name
        backtrace::resolve_frame(frame, |symbol| {
            if let Some(line) = (|| {
                let filename = symbol.filename()?.to_str()?;
                if filename.starts_with("/rustc") {
                    return None;
                }
                // if filename.contains("registry") {
                //     return None;
                // }
                Some(format!(
                    "{filename}:{}",
                    symbol.lineno().unwrap_or_default()
                ))
            })() {
                frames.push(line);
            }
        });

        frames.len() <= 10
    });
    // if frames.len() > 10 {
    //     frames = frames[3..10].to_vec();
    // }
    frames
}
