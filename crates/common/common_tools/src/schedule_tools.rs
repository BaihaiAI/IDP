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

use chrono::Local;
use cron_parser::parse;

pub fn get_schedule_list(cron_expression: String) -> Vec<String> {
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now();
    let mut next_crons_time = Vec::<String>::new();
    let mut next = parse(&cron_expression, &now).unwrap();
    for _ in 0..5 {
        next = parse(&cron_expression, &next).unwrap();
        let next_value = next.format(fmt).to_string();
        next_crons_time.push(next_value);
    }
    // for x in next_crons_time {
    //     println!("{}", x);
    // }
    // {
    //     "code": 200,
    //     "message": "success",
    //     "data": ["2023-02-10 12:07:00", "2023-02-10 12:08:00", "2023-02-10 12:09:00", "2023-02-10 12:10:00", "2023-02-10 12:11:00"]
    // }
    next_crons_time
}
