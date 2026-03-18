use chrono::{Duration, Utc};
use sqlx::PgPool;

const WINDOW_SECONDS: i64 = 300;
const LIMIT_DEFAULT: i64 = 600;
const LIMIT_PRIVATE: i64 = 1200;
const LIMIT_ADMIN: i64 = 3000;

pub struct RateLimiter<'a> {
    pool: &'a PgPool,
}

impl<'a> RateLimiter<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn check_and_increment(
        &self,
        api_key: &str,
        access_level: i16,
    ) -> Result<bool, sqlx::Error> {
        let limit = Self::get_limit(access_level);
        let now = Utc::now();
        let window_start = now - Duration::seconds(WINDOW_SECONDS);

        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            INSERT INTO rate_limits (api_key, requests)
            VALUES ($1, ARRAY[$2::timestamptz])
            ON CONFLICT (api_key) DO UPDATE SET
                requests = array_append(
                    array(SELECT unnest(rate_limits.requests) WHERE unnest > $3),
                    $2::timestamptz
                )
            RETURNING array_length(requests, 1)
            "#,
        )
        .bind(api_key)
        .bind(now)
        .bind(window_start)
        .fetch_optional(self.pool)
        .await?;

        let count = result.map(|(c,)| c).unwrap_or(1);
        Ok(count <= limit)
    }

    fn get_limit(access_level: i16) -> i64 {
        match access_level {
            4.. => LIMIT_ADMIN,
            2..=3 => LIMIT_PRIVATE,
            _ => LIMIT_DEFAULT,
        }
    }
}
