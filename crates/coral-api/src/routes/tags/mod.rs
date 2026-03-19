use axum::extract::{Path, State};
use axum::routing::{delete, patch, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

use clients::normalize_uuid;
use coral_redis::BlacklistEvent;
use database::BlacklistRepository;

use crate::auth::AuthenticatedMember;
use crate::error::ApiError;
use crate::state::AppState;

const ALLOWED_TAG_TYPES: &[&str] = &[
    "sniper",
    "blatant_cheater",
    "closet_cheater",
    "replays_needed",
    "caution",
];
const ACCESS_LEVEL_HELPER: i16 = 2;
const ACCESS_LEVEL_MODERATOR: i16 = 3;
const MAX_REASON_LENGTH: usize = 500;
const MAX_IDENTIFIER_LENGTH: usize = 36;

#[derive(Deserialize)]
struct AddTagRequest {
    uuid: String,
    tag_type: String,
    reason: String,
    #[serde(default)]
    hide_username: bool,
}

#[derive(Deserialize)]
struct OverwriteTagRequest {
    expected: ExpectedTag,
    update: UpdateTag,
}

#[derive(Deserialize)]
struct ExpectedTag {
    tag_type: String,
    reason: String,
}

#[derive(Deserialize)]
struct UpdateTag {
    tag_type: String,
    reason: String,
    #[serde(default)]
    hide_username: bool,
}

#[derive(Deserialize)]
struct LockRequest {
    reason: String,
}

#[derive(Serialize)]
struct TagIdResponse {
    id: i64,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tags", post(add_tag))
        .route("/tags/{uuid}/{tag_id}", delete(remove_tag))
        .route("/tags/{uuid}/{tag_id}", patch(overwrite_tag))
}

pub fn mod_router() -> Router<AppState> {
    Router::new()
        .route("/player/lock/{uuid}", post(lock_player))
        .route("/player/lock/{uuid}", delete(unlock_player))
}

async fn add_tag(
    State(state): State<AppState>,
    Extension(member): Extension<AuthenticatedMember>,
    Json(request): Json<AddTagRequest>,
) -> Result<Json<TagIdResponse>, ApiError> {
    if member.0.tagging_disabled {
        return Err(ApiError::Forbidden(
            "tagging is disabled on your account".into(),
        ));
    }

    if request.uuid.len() > MAX_IDENTIFIER_LENGTH {
        return Err(ApiError::BadRequest("uuid too long".into()));
    }

    if !ALLOWED_TAG_TYPES.contains(&request.tag_type.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "invalid tag type '{}', allowed: {}",
            request.tag_type,
            ALLOWED_TAG_TYPES.join(", ")
        )));
    }

    if request.reason.len() > MAX_REASON_LENGTH {
        return Err(ApiError::BadRequest(format!(
            "reason exceeds maximum length of {MAX_REASON_LENGTH} characters"
        )));
    }

    let uuid = normalize_uuid(&request.uuid);
    let repo = BlacklistRepository::new(state.db.pool());

    if let Some(player) = repo.get_player(&uuid).await?
        && player.is_locked
        && member.0.access_level < ACCESS_LEVEL_HELPER
    {
        return Err(ApiError::Forbidden("player is locked".into()));
    }

    let id = repo
        .add_tag(
            &uuid,
            &request.tag_type,
            &request.reason,
            member.0.discord_id,
            request.hide_username,
            None,
        )
        .await
        .map_err(|e| ApiError::Internal(format!("failed to add tag: {e}")))?;

    state
        .event_publisher
        .publish(&BlacklistEvent::TagAdded {
            uuid,
            tag_id: id,
            added_by: member.0.discord_id,
        })
        .await;

    Ok(Json(TagIdResponse { id }))
}

async fn remove_tag(
    State(state): State<AppState>,
    Extension(member): Extension<AuthenticatedMember>,
    Path((uuid, tag_id)): Path<(String, i64)>,
) -> Result<Json<SuccessResponse>, ApiError> {
    let repo = BlacklistRepository::new(state.db.pool());

    let tag = repo
        .get_tag_by_id(tag_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("tag not found".into()))?;

    let is_own_tag = tag.added_by == member.0.discord_id;
    let is_helper = member.0.access_level >= ACCESS_LEVEL_HELPER;

    if !is_own_tag && !is_helper {
        return Err(ApiError::Forbidden(
            "you can only remove your own tags".into(),
        ));
    }

    let is_restricted = tag.tag_type == "confirmed_cheater" || tag.tag_type == "caution";
    if is_restricted && member.0.access_level < ACCESS_LEVEL_MODERATOR {
        return Err(ApiError::Forbidden(
            "only moderators can remove confirmed_cheater and caution tags".into(),
        ));
    }

    let success = repo
        .remove_tag(tag_id, member.0.discord_id)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to remove tag: {e}")))?;

    if success {
        state
            .event_publisher
            .publish(&BlacklistEvent::TagRemoved {
                uuid: normalize_uuid(&uuid),
                tag_id,
                removed_by: member.0.discord_id,
            })
            .await;
    }

    Ok(Json(SuccessResponse { success }))
}

