use std::env;
use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use database::Database;

mod routes;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let state = init_state().await?;
    let app = build_router(state);

    serve(app).await
}

fn init_logging() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

async fn init_state() -> Result<AppState> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let db = Database::connect(&database_url).await?;
    Ok(AppState::new(db))
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::api_router())
        .merge(routes::ui_router())
        .with_state(state)
}

async fn serve(app: Router) -> Result<()> {
    let port: u16 = env::var("ADMIN_PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("ADMIN_PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("coral-admin listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
