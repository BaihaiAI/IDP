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

#[derive(Serialize, Deserialize)]
pub struct License {
    pub expire_timestamp: u32,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct LicenseFile {
    pub license: String,
    pub signature: String,
}

pub fn verify_license(pub_key_path: &str, license_path: &str) -> Result<(), &'static str> {
    let serialized_license_file = match std::fs::read(license_path) {
        Ok(serialized_license_file) => serialized_license_file,
        Err(_) => return Err("Failed to read license file from specified path."),
    };

    let license_file: LicenseFile = match bincode::deserialize(&serialized_license_file) {
        Ok(license_file) => license_file,
        Err(_) => return Err("Invalid license file format."),
    };

    let hexed_signature = match hex::decode(license_file.signature) {
        Ok(hexed_signature) => hexed_signature,
        Err(_) => return Err("Invalid signature format."),
    };

    let signature = match Signature::from_bytes(&hexed_signature) {
        Ok(signature) => signature,
        Err(_) => return Err("Invalid signature format."),
    };

    let hexed_license = match hex::decode(license_file.license) {
        Ok(hexed_license) => hexed_license,
        Err(_) => return Err("Invalid license format."),
    };

    let pub_key_str = match std::fs::read_to_string(pub_key_path) {
        Ok(pub_key_str) => pub_key_str,
        Err(_) => return Err("Failed to read public key file from specified path."),
    };

    let public_key =
        match <rsa::RsaPublicKey as rsa::pkcs1::DecodeRsaPublicKey>::from_pkcs1_pem(&pub_key_str) {
            Ok(public_key) => public_key,
            Err(_) => return Err("Invalid public key file format."),
        };

    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    if verifying_key.verify(&hexed_license, &signature).is_err() {
        return Err("Signature verify fail.");
    }

    let license: License = match bincode::deserialize(&hexed_license) {
        Ok(license) => license,
        Err(_) => return Err("Invalid license format."),
    };

    let expire_timestamp = license.expire_timestamp;
    let now_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if expire_timestamp < now_timestamp as u32 {
        return Err("License expired.");
    }

    let version = license.version;
    if version != IDP_VERSION {
        return Err("License version wrong");
    }

    Ok(())
}
