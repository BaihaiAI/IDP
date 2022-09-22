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
pub async fn write_large_to_nfs<P: AsRef<Path>>(
    path: P,
    content: String,
    file_type: Mimetype,
) -> Result<(), ErrorTrace> {
    let path = path.as_ref();
    tracing::debug!("write abs_path = {:?}", path);
    let write_bytes = match file_type {
        Mimetype::Image => base64::decode(content)?,
        _ => content.into_bytes(),
    };
    #[cfg(windows)]
    let file_name = path
        .to_str()
        .unwrap()
        .replace('/', "___")
        .replace('\\', "___")
        .replace(':', "___");
    #[cfg(unix)]
    let file_name = path.to_str().unwrap().replace('/', "___");
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
    write_large_to_nfs(path, serde_json::to_string(&notebook)?, Mimetype::Notebook).await
}

pub async fn read_notebook_from_disk<P: AsRef<Path>>(path: P) -> Result<Notebook, ErrorTrace> {
    let notebook_str = match tokio::fs::read_to_string(&path).await {
        Ok(str) => str,
        Err(_) => {
            let mut buf = Vec::new();
            let mut f = std::fs::File::open(&path)?;
            std::io::Read::read_to_end(&mut f, &mut buf)?;
            encoding_rs::GB18030.decode(&buf).0.to_string()
        }
    };
    let mut notebook = match serde_json::from_str::<Notebook>(&notebook_str) {
        Ok(notebook) => notebook,
        Err(_) => {
            return Err(
                ErrorTrace::new(&format!("invalid notebook format {:?}", path.as_ref()))
                    .code(ErrorTrace::CODE_WARNING),
            );
        }
    };

    //check notebook cell require field
    let mut index = 1.0;
    let cells = notebook
        .cells
        .into_iter()
        .map(|cell| {
            index += 1.0;
            let mut new_cell = cell;
            if new_cell.id() == None {
                let cell_id = common_model::entity::cell::Uuid::new_v4();
                tracing::warn!(
                    "this cell has no cell_id:{:?}, new uuid str:{}",
                    new_cell,
                    cell_id.to_string()
                );
                new_cell.set_id(cell_id);
            }
            new_cell.set_index(index);
            new_cell
        })
        .collect();

    notebook.cells = cells;

    Ok(notebook)
}

#[test]
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
