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

use std::path::Path;

use cache_io::CacheService;
use common_model::enums::mime::Mimetype;
use err::ErrorTrace;
use tokio::io::AsyncReadExt;
use tracing::debug;
use tracing::error;

use super::CatRspBody;

/**
# tree_magic diff to file --mime-type

||file|tree_magic|
|---|---|---|
|ipynb|text/plain|application/x-ipynb+json|
|webp|image/webp|application/x-riff|
|empty file|inode/x-empty|text/plain|
|core dump|application/x-coredump|application/x-core|
|*.sh|text/plain|application/x-shellscript|
*/
pub fn get_mime_type<P: AsRef<Path>>(path: P) -> Result<String, ErrorTrace> {
    let path = path.as_ref();
    let meta = std::fs::metadata(path)?;
    if meta.is_dir() {
        let err = format!("path: {:?} is dir, not a file", &path);
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }
    Ok(tree_magic::from_filepath(path))
}

pub async fn cat_file_content_by_mime<P: AsRef<Path>>(
    path: P,
    mime_type_str: &str,
    project_id: u64,
    inode: u64,
    redis_cache: &CacheService,
) -> Result<CatRspBody, ErrorTrace> {
    let path = path.as_ref();
    if mime_type_str.starts_with("image") || mime_type_str == "application/x-riff" {
        let mut buf = Vec::new();
        let mut f = tokio::fs::File::open(&path).await?;
        f.read_to_end(&mut buf).await?;
        return Ok(CatRspBody::Text(base64::encode(buf)));
    }
    let file_ext = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };
    if mime_type_str == "application/zip" {
        if file_ext.starts_with("doc") || file_ext.starts_with("xls") || file_ext.starts_with("ppt")
        {
            return Err(
                ErrorTrace::new(&format!("this file ext {file_ext} not support"))
                    .code(ErrorTrace::CODE_WARNING),
            );
        }
        return Ok(CatRspBody::Zip(
            super::get_zip_file_list::preview_zip_file_list(&path.to_path_buf())?,
        ));
    }
    if mime_type_str == "application/gzip" {
        /*
        if file_ext.starts_with("doc") || file_ext.starts_with("xls") || file_ext.starts_with("ppt")
        {
            return Err(
                ErrorTrace::new(&format!("this file ext {file_ext} not support"))
                    .code(ErrorTrace::CODE_WARNING),
            );
        }
        */
        return Ok(CatRspBody::Zip(
            super::get_zip_file_list::preview_gzip_file_list(&path.to_path_buf())?,
        ));
    }

    if mime_type_str == "application/x-ipynb+json" || file_ext == "ipynb" || file_ext == "idpnb" {
        let mut notebook = redis_cache.read_notebook(&path, project_id).await?;
        notebook.set_inode(inode);
        return Ok(CatRspBody::Notebook(notebook));
    };

    if std::fs::metadata(path)?.len() > 10 * 1024 * 1024 {
        return Err(ErrorTrace::new("file too large(>10 MB)").code(ErrorTrace::CODE_WARNING));
    }
    // if file not a ipynb, we assume it's a text, otherwise read to UTF-8 garbled binary
    /*
    if mime_type_str.starts_with("text")
        || mime_type_str.contains("application/csv")
        || mime_type_str.contains("application/xml")
        || mime_type_str.contains("application/x-shellscript")
        || mime_type_str.contains("json")
    */
    let mut buf = Vec::new();
    let mut f = std::fs::File::open(path)?;
    std::io::Read::read_to_end(&mut f, &mut buf)?;
    match String::from_utf8(buf.clone()) {
        Ok(text) => Ok(CatRspBody::Text(text)),
        Err(_) => {
            // gb18030 is superset of gbk contains all chinese char
            let (text, _encoding, has_error) = encoding_rs::GB18030.decode(&buf);
            if has_error {
                Err(
                    ErrorTrace::new(&format!("unsupported mimetype {mime_type_str}"))
                        .code(ErrorTrace::CODE_WARNING),
                )
            } else {
                Ok(CatRspBody::Text(text.to_string()))
            }
        }
    }
}

#[cfg(test)]
fn get_mime_from_path_by_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut cmd = std::process::Command::new("file");
    cmd.arg("-P")
        .arg("bytes=300")
        .arg("--mime-type")
        .arg("--brief")
        .arg(path.as_ref());
    tracing::debug!("cmd = {cmd:?}");
    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = unsafe { String::from_utf8_unchecked(output.stderr) };
        error!("find_mimetype command error {}", stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "file --mime-type error",
        ));
    }
    let stdout = unsafe { String::from_utf8_unchecked(output.stdout) };

    Ok(stdout.trim_end().to_string())
}

#[test]
fn test_file_mime_type() {
    let test_cases_dir = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/test_cases"));
    for (filename, wanted_mime_type) in [
        ("1.webp", Mimetype::Image),
        ("empty.sh", Mimetype::Text),
        ("bash.sh", Mimetype::Text),
        ("1.ipynb", Mimetype::Notebook),
        ("bounding-box.idpnb", Mimetype::Notebook),
        ("image-augmentation.idpnb", Mimetype::Notebook),
        ("image-augmentation.json", Mimetype::Notebook),
        ("invalid.idpnb", Mimetype::Notebook),
    ] {
        let path = test_cases_dir.join(filename);
        assert!(path.exists(), "{path:?} not exist");
        println!(
            "- {filename} {}",
            get_mime_from_path_by_file(&path).unwrap()
        );
        let mime_type = find_mimetype(path).unwrap().0;
        assert_eq!(mime_type, wanted_mime_type);
    }
}

// #[deprecated]
pub fn find_mimetype<P: AsRef<Path>>(path: P) -> Result<(Mimetype, String), ErrorTrace> {
    let path = path.as_ref();
    debug!("find_mimetype path:{:?}", &path);
    let meta = std::fs::metadata(path)?;
    if meta.is_dir() {
        let err = format!("path: {:?} is not a file", &path);
        error!("{err}");
        return Err(ErrorTrace::new(&err));
    }

    let mime_type_str = tree_magic::from_filepath(path);
    tracing::debug!("{path:?} mine_type_str = {mime_type_str}");

    // if mime_type_str.eq("inode/x-empty") {
    //     return Ok((mimetype, "text/plain".to_string()));
    // }

    if mime_type_str.starts_with("image") || mime_type_str == "application/x-riff" {
        return Ok((Mimetype::Image, mime_type_str));
    }

    // if mime_type_str == "application/zip" {
    //     return Ok((Mimetype::Zip("application/zip"), mime_type_str));
    // }
    // if mime_type_str == "application/gzip" {
    //     return Ok((Mimetype::Zip("application/gzip"), mime_type_str));
    // }

    let file_ext = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };
    if mime_type_str == "application/x-ipynb+json" || file_ext == "ipynb" || file_ext == "idpnb" {
        return Ok((Mimetype::Notebook, mime_type_str));
    };

    if mime_type_str.starts_with("text")
        || mime_type_str.contains("application/csv")
        || mime_type_str.contains("application/x-shellscript")
        || mime_type_str.contains("json")
    {
        if meta.len() > 10 * 1024 * 1024 {
            return Err(
                ErrorTrace::new("text file too large(>10 MB)").code(ErrorTrace::CODE_WARNING)
            );
        }
        return Ok((Mimetype::Text, mime_type_str));
    }

    Err(
        ErrorTrace::new(&format!("unsupported mimetype {mime_type_str}"))
            .code(ErrorTrace::CODE_WARNING),
    )
}
