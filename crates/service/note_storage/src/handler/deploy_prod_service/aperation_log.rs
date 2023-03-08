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

pub async fn add_aperation_log(
    pg_pool: &sqlx::PgPool,
    team_id: i64,
    user_id: i64,
    operation: ServiceOperation,
    service_id: i32,
) -> Result<(), ErrorTrace> {
    let content = serde_json::json!({
        "service_id": service_id.to_string(),
        "operation": operation.to_string()
    });
    sqlx::query(
        "
    INSERT  INTO    idp_operation_log 
            (team_id, creator_id, content)
    VALUES  ($1, $2, $3)",
    )
    .bind(team_id)
    .bind(user_id)
    .bind(content)
    .execute(pg_pool)
    .await?;
    Ok(())
}

pub async fn log_list(pg_pool: &sqlx::PgPool, service_id: i32) -> Result<OperationRsp, ErrorTrace> {
    let mut operation_info: Vec<OperationInfo> = sqlx::query_as(
        "
    SELECT  team_id, creator_id, create_time, content
    FROM    idp_operation_log
    WHERE   (content::json#>>'{service_id}')::text = $1
    ORDER BY create_time DESC",
    )
    .bind(service_id.to_string())
    .fetch_all(pg_pool)
    .await?;

    for item in operation_info.iter_mut() {
        item.set_time(pg_pool, Some(item.team_id)).await
    }
    Ok(OperationRsp { operation_info })
}
