use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct ResolveResponse {
    pub uuid: String,
    pub username: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/resolve/{identifier}", get(resolve))
}

async fn resolve(
    State(state): State<AppState>,
    Path(identifier): Path<String>,
) -> Result<Json<ResolveResponse>, ApiError> {
    let identity = state.mojang.resolve(&identifier).await?;

    Ok(Json(ResolveResponse {
        uuid: identity.uuid,
        username: identity.username,
    }))
}
