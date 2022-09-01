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

use err::ErrorTrace;
use err::Result;
use redis::AsyncCommands;
use redis::RedisResult;
use redis::RedisWrite;
use redis::ToRedisArgs;

use super::CacheService;
use crate::CloneState;

impl CacheService {
    pub async fn set_clone_state(&self, clone_key: &str, clone_state: CloneState) -> Result<()> {
        let key = format!("{}{}", crate::keys::CLONE_STATE_PREFIX, clone_key);
        let _cache_result: RedisResult<()> = self
            .pool
            .get()
            .await?
            .set_ex(key, clone_state, 60 * 60 * 24)
            .await;
        Ok(())
    }

    pub async fn get_clone_state(
        &self,
        clone_key: &str,
    ) -> core::result::Result<Option<String>, ErrorTrace> {
        let key = format!("{}{}", crate::keys::CLONE_STATE_PREFIX, clone_key);
        match self.pool.get().await?.get(key).await {
            Ok(clone_state) => Ok(clone_state),
            Err(err) => {
                tracing::error!("failed to get clone_state:{}", err);
                Err(ErrorTrace::from(err))
            }
        }
    }
}
impl ToRedisArgs for CloneState {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let clone_state_str = match self {
            CloneState::Cloning => "cloning",
            CloneState::Success => "success",
            CloneState::Failed => "failed",
        };
        out.write_arg(clone_state_str.as_bytes())
    }
}
