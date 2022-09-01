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
use err::ErrorTrace;

use super::CacheService;
use crate::SnapshotRedisListItem;
impl CacheService {
    pub async fn snapshot_list(&self, key: &str) -> Result<Vec<SnapshotRedisListItem>, ErrorTrace> {
        let vals = self
            .pool
            .get()
            .await?
            .lrange::<_, Vec<String>>(key, 0, -1)
            .await?;
        let mut ret = vec![];
        for val in vals {
            ret.push(serde_json::from_str(&val)?);
        }
        Ok(ret)
    }
    pub async fn snapshot_insert(
        &self,
        key: &str,
        value: SnapshotRedisListItem,
    ) -> Result<(), ErrorTrace> {
        let mut conn = self.pool.get().await?;
        conn.lpush(key, serde_json::to_string(&value)?).await?;
        conn.ltrim(key, 0, 99).await?;
        Ok(())
    }
}
