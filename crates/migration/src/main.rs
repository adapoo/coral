use anyhow::Result;
use tracing::info;

mod blacklist;
mod cache;
mod members;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    info!("Starting Coral migration from MongoDB to PostgreSQL");

    let mongodb_uri = std::env::var("MONGODB_URI")?;
    let postgres_uri = std::env::var("DATABASE_URL")?;

    let mongo_client = mongodb::Client::with_uri_str(&mongodb_uri).await?;
    let mongo_db = mongo_client.database("urchindb");

    let pg_pool = sqlx::PgPool::connect(&postgres_uri).await?;

    info!("Connected to both databases");

    info!("Migrating members...");
    let members_count = members::migrate(&mongo_db, &pg_pool).await?;
    info!("Migrated {} members", members_count);

    info!("Migrating blacklist...");
    let blacklist_count = blacklist::migrate(&mongo_db, &pg_pool).await?;
    info!("Migrated {} blacklisted players", blacklist_count);

    info!("Migrating historical cache...");
    let cache_count = cache::migrate(&mongo_db, &pg_pool).await?;
    info!("Migrated cache for {} players", cache_count);

    info!("Migration complete!");

    Ok(())
}
