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

use chrono::Duration;
use chrono::NaiveDateTime;
use err::ErrorTrace;

// pub enum TimeType {
//     Start,
//     End,
// }

pub enum OperateFlag {
    /// reset to zero timezone
    _Reduction,
    /// switch to current timezone
    Switch,
}

// tourists/visitor/not_login default to UTC+08 timezone
pub async fn change_tz(
    time: NaiveDateTime,
    team_id: Option<i64>,
    pg_pool: &sqlx::PgPool,
    operate_flag: OperateFlag,
) -> Result<NaiveDateTime, ErrorTrace> {
    let time_zone = match team_id {
        Some(team_id) => {
            match sqlx::query_as::<_, (i32,)>(
                r#"
            SELECT timezone FROM team_info WHERE team_id = $1
            "#,
            )
            .bind(team_id)
            .fetch_one(pg_pool)
            .await
            {
                Ok(time_zone) => time_zone.0 as i64,
                Err(_) => 8_i64,
            }
        }
        None => 8,
    };

    let datetime = match operate_flag {
        OperateFlag::_Reduction => time.checked_sub_signed(Duration::seconds(time_zone * 3600)),
        OperateFlag::Switch => time.checked_add_signed(Duration::seconds(time_zone * 3600)),
    };

    match datetime {
        Some(data) => Ok(data),
        None => Err(ErrorTrace::new("time overflow")),
    }
}

// pub async fn time_dealing(
//     time_string: Option<String>,
//     time_type: TimeType,
//     team_id: Option<i64>,
//     pg_pool: &sqlx::PgPool,
// ) -> Result<String, ErrorTrace> {
//     let time_string = time_string.unwrap_or_else(|| "".to_string());
//     if time_string.is_empty() {
//         Ok("".to_string())
//     } else {
//         let time_string = match time_type {
//             TimeType::Start => format!("{time_string}T00:00:00"),
//             TimeType::End => format!("{time_string}T23:59:59"),
//         };
//         let time_naivetime = NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%dT%H:%M:%S")?;
//         let time_current =
//             change_tz(time_naivetime, team_id, pg_pool, OperateFlag::Reduction).await?;
//         Ok(time_current.to_string())
//     }
// }
