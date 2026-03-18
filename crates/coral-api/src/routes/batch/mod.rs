use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use clients::normalize_uuid;
use database::BlacklistRepository;

use crate::responses::TagResponse;
use crate::state::AppState;

const MAX_BATCH_SIZE: usize = 100;

#[derive(Deserialize)]
struct BatchRequest {
    uuids: Vec<String>,
}

#[derive(Serialize)]
struct BatchResponse {
    players: HashMap<String, Vec<TagResponse>>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/players", post(batch_lookup))
}

async fn batch_lookup(
    State(state): State<AppState>,
    Json(request): Json<BatchRequest>,
) -> Result<Json<BatchResponse>, StatusCode> {
    if request.uuids.len() > MAX_BATCH_SIZE {
        return Err(StatusCode::BAD_REQUEST);
    }

    let uuids: Vec<String> = request.uuids.iter().map(|u| normalize_uuid(u)).collect();

    let repo = BlacklistRepository::new(state.db.pool());

    let results = repo
        .get_players_batch(&uuids)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let players = results
        .into_iter()
        .map(|(uuid, tags)| {
            let tag_responses = tags.iter().map(TagResponse::from_db).collect();
            (uuid, tag_responses)
        })
        .collect();

    Ok(Json(BatchResponse { players }))
}
