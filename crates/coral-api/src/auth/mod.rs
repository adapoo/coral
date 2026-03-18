use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

use database::{Member, MemberRepository};

use crate::middleware::RateLimiter;
use crate::state::AppState;

const ACCESS_LEVEL_MODERATOR: i16 = 3;
const ACCESS_LEVEL_ADMIN: i16 = 4;

#[derive(Clone)]
pub struct AuthenticatedMember(pub Member);

#[derive(Clone)]
pub struct InternalAuth;

pub async fn require_internal_or_admin(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = extract_api_key(&request).ok_or(StatusCode::UNAUTHORIZED)?;

    if is_internal_key(&state, &api_key) {
        request.extensions_mut().insert(InternalAuth);
        return Ok(next.run(request).await);
    }

    let member = authenticate_member(&state, &api_key).await?;

    if member.access_level < ACCESS_LEVEL_ADMIN {
        return Err(StatusCode::FORBIDDEN);
    }

    request.extensions_mut().insert(AuthenticatedMember(member));
    Ok(next.run(request).await)
}

pub async fn require_moderator(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = extract_api_key(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    let member = authenticate_member(&state, &api_key).await?;

    if member.access_level < ACCESS_LEVEL_MODERATOR {
        return Err(StatusCode::FORBIDDEN);
    }

    request.extensions_mut().insert(AuthenticatedMember(member));
    Ok(next.run(request).await)
}

pub async fn allow_internal_or_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = extract_api_key(&request).ok_or(StatusCode::UNAUTHORIZED)?;

    if is_internal_key(&state, &api_key) {
        request.extensions_mut().insert(InternalAuth);
        return Ok(next.run(request).await);
    }

    let member = authenticate_member(&state, &api_key).await?;
    request.extensions_mut().insert(AuthenticatedMember(member));
    Ok(next.run(request).await)
}

async fn authenticate_member(state: &AppState, api_key: &str) -> Result<Member, StatusCode> {
    let member = MemberRepository::new(state.db.pool())
        .get_by_api_key(api_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if member.key_locked {
        return Err(StatusCode::FORBIDDEN);
    }

    let allowed = RateLimiter::new(state.db.pool())
        .check_and_increment(api_key, member.access_level)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !allowed {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(member)
}

fn is_internal_key(state: &AppState, api_key: &str) -> bool {
    state
        .internal_api_key
        .as_ref()
        .map(|k| k == api_key)
        .unwrap_or(false)
}

fn extract_api_key(request: &Request) -> Option<String> {
    if let Some(header) = request.headers().get("X-API-Key") {
        return header.to_str().ok().map(String::from);
    }

    request.uri().query().and_then(|query| {
        query
            .split('&')
            .find_map(|pair| pair.strip_prefix("key="))
            .map(String::from)
    })
}
