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

use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;
use cron_parser::parse;
use err::ErrorTrace;

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/2/7
 * Time: 20:21
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

//"yyyy-MM-dd-HH-mm-ss";
pub fn check_to_run_time(
    current_time: DateTime<Local>,
    cron_expression: &str,
    cron_start_date: &str,
    cron_end_date: &str,
    cron_start_time: &str,
    cron_end_time: &str,
) -> Result<bool, ErrorTrace> {
    let fmt = "%Y-%m-%d-%H-%M-%S";
    let cron_cur_time_str = current_time.format(fmt);
    tracing::info!("{}", cron_cur_time_str);
    tracing::info!("----{}", cron_cur_time_str.to_string());

    let mut cron_start_date_time = "".to_string();

    tracing::info!("cron_start_date={}", cron_start_date);
    tracing::info!("cron_end_date  ={}", cron_end_date);

    if cron_start_date.len() > 10 {
        cron_start_date_time += &cron_start_date[0..10];
    } else {
        cron_start_date_time += cron_start_date;
    }
    cron_start_date_time += "-";
    cron_start_date_time += cron_start_time;

    let mut cron_end_date_time = "".to_string();
    if cron_end_date.len() > 10 {
        cron_end_date_time += &cron_end_date[0..10];
    } else {
        cron_end_date_time += cron_end_date;
    }
    cron_end_date_time += "-";
    cron_end_date_time += cron_end_time;

    tracing::info!("cron_start_date_time ={}", cron_start_date_time);
    tracing::info!("cron_end_date_time   ={}", cron_end_date_time);

    let start = Local.datetime_from_str(cron_start_date_time.as_str(), fmt)?;
    tracing::info!("start={}", start.timestamp());

    let end = Local.datetime_from_str(cron_end_date_time.as_str(), fmt)?;
    tracing::info!("end  ={}", end.timestamp());

    let cron_time = current_time.timestamp();
    tracing::info!("cron_time  ={}", cron_time);
    tracing::info!("current_time.to_string()  ={}", current_time.to_string());

    let mut flag = false;

    if (cron_time - start.timestamp()) >= 0 && (cron_time - end.timestamp()) <= 0 {
        tracing::info!("need to schedule ....");
        tracing::info!("cur currentTime={}", current_time);

        //here use some skill , try to get the next schedule is the current time to run the job
        let mills = (cron_time - 1) * 1000; //it's very important
        let pre_time: DateTime<Local> = Local.timestamp_millis_opt(mills).unwrap();
        tracing::info!("skill  pre_time={}", pre_time);

        let next = parse(cron_expression, &pre_time).unwrap();
        tracing::info!("next_run__time={}", next);

        let sub_seconds = next.timestamp() - current_time.timestamp(); //sec
        tracing::info!("---sub_seconds={}", sub_seconds);

        //the sub_seconds will be 0 , so the job will be scheduled
        if (0..=59).contains(&sub_seconds) {
            flag = true;
        }
    }

    tracing::info!("flag={}", flag);
    Ok(flag)
}

#[test]
fn mm() {
    use crate::pojo::component::component_dir::CronConfigFields;
    // let cron_exp = "0 0 0 * * *".to_string();
    // let result = translate_to_chinese(cron_exp);
    // println!("{}", result)

    // let cron_end_date = "2023-02-22T11:04:30.559Z";
    // let mut cron_end_date_time = "".to_string();
    // if cron_end_date.len() > 10{
    //     println!("ddddd");
    //     cron_end_date_time += &cron_end_date[0..10];
    //     println!("cron_end_date_time={}",cron_end_date_time);
    // }else {
    //     cron_end_date_time += cron_end_date;
    // }
    // cron_end_date_time += "-00-00-00";

    // let json="{\"cronType\":\"simple\",\"cronEndDate\":\"2023-02-23\",\"cronEndTime\":\"06:05\",\"cronStartDate\":\"2023-02-22T12:54:39.034Z\",\"cronStartTime\":\"00:00\",\"cronExpression\":\"0 0 * * *\"}";
    let json = "{\"cronType\":\"simple\",\"cronEndDate\":\"2023-02-23\",\"cronStartDate\":\"2023-02-22T12:54:39.034Z\",\"cronStartTime\":\"00:00\",\"cronExpression\":\"0 0 * * *\"}";
    // let obj: CronConfigFields = serde_json::from_value(json)?;
    let obj: CronConfigFields = serde_json::from_str(json).unwrap();

    println!("{}", obj.cron_start_date);
    println!("{:?}", obj.cron_start_time);
    println!("{:?}", obj.cron_end_time);
}

