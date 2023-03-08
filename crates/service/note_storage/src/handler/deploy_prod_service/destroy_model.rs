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

use super::*;

pub async fn destroy_model(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    service_id: i32,
    region: &str,
) -> Result<(), ErrorTrace> {
    tracing::info!("run destroy_model,team_id={}", team_id);

    let sql = r#"
        SELECT  service_type
        FROM    idp_model_deploy_prod_service
        WHERE   id =$1"#;
    let service_type = sqlx::query_as::<_, (i32,)>(sql)
        .bind(service_id)
        .fetch_one(pg_pool)
        .await?
        .0;

    let service_type = ServiceType::to_string(service_type);
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/destroy"))
        .json(&ModelDestroyBody {
            account: service_id.to_string(),
            region: region.to_string(),
            action: "model-destroy",
            service: "idp",
            platform: "idp-model",
            service_type,
        })
        .send()
        .await?;
    tracing::info!("{K8S_SERVICE_API_BASE_URL}/destroy end");

    let resp_body = resp.json::<RspBody>().await?;
    let resp_code = resp_body.code.to_string();
    if !resp_code.starts_with('2') {
        tracing::error!(
            "code:{resp_code},message:{},data:{:?}",
            resp_body.message,
            resp_body.data
        );
        return Err(ErrorTrace::new("failed to destroy model"));
    }
    Ok(())
}
