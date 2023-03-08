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

use base64::engine::Engine;
use base64::prelude::BASE64_STANDARD;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs1v15::VerifyingKey;
use rsa::signature;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use signature::Signature;
use signature::Verifier;

pub const IDP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_LICENSE_PATH: &str = "/opt/config/idp_license";
pub const DEFAULT_LICENSE_PUBLIC_KEY_PATH: &str = "/opt/config/idp_license_public_key.pem";

#[derive(Serialize, Deserialize)]
pub struct License {
    pub expire_timestamp: u64,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct LicenseFile {
    /// license struct bincode serialize
    pub license_struct_bytes: Vec<u8>,
    /// SigningKey::<Sha256>::new(private_key).sign_with_rng
    pub signature: Vec<u8>,
}

pub fn verify_license(pub_key_path: &str, license_path: &str) -> Result<License, String> {
    let base64_encoded_license_content = match std::fs::read(license_path) {
        Ok(base64_encoded_license_content) => base64_encoded_license_content,
        Err(err) => {
            return Err(format!(
                "Failed to read license file from path {license_path} {err}"
            ));
        }
    };
    let serialized_license_content = match BASE64_STANDARD.decode(base64_encoded_license_content) {
        Ok(serialized_license_content) => serialized_license_content,
        Err(err) => return Err(format!("base64::decode {err}")),
    };
    let license_file = match bincode::deserialize::<LicenseFile>(&serialized_license_content) {
        Ok(license_file) => license_file,
        Err(err) => return Err(format!("bincode::deserialize {err}")),
    };

    let signature = match Signature::from_bytes(&license_file.signature) {
        Ok(signature) => signature,
        Err(err) => return Err(format!("Signature::from_bytes {err}")),
    };
    let license_struct_bytes = license_file.license_struct_bytes;

    let pub_key_str = match std::fs::read_to_string(pub_key_path) {
        Ok(pub_key_str) => pub_key_str,
        Err(_) => return Err("Failed to read public key file from specified path.".to_string()),
    };
    let public_key =
        match <rsa::RsaPublicKey as rsa::pkcs1::DecodeRsaPublicKey>::from_pkcs1_pem(&pub_key_str) {
            Ok(public_key) => public_key,
            Err(err) => return Err(format!("Invalid public key file format {err}")),
        };
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    if verifying_key
        .verify(&license_struct_bytes, &signature)
        .is_err()
    {
        return Err("Signature verify fail.".to_string());
    }

    let license = match bincode::deserialize::<License>(&license_struct_bytes) {
        Ok(license) => license,
        Err(_) => return Err("bincode::deserialize fail, Invalid license format.".to_string()),
    };
    let now_timestamp = get_timestamp_from_internet();
    let expire_timestamp = license.expire_timestamp;
    if now_timestamp >= expire_timestamp {
        return Err("License expired.".to_string());
    }
    if license.version != IDP_VERSION {
        return Err("License version wrong".to_string());
    }

    Ok(license)
}

pub fn get_timestamp_from_internet() -> u64 {
    // default timeout 30s
    match get_timestamp_from_internet_inner() {
        Ok(rsp) => rsp,
        Err(_) => std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

fn get_timestamp_from_internet_inner() -> Result<u64, ureq::Error> {
    #[derive(serde::Deserialize)]
    struct TimestampApiRsp {
        unixtime: u64,
    }
    let rsp = ureq::get("http://worldtimeapi.org/api/timezone/Asia/Shanghai")
        .timeout(std::time::Duration::from_secs(3))
        .call()?
        .into_json::<TimestampApiRsp>()?;
    Ok(rsp.unixtime)
}

#[test]
fn test_get_timestamp_from_internet_() {
    let internet_timestamp = get_timestamp_from_internet_inner().unwrap();
    let system_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(system_timestamp - internet_timestamp <= 3);
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

pub fn generate_license_file(
    private_key: rsa::RsaPrivateKey,
    license_expire_days: u64,
    license_output_path: String,
) {
    use signature::RandomizedSigner;
    let license = generate_license(license_expire_days);
    let data = bincode::serialize(&license).unwrap();

    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = rand::thread_rng();
    let signature = signing_key.sign_with_rng(&mut rng, &data);

    let license_file = LicenseFile {
        license_struct_bytes: data,
        signature: signature.as_bytes().to_vec(),
    };
    let file_content = bincode::serialize(&license_file).expect("failed to serialize license");
    let file_content_base64 = BASE64_STANDARD.encode(file_content);
    let license_path = license_output_path;
    std::fs::write(license_path, file_content_base64).expect("failed to write license file");
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
