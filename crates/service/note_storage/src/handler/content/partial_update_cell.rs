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

use axum::extract::Query;
use axum::Extension;
use axum::Json;
use common_model::api_model::PartialUpdateCellReq;
use common_model::service::rsp::Rsp;
use err::ErrorTrace;

use crate::api_model::TeamIdQueryString;
use crate::app_context::AppContext;

pub async fn put_cell(
    Json(PartialUpdateCellReq {
        path,
        project_id,
        cells,
    }): Json<PartialUpdateCellReq>,
    Query(TeamIdQueryString { team_id }): Query<TeamIdQueryString>,
    Extension(app_context): Extension<AppContext>,
) -> Result<Rsp<()>, ErrorTrace> {
    if cells.is_empty() {
        return Err(ErrorTrace::new("cells is empty"));
    }
    if !(path.ends_with(".ipynb") || path.ends_with(".idpnb")) {
        return Err(ErrorTrace::new("path is not a ipynb"));
    }

    let path = business::path_tool::get_store_full_path(team_id, project_id, path);
    if !path.exists() {
        return Err(ErrorTrace::new(&format!("{path:?} not exist")));
    }
    let mut futs = Vec::new();
    for cell_update_item in cells {
        /*
        dbg!(&cell_update_item.id);
        dbg!(
            &cell_update_item
                .updates
                .outputs
                .as_ref()
                .map(|x| x.len())
                .unwrap_or_default()
        );
        */
        futs.push(
            app_context
                .redis_cache
                .partial_update_cell(&path, cell_update_item, project_id),
        );
    }
    futures::future::try_join_all(futs).await?;
    Ok(Rsp::success(()))
}

#[test]
#[ignore]
fn test_put_cell() {
    let large_payload = "1".repeat(3 * 1024 * 1024);
    let rsp = reqwest::blocking::Client::new()
        .put("http://127.0.0.1:8082/api/v2/idp-note-rs/content/cell?teamId=1")
        .json(&PartialUpdateCellReq {
            path: large_payload,
            project_id: 1,
            cells: Vec::new(),
        })
        .send()
        .unwrap();
    dbg!(rsp.status());
    dbg!(rsp.text().unwrap());
}
