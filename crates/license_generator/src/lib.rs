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

use rsa::pkcs1v15::VerifyingKey;
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
    /// license struct bincode hexed string
    pub license: String,
    /// SigningKey::<Sha256>::new(private_key).sign_with_rng
    pub signature: String,
}

pub fn verify_license(pub_key_path: &str, license_path: &str) -> Result<License, String> {
    let serialized_license_file = match std::fs::read(license_path) {
        Ok(serialized_license_file) => serialized_license_file,
        Err(err) => {
            return Err(format!(
                "Failed to read license file from path {license_path} {err}"
            ));
        }
    };

    let license_file: LicenseFile = match bincode::deserialize(&serialized_license_file) {
        Ok(license_file) => license_file,
        Err(err) => return Err(format!("Invalid license file format {err}")),
    };

    let hexed_signature = match hex::decode(license_file.signature) {
        Ok(hexed_signature) => hexed_signature,
        Err(_) => return Err("hex::decode fail, Invalid signature format.".to_string()),
    };

    let signature = match Signature::from_bytes(&hexed_signature) {
        Ok(signature) => signature,
        Err(_) => return Err("Signature::from_bytes fail, Invalid signature format.".to_string()),
    };

    let license_struct_bytes = match hex::decode(license_file.license) {
        Ok(hexed_license) => hexed_license,
        Err(_) => return Err("hex::decode fail2, Invalid license format.".to_string()),
    };

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

    let license: License = match bincode::deserialize(&license_struct_bytes) {
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
    match get_timestamp_from_internet_() {
        Ok(rsp) => rsp,
        Err(_) => std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

fn get_timestamp_from_internet_() -> Result<u64, ureq::Error> {
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
    let internet_timestamp = get_timestamp_from_internet_().unwrap();
    let system_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(system_timestamp - internet_timestamp <= 3);
}
