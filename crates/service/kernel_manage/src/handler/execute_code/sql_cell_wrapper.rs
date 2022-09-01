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

use super::execute_req_model::SqlCell;
use crate::handler::execute_code::ExecuteCodeReq;

const LOCAL_FILE_DATASOURCE: &str = "local_csv";

pub fn sql2python(sql_cell: &SqlCell, req: &ExecuteCodeReq) -> String {
    let sql = req.code.trim().replace(r"\n", " ");
    let data_source = &sql_cell.data_source;
    let df_name = sql_cell.df_name.as_deref().unwrap_or("df_0");
    if data_source.is_empty() || data_source == LOCAL_FILE_DATASOURCE {
        tracing::debug!(
            "data_source is empty or {}, treat as local csv",
            data_source
        );
        return sql_from_csv2python(df_name, &sql, req);
    }
    tracing::debug!("data_source is  {}, treat as real data source", data_source);

    let team_id = &sql_cell.uid;
    // find postgres connection config by `req.region + "-" + &uid`
    format!(
        "{df_name} = __import__('baihai_aid').run_sql('{}', '{}', '''{} ''', '{}', '{}')\n{df_name}",
        team_id,
        req.region.clone(),
        sql,
        data_source,
        req.header.project_id,
        df_name = df_name
    )
}

pub fn sql_from_csv2python(df_name: &str, sql: &str, req: &ExecuteCodeReq) -> String {
    format!(
        "{df_name} = __import__('baihai_aid').csv_sql('''{} ''', '{}', '{}')\n{df_name}",
        sql,
        business::path_tool::get_store_path(
            req.header.team_id,
            req.header.project_id,
            business::business_term::ProjectFolder::NOTEBOOKS
        )
        .to_str()
        .unwrap(),
        req.header.path,
        df_name = df_name
    )
}

#[test]
fn test_de_sql_req() {
    let code = r#"{"teamId":"1","projectId":"1","region":"ga","session":"bbb5b78a-6001-415b-a1f9-45037d6a3045","userId":"1483269813963870208","executeType":"cell","msgId":"/😂.ipynb/22421899-8190-4b76-b47f-d84675b9d88e/8909","path":"/😂.ipynb","cellId":"22421899-8190-4b76-b47f-d84675b9d88e","cellType":"sql","code":"SELECT * FROM local.csv","meta":{"uid":"1483269813963870208","id":"22421899-8190-4b76-b47f-d84675b9d88e","path":"/store/idp-note/projects/1/notebooks/helloworld.ipynb","index":9,"dataSource":"local_csv"},"kernel":"1642058392722","identity":"87b86aa9-9d3c-4d04-a8b7-181c80da622b","recordExecuteTime":"true","batchId":1650877768641}"#;
    serde_json::from_str::<ExecuteCodeReq>(code).unwrap();
}
