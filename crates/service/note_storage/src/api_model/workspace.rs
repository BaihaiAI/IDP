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

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRenameReq {
    pub path: String,
    pub source: String,
    pub dest: String,
    pub project_id: u64,
    pub auto_close: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUploadReq {
    pub path: String,
    pub team_id: String,
    pub user_id: String,
    pub project_id: u64,
    pub model_name: String,
    pub version: String,
    pub intro: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMove {
    pub origin_path: String,
    pub target_path: String,
    pub project_id: u64,
    pub auto_close: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFile {
    pub path: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub dest: Option<String>,
    pub project_id: u64,
    pub output_type: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspacePathRto {
    pub path: String,
    pub project_id: u64,
    pub team_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirLazyLoadPara {
    pub path: Vec<String>,
    pub team_id: String,
    pub project_id: u64,
    pub only_pipeline_support: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirSearchPara {
    pub project_id: u64,
    pub keyword: String,
    pub only_pipeline_support: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalKeywordSearchPara {
    pub project_id: u64,
    pub keyword: String,
}

// TODO(code_review): diff to FullFileTreeNode?
#[derive(Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileTreeNode {
    // #[serde(skip)]
    pub absolute_path: String,
    pub browser_path: String,
    pub project_id: String,
    pub file_name: String,
    /// "DIRECTORY" or "FILE"
    // pub file_type: &'static str,
    pub file_type: String,
    pub has_children: bool,
    pub children: Vec<FileTreeNode>,

    // extra fields diff to FullFileTreeNode
    /// "amazon s3" or "postgresql"
    pub source_type: String,
    pub bucket: String,
    pub end_point: String,
    pub active: bool,
    pub contains_keywords: bool,
}

impl Default for FileTreeNode {
    fn default() -> Self {
        FileTreeNode {
            absolute_path: "".to_string(),
            browser_path: "".to_string(),
            project_id: 0.to_string(),
            file_name: "untitiled".to_string(),
            file_type: "FILE".to_string(),
            source_type: "".to_string(),
            bucket: "".to_string(),
            end_point: "".to_string(),
            active: false,
            has_children: false,
            children: vec![],
            contains_keywords: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathPojo {
    pub path: String,
    pub sort_path: String,
}
impl PathPojo {
    pub fn new(path: String, sort_path: String) -> Self {
        PathPojo { path, sort_path }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PathBufPojo {
    pub path: PathBuf,
    pub short_path: String,
    pub filename: String,
    pub sort_path: String,
}
impl PathBufPojo {
    pub fn new(path: PathBuf, short_path: String, filename: String, sort_path: String) -> Self {
        PathBufPojo {
            path,
            short_path,
            filename,
            sort_path,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSearchResult {
    pub absolute_path: String, //"/store/idp-note/projects/1/notebooks",
    pub browser_path: String,  // "",
    pub project_id: String,    //"1",
    pub file_name: String,     //"notebooks",
    pub cell_id: String,
    pub text: String,
    pub line: u32,
}

#[derive(Serialize)]
pub struct UploadFileDataRet {
    pub code: u32,
    pub message: String,
    pub data: String,
}
#[cfg(not)]
impl UploadFileDataRet {
    pub fn new(code: u32, message: String, data: String) -> Self {
        UploadFileDataRet {
            code,
            message,
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSourceObj {
    pub alias: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub datasource: String,
    pub path: String,
    pub dbname: String,
    #[serde(default)]
    pub active: bool,
}
impl DataSourceObj {
    pub fn new(
        alias: String,
        type_: String,
        datasource: String,
        path: String,
        dbname: String,
        active: bool,
    ) -> Self {
        DataSourceObj {
            alias,
            type_,
            datasource,
            path,
            dbname,
            active,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataSourceRet {
    pub code: u32,
    pub message: String,
    pub data: DataObj,
}

// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd,Serialize,Deserialize,Clone)]
#[derive(Serialize, Deserialize)]
pub struct DataObj {
    pub schema: Value,
    pub record: Vec<DataSourceObj>,
}

#[cfg(not)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Clone)]
pub struct SchemaObj {
    pub alias: u32,
    #[serde(rename = "type")]
    pub type_: u32,
    pub datasource: u32,
    pub path: u32,
    pub dbname: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirFullLoadPara {
    pub path: String,
    pub project_id: u64,
    pub team_id: String,
    pub only_pipeline_support: bool,
}

// TODO(code_review): diff to FileTreeNode?
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullFileTreeNode {
    pub absolute_path: String, //"/store/idp-note/projects/1/notebooks",
    pub browser_path: String,  // "",
    pub project_id: String,    //"1",
    pub file_name: String,     //"notebooks",
    pub file_type: String,     //"DIRECTORY", or "FILE"
    pub has_children: bool,
    pub children: Vec<FullFileTreeNode>,
}

//Cell Model
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IpynbFileJson {
    pub cells: Vec<Cells>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Cells {
    pub cell_type: String,
    pub source: Vec<String>,
    pub metadata: MetaData,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MetaData {
    pub id: String,
    pub index: u32,
}
