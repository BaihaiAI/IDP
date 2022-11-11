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



// clap v3.2 change: https://github.com/clap-rs/clap/commit/e23800e10e104084a06e030b205b27620d230415
// now clap require value_parser
#[derive(clap::Parser, Debug, Clone)]
#[clap(version = env!("VERSION"))]
pub struct KernelArgs {
    #[clap(long, value_parser, value_name = "JSON")]
    pub header: String,
    // #[clap(long, value_parser, value_name = "INT")]
    // pub project_id: u64,
    // #[clap(long, value_parser, value_name = "INT")]
    // pub team_id: u64,
}

impl KernelArgs {
    pub fn to_header_by_cell_id(&self, cell_id: &str) -> kernel_common::Header {
        serde_json::from_str(self.h)
        // kernel_common::Header {
        //     project_id: self.project_id,
        //     team_id: self.team_id,
        //     cell_id: cell_id.to_string(),
        //     path: self.path.clone(),
        //     pipeline_opt: None,
        // }
    }
}
