use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Deserialize)]
pub struct StoreCodeRequest {
    pub code: String,
    pub uuid: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct RedeemCodeResponse {
    pub uuid: String,
    pub username: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/verify/codes", post(store_code))
        .route("/verify/codes/{code}", delete(redeem_code))
}

async fn store_code(
    State(state): State<AppState>,
    Json(body): Json<StoreCodeRequest>,
) -> StatusCode {
    let mut conn = state.redis.connection();
    let uuid = match uuid::Uuid::parse_str(&body.uuid) {
        Ok(u) => u,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    match coral_redis::verify::store_code(&mut conn, &body.code, uuid, &body.username).await {
        Ok(true) => StatusCode::CREATED,
        Ok(false) => StatusCode::CONFLICT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn redeem_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<RedeemCodeResponse>, StatusCode> {
    let mut conn = state.redis.connection();
    let player = coral_redis::verify::redeem_code(&mut conn, &code)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(RedeemCodeResponse {
        uuid: player.uuid.simple().to_string(),
        username: player.username,
    }))
}
