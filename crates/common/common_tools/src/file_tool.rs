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
use std::time::UNIX_EPOCH;

use common_model::entity::notebook::Notebook;
use common_model::enums::mime::Mimetype;
use err::ErrorTrace;

/// write large text file to NFS, prevent pod/process/NFS down while on writing and lose data
/// so we write to temp file on NFS first, and mv to target path on NFS
/// make sure 100% data write success
pub async fn write_large_to_nfs(
    path: &str,
    content: String,
    file_type: Mimetype,
) -> Result<(), ErrorTrace> {
    /*
    if path.ends_with("idpnb") || path.ends_with("ipynb") {
        let old_ipynb_size = tokio::fs::metadata(path).await?.len() as usize;
        let new_ipynb_size = content.as_bytes().len();
        if new_ipynb_size < old_ipynb_size {
            if new_ipynb_size * 4 < old_ipynb_size {
                tracing::warn!(
                    "ipynb size decrease >75%, size change {old_ipynb_size} to {new_ipynb_size}"
                );
            }
            if new_ipynb_size * 9 < old_ipynb_size {
                tracing::error!(
                    "panicked ipynb size decrease >90%, size change {old_ipynb_size} to {new_ipynb_size}"
                );
                return Err(ErrorTrace::new(
                    "ipynb size decrease >90%, maybe cell lose bug occur, forbid persist to disk",
                ));
            }
        }
    }
    */
    let write_bytes = match file_type {
        Mimetype::Image => base64::Engine::decode(&base64::prelude::BASE64_STANDARD, content)?,
        Mimetype::Notebook => {
            #[cfg(not)]
            if std::path::Path::new(path).exists() {
                // if update old file on fs, if create new file on fs doesn't need to check cell lose
                let old_notebook = read_notebook_from_disk(path).await?;
                let old_num_cells = old_notebook.cells.len();
                if num_cells < old_num_cells && old_num_cells - num_cells >= 2 {
                    tracing::error!(
                        "panicked ipynb cell lose, num_cells change {old_num_cells} to {num_cells}"
                    );
                    return Err(ErrorTrace::new("panicked ipynb cell lose"));
                }
            }
            content.into_bytes()
        }
        _ => content.into_bytes(),
    };
    #[cfg(windows)]
    let file_name = path
        .replace('/', "___")
        .replace('\\', "___")
        .replace(':', "___");
    #[cfg(unix)]
    let file_name = path.replace('/', "___");
    let timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let dir = business::path_tool::store_parent_dir();
    let tmp = format!(
        "{}/store/tmp/{:?}{}",
        dir.to_str().unwrap(),
        timestamp,
        file_name
    );
    let tmp_path = std::path::Path::new(&tmp);
    tracing::debug!("write nfs_path = {:?}", tmp_path);

    tokio::fs::write(&tmp_path, &write_bytes).await?;
    tokio::fs::rename(tmp_path, path).await?;
    Ok(())
}

pub async fn write_notebook_to_disk<P: AsRef<Path>>(
    path: P,
    notebook: &Notebook,
) -> Result<(), ErrorTrace> {
    write_large_to_nfs(
        path.as_ref().to_str().unwrap(),
        serde_json::to_string_pretty(&notebook)?,
        Mimetype::Notebook,
    )
    .await
}

pub async fn read_notebook_from_disk(abs_path: &str) -> Result<Notebook, ErrorTrace> {
    let notebook_str = match tokio::fs::read_to_string(abs_path).await {
        Ok(str) => str,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::InvalidData {
                // 99% file is utf-8, if gpk we read twice no performance lose
                encoding_rs::GB18030
                    .decode(&tokio::fs::read(abs_path).await?)
                    .0
                    .to_string()
            } else {
                return Err(ErrorTrace::new(&format!("{abs_path} {err}")));
            }
        }
    };
    let notebook = match serde_json::from_str::<Notebook>(&notebook_str) {
        Ok(notebook) => notebook,
        Err(_) => {
            return Err(
                ErrorTrace::new(&format!("invalid notebook format, path={abs_path}"))
                    .code(ErrorTrace::CODE_WARNING),
            );
        }
    };

    // if notebook.cells.is_empty() {
    //     return Err(ErrorTrace::new(&format!("empty cells {abs_path}")));
    // }

    // add index/cell_id to all cells before fs->redis
    // first cell index can't be 0
    let cells = (1usize..)
        .zip(notebook.cells.into_iter())
        .map(|(index, mut cell)| {
            if cell.id().is_none() {
                let cell_id = common_model::entity::cell::Uuid::new_v4();
                tracing::warn!(
                    "this cell has no cell_id:{:?}, new uuid str:{}",
                    cell,
                    cell_id.to_string()
                );
                cell.set_id(cell_id);
            }
            cell.set_index(index as f64);
            cell
        })
        .collect();

    Ok(Notebook::new(cells))
}

#[test]
#[cfg(not)]
fn test_write() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    println!("{:?}", timestamp);
    // let a = std::env::current_exe().unwrap();
    let a = std::path::Path::new("/home/foo.ipynb");
    let file_name = a.display().to_string().replace('/', "___");
    let b = format!("{:?}{:?}", timestamp, file_name).replace('"', "");
    print!("{:?}", b)
}
