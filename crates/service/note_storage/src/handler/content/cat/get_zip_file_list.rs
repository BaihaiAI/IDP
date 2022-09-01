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

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use err::ErrorTrace;

#[derive(serde::Serialize, Debug)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct ZipNode {
    /// frontend ant design require a unique key for each tree node, here we must abs path
    pub absolute_path: String, //"/store/idp-note/projects/1/notebooks",
    pub file_name: String, //"notebooks",
    pub has_children: bool,
    // Axum resp model require Send+Sync, so we can't use Rc
    pub children: Vec<Arc<Mutex<ZipNode>>>,
    #[serde(skip)]
    pub is_top_level: bool,
}

impl Default for ZipNode {
    fn default() -> Self {
        Self {
            absolute_path: "".to_string(),
            file_name: "".to_string(),
            has_children: true,
            children: Vec::new(),
            is_top_level: false,
        }
    }
}

trait ZipTraverse {
    fn is_dir(&self) -> bool;
    fn path(&self) -> std::path::PathBuf;
}

impl ZipTraverse for String {
    fn is_dir(&self) -> bool {
        self.ends_with('/')
    }

    fn path(&self) -> std::path::PathBuf {
        std::path::Path::new(self).to_path_buf()
    }
}

/*
= note: `FnMut` closures only have access to their captured variables while they are executing...
= note: ...therefore, they cannot allow references to captured variables to escape
= note: requirement occurs because of the type `ZipFile<'_>`, which makes the generic argument `'_` invariant
= note: the struct `ZipFile<'a>` is invariant over the parameter `'a`
= help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

((0..zip.len()).map(|i| {
    let file = zip.by_index(i)?;
    Ok(Arc::new(file))
}))
*/
impl ZipTraverse for tar::Entry<'_, flate2::read::GzDecoder<std::fs::File>> {
    fn is_dir(&self) -> bool {
        self.header().entry_type().is_dir()
    }

    fn path(&self) -> std::path::PathBuf {
        self.path().unwrap().to_path_buf()
    }
}

pub fn preview_zip_file_list(path: &PathBuf) -> Result<Vec<ZipNode>, ErrorTrace> {
    let mut zip = zip::ZipArchive::new(std::fs::File::open(path)?)?;
    let mut iter = Vec::new();
    for i in 0..zip.len() {
        let index = zip.by_index(i)?;
        let name_raw = index.name_raw();
        let name = match String::from_utf8((name_raw).to_vec()) {
            Ok(name) => name,
            Err(_) => {
                let (text, _encoding, has_error) = encoding_rs::GB18030.decode(name_raw);
                if has_error {
                    String::from_utf8_lossy(name_raw).to_string()
                } else {
                    text.to_string()
                }
            }
        };
        iter.push(Ok(name));
    }
    // file_names is unordered
    // files_to_tree(zip.file_names().map(|x| Ok(x)))

    files_to_tree(iter)
}

pub fn preview_gzip_file_list(path: &PathBuf) -> Result<Vec<ZipNode>, ErrorTrace> {
    let gzip = flate2::read::GzDecoder::new(std::fs::File::open(path)?);
    let mut archive = tar::Archive::new(gzip);
    files_to_tree(archive.entries()?)
}

// TODO support non-dir multi single file zip
// if zip/gzip is a single file, then root element is not a dir
// debug_assert!(entry.is_dir());
fn files_to_tree<T: ZipTraverse>(
    entries: impl IntoIterator<Item = Result<T, std::io::Error>>,
) -> Result<Vec<ZipNode>, ErrorTrace> {
    let mut dirs_map = std::collections::HashMap::new();
    let mut top_level_leafs = Vec::new();

    for entry_res in entries {
        let entry = entry_res?;
        let path = &entry.path();
        let path_str = path.to_str().unwrap();

        let parent_dir = path.parent().unwrap().to_str().unwrap();
        let node = if entry.is_dir() {
            let dir_name = path_str.trim_end_matches('/');
            let node = Arc::new(Mutex::new(ZipNode {
                file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                absolute_path: path_str.to_string(),
                ..Default::default()
            }));
            dirs_map.insert(dir_name.to_string(), node.clone());
            node
        } else {
            Arc::new(Mutex::new(ZipNode {
                file_name: path.file_name().unwrap().to_str().unwrap().to_string(),
                absolute_path: path_str.to_string(),
                has_children: false,
                ..Default::default()
            }))
        };
        if let Some(dir) = dirs_map.get(parent_dir) {
            dir.lock().unwrap().children.push(node);
        } else {
            node.lock().unwrap().is_top_level = true;
            if !entry.is_dir() {
                top_level_leafs.push(node);
            }
        }
    }

    let top_level_dirs = dirs_map.into_iter().filter_map(|(_, node)| {
        let node_guard = node.lock().unwrap();
        if node_guard.is_top_level {
            drop(node_guard);
            Some(Arc::try_unwrap(node).unwrap().into_inner().unwrap())
        } else {
            None
        }
    });
    let top_level_files = top_level_leafs
        .into_iter()
        .map(|node| Arc::try_unwrap(node).unwrap().into_inner().unwrap());
    let ret = top_level_dirs.chain(top_level_files).collect();
    Ok(ret)
}

#[test]
fn test_zip_files_to_tree() {
    let nodes = preview_zip_file_list(
        &std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_cases/two_files_and_a_dir.zip"
        ))
        .to_path_buf(),
    )
    .unwrap();
    assert_eq!(
        nodes
            .into_iter()
            .map(|node| node.file_name)
            .collect::<Vec<_>>(),
        vec!["usr", "settings.json", "keybindings.json"]
    );
}

#[test]
fn test_tar_gz_files_to_tree() {
    let nodes = preview_gzip_file_list(
        &std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_cases/two_files_and_a_dir.tar.gz"
        ))
        .to_path_buf(),
    )
    .unwrap();
    assert_eq!(
        nodes
            .into_iter()
            .map(|node| node.file_name)
            .collect::<Vec<_>>(),
        vec!["usr", "settings.json", "keybindings.json"]
    );
}
