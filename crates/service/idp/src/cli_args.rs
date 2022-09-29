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

#[derive(clap::Parser, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
#[clap(version = env!("VERSION"))]
pub struct CliArgs {
    #[clap(long, value_parser, value_name = "IP")]
    pub listen_addr: Option<std::net::Ipv4Addr>,
    #[clap(long, value_parser, value_name = "PORT", default_value = "3000")]
    pub gateway_port: u16,
    #[clap(long, value_parser, value_name = "PORT", default_value = "8089")]
    pub terminal_port: u16,
    #[clap(long, value_parser, value_name = "PORT", default_value = "7777")]
    pub lsp_port: u16,
    // #[clap(long, value_parser, value_name = "PORT", default_value = "9240")]
    // pub submitter_port: u16,
    #[clap(long, value_parser, value_name = "PORT", default_value = "16379")]
    redis_port: u16,
    #[clap(long, value_parser, value_name = "PORT", default_value = "8082")]
    pub note_storage_port: u16,
    #[clap(long, value_parser, value_name = "PORT", default_value = "9007")]
    pub kernel_manage_port: u16,
}

impl CliArgs {
    pub fn write_env(&self) {
        fn inner(env_key: &str, val: u16) {
            if std::env::var(env_key).is_err() {
                std::env::set_var(env_key, val.to_string());
            }
        }

        // inner("SUBMITTER_PORT", self.submitter_port);
        inner("IDP_REDIS_PORT", self.redis_port);
        inner("NOTE_STORAGE_PORT", self.note_storage_port);
        inner("KERNEL_MANAGE_PORT", self.kernel_manage_port);
    }
}

#[cfg(test)]
impl Default for CliArgs {
    fn default() -> Self {
        Self {
            gateway_port: 3000,
            terminal_port: 8089,
            lsp_port: 7777,
            // submitter_port: 9240,
            redis_port: 16379,
            note_storage_port: 8082,
            kernel_manage_port: 9007,
        }
    }
}

#[test]
fn test_cli_args() {
    let args = <CliArgs as clap::Parser>::parse_from(["binary_name"]);
    assert_eq!(args, CliArgs::default());
}
