use anyhow::Result;
use mongodb::{Database, bson::doc};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
struct MongoMember {
    discord_id: i64,
    uuid: Option<String>,
    api_key: Option<String>,
    join_date: Option<String>,
    request_count: Option<i64>,
    config: Option<serde_json::Value>,
    is_admin: Option<bool>,
    is_mod: Option<bool>,
    private: Option<bool>,
    beta_access: Option<bool>,
    key_locked: Option<bool>,
    ip_history: Option<Vec<IpHistoryEntry>>,
    minecraft_accounts: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct IpHistoryEntry {
    ip_address: String,
    first_seen: Option<String>,
}

pub async fn migrate(mongo_db: &Database, pg_pool: &PgPool) -> Result<usize> {
    let collection = mongo_db.collection::<MongoMember>("members");
    let mut cursor = collection.find(doc! {}).await?;

    let mut count = 0;
    let mut errors = 0;

    while cursor.advance().await? {
        let member = match cursor.deserialize_current() {
            Ok(m) => m,
            Err(e) => {
                warn!("Failed to deserialize member: {}", e);
                errors += 1;
                continue;
            }
        };

        if let Err(e) = insert_member(pg_pool, &member).await {
            warn!("Failed to insert member {}: {}", member.discord_id, e);
            errors += 1;
            continue;
        }

        if let Some(ref ips) = member.ip_history {
            if let Err(e) = insert_ip_history(pg_pool, member.discord_id, ips).await {
                warn!(
                    "Failed to insert IP history for {}: {}",
                    member.discord_id, e
                );
            }
        }

        if let Some(ref accounts) = member.minecraft_accounts {
            if let Err(e) = insert_minecraft_accounts(pg_pool, member.discord_id, accounts).await {
                warn!(
                    "Failed to insert minecraft accounts for {}: {}",
                    member.discord_id, e
                );
            }
        }

        count += 1;
        if count % 100 == 0 {
            info!("Processed {} members...", count);
        }
    }

    if errors > 0 {
        warn!("Completed with {} errors", errors);
    }

    Ok(count)
}

async fn insert_member(pool: &PgPool, member: &MongoMember) -> Result<()> {
    let join_date = member
        .join_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    let config = member
        .config
        .clone()
        .unwrap_or_else(|| serde_json::json!({}));

    sqlx::query(
        r#"
        INSERT INTO members (
            discord_id, uuid, api_key, join_date, request_count,
            is_admin, is_mod, is_private, is_beta, key_locked, config
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (discord_id) DO UPDATE SET
            uuid = EXCLUDED.uuid,
            api_key = EXCLUDED.api_key,
            request_count = EXCLUDED.request_count,
            is_admin = EXCLUDED.is_admin,
            is_mod = EXCLUDED.is_mod,
            is_private = EXCLUDED.is_private,
            is_beta = EXCLUDED.is_beta,
            key_locked = EXCLUDED.key_locked,
            config = EXCLUDED.config
        "#,
    )
    .bind(member.discord_id)
    .bind(&member.uuid)
    .bind(&member.api_key)
    .bind(join_date)
    .bind(member.request_count.unwrap_or(0))
    .bind(member.is_admin.unwrap_or(false))
    .bind(member.is_mod.unwrap_or(false))
    .bind(member.private.unwrap_or(false))
    .bind(member.beta_access.unwrap_or(false))
    .bind(member.key_locked.unwrap_or(false))
    .bind(config)
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_ip_history(pool: &PgPool, discord_id: i64, ips: &[IpHistoryEntry]) -> Result<()> {
    let member_id: Option<(i64,)> = sqlx::query_as("SELECT id FROM members WHERE discord_id = $1")
        .bind(discord_id)
        .fetch_optional(pool)
        .await?;

    let Some((member_id,)) = member_id else {
        return Ok(());
    };

    for ip in ips {
        let first_seen = ip
            .first_seen
            .as_ref()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        sqlx::query(
            r#"
            INSERT INTO api_key_ips (member_id, ip_address, first_seen, last_seen)
            VALUES ($1, $2::inet, $3, $3)
            ON CONFLICT (member_id, ip_address) DO NOTHING
            "#,
        )
        .bind(member_id)
        .bind(&ip.ip_address)
        .bind(first_seen)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn insert_minecraft_accounts(
    pool: &PgPool,
    discord_id: i64,
    accounts: &[String],
) -> Result<()> {
    let member_id: Option<(i64,)> = sqlx::query_as("SELECT id FROM members WHERE discord_id = $1")
        .bind(discord_id)
        .fetch_optional(pool)
        .await?;

    let Some((member_id,)) = member_id else {
        return Ok(());
    };

    let member_uuid: Option<String> = sqlx::query_scalar("SELECT uuid FROM members WHERE id = $1")
        .bind(member_id)
        .fetch_optional(pool)
        .await?;

    for uuid in accounts {
        let is_primary = member_uuid.as_deref() == Some(uuid.as_str());

        sqlx::query(
            r#"
            INSERT INTO minecraft_accounts (member_id, uuid, is_primary)
            VALUES ($1, $2, $3)
            ON CONFLICT (member_id, uuid) DO NOTHING
            "#,
        )
        .bind(member_id)
        .bind(uuid)
        .bind(is_primary)
        .execute(pool)
        .await?;
    }

    Ok(())
}
