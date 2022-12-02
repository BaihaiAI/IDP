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
use business::business_term::ProjectId;
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
    pub async fn get_optimize_list(
        &self,
        project_id: ProjectId,
        db_name: String,
    ) -> core::result::Result<Vec<crate::OptimizeStatus>, ErrorTrace> {
        let match_prefix = format!(
            "{}{}:{}:*",
            crate::keys::OPTIMIZE_STATE_PREFIX,
            project_id,
            db_name
        );
        // get all keys.
        let match_keys = match self
            .pool
            .get()
            .await?
            .keys::<_, Vec<String>>(match_prefix)
            .await
        {
            Ok(it) => it,
            Err(err) => return Err(ErrorTrace::from(err)),
        };
        // get all optimize state.
        let mut opt_state_list = Vec::new();
        for key in match_keys {
            let opt_state = self.pool.get().await?.get::<_, String>(key.clone()).await?;
            //optimize_state:{project_id}:{db_name}:{study_id}:{pid}
            let opt_status = crate::OptimizeStatus {
                state: OptimizeState::from(opt_state),
                opt_run_key: key.clone(),
                study_id: key
                    .split(':')
                    .nth(4)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap(),
            };
            opt_state_list.push(opt_status);
        }
        Ok(opt_state_list)
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
// impl from string for optimizestate
impl From<String> for OptimizeState {
    fn from(state: String) -> Self {
        match state.as_str() {
            "running" => OptimizeState::Running,
            "success" => OptimizeState::Success,
            "failed" => OptimizeState::Failed,
            _ => OptimizeState::Failed,
        }
    }
}
