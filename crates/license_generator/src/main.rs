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

#[derive(clap::Parser)]
#[command(version)]
struct Args {
    // #[clap(long, value_parser, value_name = "PATH")]
    // pub_key: String,
    #[clap(long, value_parser, value_name = "PATH")]
    pri_key: String,
    #[clap(long, value_parser, value_name = "INT", default_value = "30")]
    expire_in_days: u64,
    #[clap(
        long,
        value_parser,
        value_name = "PATH",
        default_value = "license",
        help = "license file output path"
    )]
    output: String,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    let priv_key_path = args.pri_key;
    let priv_key_str = std::fs::read_to_string(&priv_key_path)
        .unwrap_or_else(|_| panic!("private key path read err {priv_key_path}"));
    let private_key =
        <rsa::RsaPrivateKey as rsa::pkcs1::DecodeRsaPrivateKey>::from_pkcs1_pem(&priv_key_str)
            .expect("invalid rsa pem file format");

    license_generator::generate_license_file(private_key, args.expire_in_days, args.output);
}
