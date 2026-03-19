use anyhow::Result;
use mongodb::{Database, bson::doc};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
struct MongoBlacklistPlayer {
    uuid: String,
    is_locked: Option<bool>,
    lock_reason: Option<String>,
    locked_by: Option<String>,
    lock_timestamp: Option<mongodb::bson::DateTime>,
    tags: Option<Vec<MongoTag>>,
}

#[derive(Debug, Deserialize)]
struct MongoTag {
    tag_type: String,
    reason: String,
    added_by: Option<i64>,
    added_on: Option<String>,
    evidence: Option<String>,
}

pub async fn migrate(mongo_db: &Database, pg_pool: &PgPool) -> Result<usize> {
    let collection = mongo_db.collection::<MongoBlacklistPlayer>("blacklist");
    let mut cursor = collection.find(doc! {}).await?;

    let mut count = 0;
    let mut errors = 0;

    while cursor.advance().await? {
        let player = match cursor.deserialize_current() {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to deserialize blacklist player: {}", e);
                errors += 1;
                continue;
            }
        };

        if let Err(e) = insert_player(pg_pool, &player).await {
            warn!("Failed to insert blacklist player {}: {}", player.uuid, e);
            errors += 1;
            continue;
        }

        count += 1;
        if count % 100 == 0 {
            info!("Processed {} blacklisted players...", count);
        }
    }

    if errors > 0 {
        warn!("Completed with {} errors", errors);
    }

    Ok(count)
}

async fn insert_player(pool: &PgPool, player: &MongoBlacklistPlayer) -> Result<()> {
    let lock_timestamp = player.lock_timestamp.map(|dt| {
        chrono::DateTime::from_timestamp_millis(dt.timestamp_millis())
            .unwrap_or_else(chrono::Utc::now)
    });

    let locked_by = player.locked_by.as_ref().and_then(|s| {
        s.parse::<i64>().ok().or_else(|| {
            warn!("Invalid locked_by value '{}' for player {}", s, player.uuid);
            None
        })
    });

    sqlx::query(
        r#"
        INSERT INTO blacklist_players (uuid, is_locked, lock_reason, locked_by, locked_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (uuid) DO UPDATE SET
            is_locked = EXCLUDED.is_locked,
            lock_reason = EXCLUDED.lock_reason,
            locked_by = EXCLUDED.locked_by,
            locked_at = EXCLUDED.locked_at
        "#,
    )
    .bind(&player.uuid)
    .bind(player.is_locked.unwrap_or(false))
    .bind(&player.lock_reason)
    .bind(locked_by)
    .bind(lock_timestamp)
    .execute(pool)
    .await?;

    if let Some(ref tags) = player.tags {
        for tag in tags {
            if let Err(e) = insert_tag(pool, &player.uuid, tag).await {
                warn!("Failed to insert tag for {}: {}", player.uuid, e);
            }
        }
    }

    Ok(())
}

async fn insert_tag(pool: &PgPool, uuid: &str, tag: &MongoTag) -> Result<()> {
    let added_on = tag
        .added_on
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    sqlx::query(
        r#"
        INSERT INTO player_tags (uuid, tag_type, reason, evidence, added_by, added_on)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (uuid, tag_type) DO UPDATE SET
            reason = EXCLUDED.reason,
            evidence = EXCLUDED.evidence
        "#,
    )
    .bind(uuid)
    .bind(&tag.tag_type)
    .bind(&tag.reason)
    .bind(&tag.evidence)
    .bind(tag.added_by)
    .bind(added_on)
    .execute(pool)
    .await?;

    Ok(())
}
