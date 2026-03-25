use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

use coral_redis::RateLimitResult;
use database::{AccessRank, Member, MemberRepository};

use crate::state::AppState;


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
    if AccessRank::from_level(member.access_level) < AccessRank::Admin {
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
    if AccessRank::from_level(member.access_level) < AccessRank::Moderator {
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

    match state.rate_limiter.check_and_record(api_key, rate_limit_for_access(member.access_level)).await {
        Ok(RateLimitResult::Allowed { .. }) => {}
        Ok(RateLimitResult::Exceeded) => return Err(StatusCode::TOO_MANY_REQUESTS),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
    Ok(member)
}


pub fn rate_limit_for_access(access_level: i16) -> i64 {
    match access_level {
        4.. => 3000,
        2..=3 => 1200,
        _ => 600,
    }
}


fn is_internal_key(state: &AppState, api_key: &str) -> bool {
    state.internal_api_key.as_ref().is_some_and(|k| k == api_key)
}


fn extract_api_key(request: &Request) -> Option<String> {
    if let Some(header) = request.headers().get("X-API-Key") {
        return header.to_str().ok().map(String::from);
    }
    request.uri().query().and_then(|q| {
        form_urlencoded::parse(q.as_bytes())
            .find(|(k, _)| k == "key")
            .map(|(_, v)| v.into_owned())
    })
}
