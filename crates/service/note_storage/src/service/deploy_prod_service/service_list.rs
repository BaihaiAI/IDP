// Copyright 2023 BaihaiAI, Inc.
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
use common_model::Rsp;
use err::ErrorTrace;

use crate::api_model::deploy_service::ServiceList;
use crate::api_model::deploy_service::ServiceListQto;
use crate::api_model::deploy_service::ServiceModel;

pub async fn list_by_equip(
    Query(req): Query<ServiceListQto>,
    Extension(pg_pool): Extension<sqlx::PgPool>,
) -> Result<Rsp<Vec<ServiceList>>, ErrorTrace> {
    let sql = r#"
        SELECT 
            CONCAT(dpl.id,'') as service_id,
            svc.service_name,
            svc.intro as service_intro,
            model.model_name,
            edition.version,
            model.latest_edition,
            dpl.status,
            dpl.status_message
        FROM 
            (((idp_model_service_pod as dpl INNER JOIN idp_model_deploy_prod_service as svc
                ON dpl.service_id = svc.id)
            LEFT JOIN idp_model_package as model
                ON dpl.package_id = model.id)
            LEFT JOIN idp_model_edition as edition
                ON dpl.edition_id = edition.id)
        WHERE 
            dpl.equipment_sn = $1
        ORDER BY dpl.id desc
            "#;
    let service_model = sqlx::query_as::<_, ServiceModel>(sql)
        .bind(req.serial_number)
        .fetch_all(&pg_pool)
        .await?;
    let mut service_vec = Vec::new();
    for item in service_model.into_iter() {
        service_vec.push(ServiceList::init(item));
    }

    Ok(Rsp::success(service_vec))
}
