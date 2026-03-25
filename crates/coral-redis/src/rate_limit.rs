use redis::AsyncCommands;

use crate::RedisPool;

const WINDOW_SECS: i64 = 300;
const KEY_PREFIX: &str = "ratelimit:";


pub enum RateLimitResult {
    Allowed { remaining: i64 },
    Exceeded,
}


#[derive(Clone)]
pub struct RateLimiter {
    pool: RedisPool,
}


impl RateLimiter {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub async fn check_and_record(
        &self,
        api_key: &str,
        limit: i64,
    ) -> Result<RateLimitResult, redis::RedisError> {
        let key = format!("{KEY_PREFIX}{api_key}");
        let now = chrono::Utc::now().timestamp();
        let mut conn = self.pool.connection();

        redis::pipe()
            .atomic()
            .cmd("ZREMRANGEBYSCORE").arg(&key).arg("-inf").arg(now - WINDOW_SECS).ignore()
            .cmd("ZADD").arg(&key).arg(now).arg(format!("{now}:{}", uuid::Uuid::new_v4())).ignore()
            .cmd("EXPIRE").arg(&key).arg(WINDOW_SECS + 10).ignore()
            .query_async::<()>(&mut conn)
            .await?;

        let count: i64 = conn.zcard(&key).await?;
        match count > limit {
            true => Ok(RateLimitResult::Exceeded),
            false => Ok(RateLimitResult::Allowed { remaining: limit - count }),
        }
    }
}
