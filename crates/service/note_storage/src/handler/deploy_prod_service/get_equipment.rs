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

pub async fn get_equipment_list(
    pg_pool: &sqlx::PgPool,
    size: i32,
    current: i32,
    team_id: i64,
    service_id: i32,
) -> Result<EquipmentListRsp, ErrorTrace> {
    let count_sql = "
    SELECT  count(*)
    FROM    idp_model_equipment as equipment
    INNER JOIN  idp_model_service_pod as pod 
    ON      equipment.serial_number = pod.equipment_sn
    INNER JOIN  idp_model_deploy_prod_service as service 
    ON      service.id = pod.service_id
    WHERE   service.id = $1 and service.team_id = $2
        ";
    let total = sqlx::query_as::<_, (i64,)>(count_sql)
        .bind(service_id)
        .bind(team_id)
        .fetch_one(pg_pool)
        .await?
        .0;
    let total = total as i32;
    let pages = if total % size == 0 {
        total / size
    } else {
        total / size + 1
    };

    let skip = size * (current - 1);

    let equipment_sql = "
    SELECT  equipment.id as equipment_id,equipment.status as equipment_status,
            equipment_name, serial_number, memo,
            pod.update_time, pod.status as pod_status, pod.status_message,
            pod.edition_id=package.latest_edition_id as is_latest,
            concat(package.model_name,'|',edition.version) as model
    FROM    idp_model_equipment as equipment
    INNER JOIN  idp_model_service_pod as pod 
    ON      equipment.serial_number = pod.equipment_sn
    INNER JOIN  idp_model_package as package 
    ON      pod.package_id = package.id
    INNER JOIN  idp_model_edition as edition 
    ON      edition.package_id = package.id
    INNER JOIN  idp_model_deploy_prod_service as service 
    ON      service.id = pod.service_id
    WHERE   service.id = $1 and service.team_id = $2
    ORDER BY pod.create_time DESC
    OFFSET  $3
    LIMIT   $4
    ";
    let mut equipment_info: Vec<EquipmentInfo> = sqlx::query_as(equipment_sql)
        .bind(service_id)
        .bind(team_id)
        .bind(skip)
        .bind(size)
        .fetch_all(pg_pool)
        .await?;

    for item in equipment_info.iter_mut() {
        item.set_time(pg_pool, Some(team_id)).await;
    }

    let equipment_list = EquipmentListRsp {
        equipment_info,
        size,
        current,
        total,
        pages,
    };
    Ok(equipment_list)
}
