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

use super::execute_req_model::VisualCell;

pub fn vis2python2(df_var_name: &str, req: &str) -> String {
    format!("__import__('baihai_aid').draw_dataframe2({df_var_name}, '{req}')",)
}

pub fn vis2python(req: &VisualCell) -> String {
    let df_name = req.df_name.as_deref().unwrap_or("df_0");
    let show_table = req
        .show_table
        .as_ref()
        .map(|x| x == "true")
        .unwrap_or_else(|| false);
    if show_table {
        return df_name.to_string();
    }
    let x_col = req.x_col.as_deref().unwrap_or("x_col");
    let y_col = req.y_col.as_deref().unwrap_or("y_col");

    let color_col = req.color_col.as_deref().unwrap_or("color_col");
    let pic_type = req.pic_type.as_deref().unwrap_or("point");
    let title = &req.title;

    let draw_code = format!(
        "__import__('baihai_aid').draw_dataframe({df_name}, '{x_col}', '{y_col}', '{color_col}', '{pic_type}', '{title}')",
    );

    tracing::info!("vis2python show table: {:#?}", &draw_code);
    draw_code
}

#[test]
fn test_de_visualization_req() {
    let code = r#"{"teamId":"1","projectId":"1","region":"ga","session":"bbb5b78a-6001-415b-a1f9-45037d6a3045","userId":"1483269813963870208","executeType":"cell","msgId":"/😂.ipynb/cbf310a3-fa54-46c7-9ac9-eadb62dce290/1962","path":"/😂.ipynb","cellId":"cbf310a3-fa54-46c7-9ac9-eadb62dce290","cellType":"visualization","code":"","meta":{"uid":"1483269813963870208","id":"cbf310a3-fa54-46c7-9ac9-eadb62dce290","path":"/store/idp-note/projects/1/5c8faa04-498d-4755-b000-d3dd81633814/share.ipynb","index":12,"x_col":"a","y_col":"b","color_col":"c","pic_type":"line","df_name":"df_0","show_table":""},"kernel":"1642058392722","identity":"87b86aa9-9d3c-4d04-a8b7-181c80da622b","recordExecuteTime":"true","batchId":1650877776285}"#;
    serde_json::from_str::<super::ExecuteCodeReq>(code).unwrap();
}
