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

use std::fs;
use std::path::PathBuf;

use axum::Json;
use common_model::Rsp;
use err::ErrorTrace;
use tracing::info;

use crate::handler::content::cat::file_mime_magic::get_mime_type;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecompressReq {
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: u64,
    pub path: String,
    pub extract_to: Option<String>,
}

/// if extract_to exist, would overwrite
pub async fn unzip(
    Json(DecompressReq {
        team_id,
        project_id,
        path,
        extract_to,
    }): Json<DecompressReq>,
) -> Result<Rsp<()>, ErrorTrace> {
    let abs_path = business::path_tool::get_store_full_path(team_id, project_id, &path);
    let extract_to = match extract_to {
        Some(extract_to) => {
            let extract_to =
                business::path_tool::get_store_full_path(team_id, project_id, &extract_to);
            let meta = extract_to.metadata()?;
            if meta.is_dir() {
                return Err(ErrorTrace::new("extractTo not a dir "));
            }
            extract_to
        }
        None => abs_path.parent().unwrap().to_path_buf(),
    };
    let mime = get_mime_type(&abs_path)?;
    if mime == "application/zip" {
        extract_zip(abs_path, extract_to)?;
        return Ok(Rsp::success(()));
    }
    if mime == "application/gzip" {
        extract_gzip(abs_path, extract_to)?;
        return Ok(Rsp::success(()));
    }
    Err(ErrorTrace::new("not a zip archive").code(ErrorTrace::CODE_WARNING))
}

fn extract_zip(abs_path: PathBuf, extract_to: PathBuf) -> Result<(), ErrorTrace> {
    info!("--> extract_zip: abs_path={abs_path:?}");
    let f = fs::File::open(abs_path)?;
    let mut archive = zip::ZipArchive::new(f)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_to.join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                info!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            info!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn extract_gzip(abs_path: PathBuf, extract_to: PathBuf) -> Result<(), ErrorTrace> {
    let mut ar = tar::Archive::new(flate2::read::GzDecoder::new(std::fs::File::open(abs_path)?));
    for file in ar.entries()? {
        let mut file = file?;
        let outpath = extract_to.join(file.path()?);
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(&p)?;
            }
        }
        let mut outfile = fs::File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;
    }
    Ok(())
}

#[test]
#[ignore]
fn test_extract_gzip() {
    use std::path::Path;
    extract_gzip(
        Path::new("/home/w/Downloads/newFile.idpnb.tar.gz").to_path_buf(),
        Path::new("/home/w/Downloads").to_path_buf(),
    )
    .unwrap();
}
