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
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::pkcs1v15::SigningKey;
use rsa::RsaPrivateKey;
use sha2::Sha256;
use signature::RandomizedSigner;
use signature::Signature;

fn main() {
    let private_key = read_priv_key();
    let license = generate_license();

    generate_pub_key_file(&private_key);
    generate_license_file(private_key, license);
}

fn read_priv_key() -> RsaPrivateKey {
    let priv_key_path = std::env::var("PRIV_KEY_PATH").expect("env PRIV_KEY_PATH not present");
    let priv_key_str = std::fs::read_to_string(&priv_key_path)
        .unwrap_or_else(|_| panic!("private key path read err {priv_key_path}"));
    <rsa::RsaPrivateKey as rsa::pkcs1::DecodeRsaPrivateKey>::from_pkcs1_pem(&priv_key_str)
        .expect("invalid rsa pem file format")
}

fn generate_license() -> License {
    let license_expire_days =
        std::env::var("LICENSE_EXPIRE_DAYS").expect("env LICENSE_EXPIRE_DAYS not present");
    let license_expire_days = license_expire_days
        .parse::<u64>()
        .expect("env LICENSE_EXPIRE_DAYS is not a number");
    let now_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let license_expire_timestamp = now_timestamp + license_expire_days * 24 * 3600;
    License {
        expire_timestamp: license_expire_timestamp as u32,
        version: IDP_VERSION.to_string(),
    }
}

fn generate_license_file(private_key: RsaPrivateKey, license: License) {
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = rand::thread_rng();
    let data = bincode::serialize(&license).expect("failed to serialize license");
    let signature = signing_key.sign_with_rng(&mut rng, &data);

    let license_file = LicenseFile {
        license: hex::encode(&data),
        signature: hex::encode(signature.as_bytes()),
    };
    let file_content = bincode::serialize(&license_file).expect("failed to serialize license");
    let license_path = std::env::var("LICENSE_PATH").expect("env LICENSE_PATH not present");
    std::fs::write(&license_path, &file_content).expect("failed to write license file");
    println!("License generate succeed. Path: {license_path}");
}

#[allow(clippy::expect_fun_call)]
fn generate_pub_key_file(private_key: &RsaPrivateKey) {
    let public_key = private_key.to_public_key();
    let pub_key_str = public_key
        .to_pkcs1_pem(rsa::pkcs1::LineEnding::CRLF)
        .expect("failed to encode public key to pem file");
    let pub_key_path = std::env::var("PUB_KEY_PATH").expect("env PUB_KEY_PATH not present");
    std::fs::write(&pub_key_path, &pub_key_str)
        .expect(&format!("public key write err {pub_key_path}"));
    println!("Public key generate succeed. Path: {pub_key_path}");
}
