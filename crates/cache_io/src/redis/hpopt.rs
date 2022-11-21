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