pub async fn translate_to_chinese(cron_exp: String) -> String {
    let tmp_corns = cron_exp.split(' ').collect::<Vec<&str>>();
    let mut buffer = "".to_string();

    if tmp_corns.len() == 6 {
        if !tmp_corns[4].eq_ignore_ascii_case("*") {
            if tmp_corns[4].contains('/') {
                let month_every = tmp_corns[4].split('/').collect::<Vec<&str>>();
                buffer = "".to_string();
                let tmp_m = month_every[1];
                let m_string = format!("每{tmp_m}个月的");
                buffer += m_string.as_str();
            } else {
                buffer += tmp_corns[4];
                buffer += "月";
            }
        } else {
            //do nothing
        }

        //parse week
        if !tmp_corns[5].eq_ignore_ascii_case("*") && !tmp_corns[5].eq_ignore_ascii_case("?") {
            let tmp_array = tmp_corns[5].split(',').collect::<Vec<&str>>();
            for (_index, item) in tmp_array.iter().enumerate() {
                match *item {
                    "1" => buffer += "每星期一",
                    "2" => buffer += "每星期二",
                    "3" => buffer += "每星期三",
                    "4" => buffer += "每星期四",
                    "5" => buffer += "每星期五",
                    "6" => buffer += "每星期六",
                    "7" => buffer += "每星期日",
                    "-" => buffer += "至",
                    _ => buffer += "Error Crontab",
                }
            }
        }

        if !tmp_corns[5].eq_ignore_ascii_case("*") && !tmp_corns[5].eq_ignore_ascii_case("?") {
            //if week is validate , the parse day is not necessary
        } else {
            //parse day  0 0 0 1/2 * *
            if !tmp_corns[3].eq_ignore_ascii_case("?") {
                if !tmp_corns[3].eq_ignore_ascii_case("*") {
                    if tmp_corns[3].contains('/') {
                        let day_every = tmp_corns[3].split('/').collect::<Vec<&str>>();
                        buffer = "".to_string();
                        let tmp_d = day_every[1];
                        let d_string = format!("每{tmp_d}天的");
                        buffer += d_string.as_str();
                    } else {
                        if buffer.is_empty() {
                            buffer += "每月";
                        }
                        buffer += tmp_corns[3];
                        buffer += "日";
                    }
                } else {
                    buffer += "每日";
                }
            }
        }

        //parse hour
        if !tmp_corns[2].eq_ignore_ascii_case("*") {
            if tmp_corns[2].contains('/') {
                let hour_every = tmp_corns[2].split('/').collect::<Vec<&str>>();
                buffer = "".to_string();
                let tmp_h = hour_every[1];
                let h_string = format!("每{tmp_h}小时的");
                buffer += h_string.as_str();
            } else {
                buffer += tmp_corns[2];
                buffer += "时";
            }
        } else {
            buffer += "每时";
        }

        //parse minute
        if !tmp_corns[1].eq_ignore_ascii_case("*") {
            if tmp_corns[1].contains('/') {
                let minute_every = tmp_corns[1].split('/').collect::<Vec<&str>>();
                buffer = "".to_string();
                let tmp_m = minute_every[1];
                let m_string = format!("每{tmp_m}分钟");
                buffer += m_string.as_str();
            } else {
                buffer += tmp_corns[1];
                buffer += "分";
            }
        } else {
            buffer += "每分";
        }
    }
    buffer
}
