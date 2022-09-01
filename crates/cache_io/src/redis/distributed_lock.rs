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

use std::fmt::Display;

use common_model::entity::cell::Uuid;
use err::ErrorTrace;
use redis::aio::ConnectionLike;
use redis::Cmd;
use redis::ToRedisArgs;
use redis::Value;
use tracing::debug;
use tracing::error;
pub const LOCK: &str = "lock:";

/// Copyright @baihai 2021
/// @author Kim Huang
/// @date 2022/5/5 pm.2:57
///  FIXME: Maybe occur flowing error on cluster mode of redis.need using read_lock instead of lock and release_lock;
/// Using the set key value [EX seconds][PX milliseconds][NX|XX] command looks fine,
/// In fact, there will be problems in Redis cluster.
/// For example, client A gets the lock on the master node of Redis,but this locked key has not been synchronized to the slave node.
/// The master node fails, so a slave node is upgraded to the master node,
/// Client B can also acquire the lock of the same key, but client A has also acquired the lock, which causes multiple clients to acquire the lock.
///@return if lock success,return value of this key.
/// key:lock key
/// seconds: expire seconds
pub async fn try_lock<T: ToRedisArgs + Display>(
    con: &mut redis::aio::Connection,
    key: T,
    seconds: usize,
) -> Result<Uuid, ErrorTrace> {
    let key = format!("{}{}", LOCK, key);
    let uuid = Uuid::new_v4();
    let mut cmd = Cmd::new();
    let cmd = cmd
        .arg("SET")
        .arg(key)
        .arg(uuid.to_string())
        .arg("NX")
        .arg("EX")
        .arg(seconds);

    let result = con.req_packed_command(cmd).await?;

    // dbg!(result.clone());
    match result {
        Value::Okay => return Ok(uuid),
        Value::Nil => return Err(ErrorTrace::new("key already exists!")),
        _ => {
            error!("error! not expect type of return!");
        }
    }
    Err(ErrorTrace::new("error"))
}

pub async fn release_lock<T: ToRedisArgs + Display>(
    con: &mut redis::aio::Connection,
    key: T,
    lock_value: common_model::entity::cell::Uuid,
) -> Result<bool, ErrorTrace> {
    debug!("release lock ,key:{}", key);

    let key = format!("{}{}", LOCK, key);

    let script = redis::Script::new(
        r"
if redis.call('get',KEYS[1]) == ARGV[1] then 
    return redis.call('del',KEYS[1]) 
else 
    return 0 
end
",
    );
    let result_value: i32 = script
        .key(key)
        .arg(lock_value.to_string())
        .invoke_async(con)
        .await?;

    if result_value == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}
