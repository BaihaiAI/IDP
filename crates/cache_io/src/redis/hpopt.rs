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

use bb8_redis::redis::AsyncCommands;
use bb8_redis::redis::RedisWrite;
use bb8_redis::redis::ToRedisArgs;
use err::ErrorTrace;
use err::Result;

use crate::CacheService;
use crate::OptimizeState;

impl CacheService {
    pub async fn set_optimize_state(&self, opt_key: &str, opt_state: OptimizeState) -> Result<()> {
        let key = format!("{}{}", crate::keys::OPTIMIZE_STATE_PREFIX, opt_key);
        self.pool
            .get()
            .await?
            .set_ex::<_, _, ()>(key, opt_state, 60 * 60 * 24)
            .await?;
        Ok(())
    }

    pub async fn get_optimize_state(
        &self,
        opt_key: &str,
    ) -> core::result::Result<Option<String>, ErrorTrace> {
        let key = format!("{}{}", crate::keys::OPTIMIZE_STATE_PREFIX, opt_key);
        match self.pool.get().await?.get(key).await {
            Ok(opt_state) => Ok(opt_state),
            Err(err) => {
                tracing::error!("failed to get opt_state:{}", err);
                Err(ErrorTrace::from(err))
            }
        }
    }
}
impl ToRedisArgs for OptimizeState {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let clone_state_str = match self {
            OptimizeState::Running => "running",
            OptimizeState::Success => "success",
            OptimizeState::Failed => "failed",
        };
        out.write_arg(clone_state_str.as_bytes())
    }
}