async fn overwrite_tag(
    State(state): State<AppState>,
    Extension(member): Extension<AuthenticatedMember>,
    Path((uuid, tag_id)): Path<(String, i64)>,
    Json(request): Json<OverwriteTagRequest>,
) -> Result<Json<TagIdResponse>, ApiError> {
    if member.0.tagging_disabled {
        return Err(ApiError::Forbidden(
            "tagging is disabled on your account".into(),
        ));
    }

    if !ALLOWED_TAG_TYPES.contains(&request.update.tag_type.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "invalid tag type '{}', allowed: {}",
            request.update.tag_type,
            ALLOWED_TAG_TYPES.join(", ")
        )));
    }

    if request.update.reason.len() > MAX_REASON_LENGTH {
        return Err(ApiError::BadRequest(format!(
            "reason exceeds maximum length of {MAX_REASON_LENGTH} characters"
        )));
    }

    let uuid = normalize_uuid(&uuid);
    let repo = BlacklistRepository::new(state.db.pool());

    let tag = repo
        .get_tag_by_id(tag_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("tag not found".into()))?;

    if tag.tag_type != request.expected.tag_type || tag.reason != request.expected.reason {
        return Err(ApiError::Conflict(
            "tag has been modified since you last viewed it".into(),
        ));
    }

    let is_own_tag = tag.added_by == member.0.discord_id;
    let is_helper = member.0.access_level >= ACCESS_LEVEL_HELPER;

    if !is_own_tag && !is_helper {
        return Err(ApiError::Forbidden(
            "you can only overwrite your own tags".into(),
        ));
    }

    repo.remove_tag(tag_id, member.0.discord_id)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to remove old tag: {e}")))?;

    let id = repo
        .add_tag(
            &uuid,
            &request.update.tag_type,
            &request.update.reason,
            member.0.discord_id,
            request.update.hide_username,
            None,
        )
        .await
        .map_err(|e| ApiError::Internal(format!("failed to add new tag: {e}")))?;

    state
        .event_publisher
        .publish(&BlacklistEvent::TagOverwritten {
            uuid,
            old_tag_id: tag_id,
            old_tag_type: tag.tag_type.clone(),
            old_reason: tag.reason.clone(),
            new_tag_id: id,
            overwritten_by: member.0.discord_id,
        })
        .await;

    Ok(Json(TagIdResponse { id }))
}

async fn lock_player(
    State(state): State<AppState>,
    Extension(member): Extension<AuthenticatedMember>,
    Path(uuid): Path<String>,
    Json(request): Json<LockRequest>,
) -> Result<Json<SuccessResponse>, ApiError> {
    if member.0.access_level < ACCESS_LEVEL_MODERATOR {
        return Err(ApiError::Forbidden("moderator access required".into()));
    }

    if request.reason.len() > MAX_REASON_LENGTH {
        return Err(ApiError::BadRequest(format!(
            "reason exceeds maximum length of {MAX_REASON_LENGTH} characters"
        )));
    }

    let uuid = normalize_uuid(&uuid);
    let repo = BlacklistRepository::new(state.db.pool());

    let success = repo
        .lock_player(&uuid, &request.reason, member.0.discord_id)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to lock player: {e}")))?;

    if success {
        state
            .event_publisher
            .publish(&BlacklistEvent::PlayerLocked {
                uuid,
                locked_by: member.0.discord_id,
                reason: request.reason,
            })
            .await;
    }

    Ok(Json(SuccessResponse { success }))
}

async fn unlock_player(
    State(state): State<AppState>,
    Extension(member): Extension<AuthenticatedMember>,
    Path(uuid): Path<String>,
) -> Result<Json<SuccessResponse>, ApiError> {
    if member.0.access_level < ACCESS_LEVEL_MODERATOR {
        return Err(ApiError::Forbidden("moderator access required".into()));
    }

    let uuid = normalize_uuid(&uuid);
    let repo = BlacklistRepository::new(state.db.pool());

    let success = repo
        .unlock_player(&uuid)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to unlock player: {e}")))?;

    if success {
        state
            .event_publisher
            .publish(&BlacklistEvent::PlayerUnlocked {
                uuid,
                unlocked_by: member.0.discord_id,
            })
            .await;
    }

    Ok(Json(SuccessResponse { success }))
}
