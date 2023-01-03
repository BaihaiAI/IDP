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

use license_generator::License;
use license_generator::LicenseFile;
use license_generator::IDP_VERSION;
use rsa::pkcs1v15::SigningKey;
use rsa::RsaPrivateKey;
use sha2::Sha256;
use signature::RandomizedSigner;
use signature::Signature;

#[derive(clap::Parser)]
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

    let license = generate_license(args.expire_in_days);

    // generate_pub_key_file(&private_key);
    generate_license_file(private_key, license, args.output);
}

fn generate_license(license_expire_days: u64) -> License {
    let now_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let license_expire_timestamp = now_timestamp + license_expire_days * 24 * 3600;
    License {
        expire_timestamp: license_expire_timestamp,
        version: IDP_VERSION.to_string(),
    }
}

fn generate_license_file(
    private_key: RsaPrivateKey,
    license: License,
    license_output_path: String,
) {
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = rand::thread_rng();
    let data = bincode::serialize(&license).expect("failed to serialize license");
    let signature = signing_key.sign_with_rng(&mut rng, &data);

    let license_file = LicenseFile {
        license: hex::encode(&data),
        signature: hex::encode(signature.as_bytes()),
    };
    let file_content = bincode::serialize(&license_file).expect("failed to serialize license");
    let license_path = license_output_path;
    std::fs::write(&license_path, &file_content).expect("failed to write license file");
}

#[cfg(not)]
#[allow(clippy::expect_fun_call)]
fn generate_pub_key_file(private_key: &RsaPrivateKey) {
    use rsa::pkcs1::EncodeRsaPublicKey;
    let public_key = private_key.to_public_key();
    let pub_key_str = public_key
        .to_pkcs1_pem(rsa::pkcs1::LineEnding::CRLF)
        .expect("failed to encode public key to pem file");
    let pub_key_path = std::env::var("PUB_KEY_PATH").expect("env PUB_KEY_PATH not present");
    std::fs::write(&pub_key_path, &pub_key_str)
        .expect(&format!("public key write err {pub_key_path}"));
    println!("Public key generate succeed. Path: {pub_key_path}");
}
