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

use crate::handler::prelude::*;

/**
"reqType": "execute_req"
- code
- path
- cellId
- cellType: code,sql
- projectId
*/
#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCodeReq {
    // pub inode: u64,
    #[serde(flatten)]
    pub header: kernel_common::Header,

    #[serde(default)]
    pub resource: kernel_common::spawn_kernel_process::Resource,

    pub input_reply: Option<String>,

    // pub cell_type: String,
    // pub meta: Meta,
    #[serde(flatten)]
    pub cell_type: CellTypeMeta,
    pub code: String,

    /// e.g. `ga` means gpu alpha version, used in baihai_aid
    pub region: String,
    // - executeType field frontend has filter to code, no markdown
}

/// meta from ipynb json cell struct meta field
#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(tag = "cellType", content = "meta")]
#[serde(rename_all = "lowercase")]
pub enum CellTypeMeta {
    Code {},
    Sql(SqlCell),
    /**
    ```text
    {
        "df_name": "df_0",
        "id": "54606155-6a2d-48c2-b0e0-8e981915deb1",
        "index": 4,
        "chart": {
            "pic_type": "",
            "title": "",
            "x": "",
            "y": "",
            "color": "",
            "size": "",
            "hover_data": "",
            "facet_col": "",
            "facet_row": "",
            "text": ""
        }
    }
    ```
    */
    Visualization {
        df_name: String,
        chart: std::collections::HashMap<String, String>,
    },
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct SqlCell {
    pub uid: String,
    pub df_name: Option<String>,
    pub data_source: String,
}

#[cfg(not)]
#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct VisualCell {
    pub(crate) df_name: Option<String>,
    pub(crate) show_table: Option<String>,
    pub(crate) x_col: Option<String>,
    pub(crate) y_col: Option<String>,
    pub(crate) color_col: Option<String>,
    pub(crate) pic_type: Option<String>,
    #[serde(default)]
    pub(crate) title: String,
}

#[test]
fn test_deser() {
    let req = r#"{
"inode": "17339932244427724339",
"teamId": "1520684197767442432",
"projectId": "156",
"region": "a",
"userId": "1520684197767442432",
"executeType": "cell",
"msgId": "/test.ipynb/7b721981-5347-4328-a148-123f6258b2a6/8771",
"path": "/test.ipynb",
"cellId": "7b721981-5347-4328-a148-123f6258b2a6",
"cellType": "code",
"code": "print(1)",
"meta": {
    "uid": "1520684197767442432",
    "id": "7b721981-5347-4328-a148-123f6258b2a6",
    "index": 0
}
    }"#;
    dbg!(serde_json::from_str::<ExecuteCodeReq>(req).unwrap());
}
