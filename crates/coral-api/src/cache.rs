use database::CacheRepository;

use crate::state::AppState;

pub const SNAPSHOT_SOURCE: &str = "api";

pub async fn refresh_player_cache(state: &AppState, uuid: &str, username: Option<&str>) {
    let player_data = match state.hypixel.get_player(uuid).await {
        Ok(Some(data)) => data,
        _ => return,
    };

    let cache = CacheRepository::new(state.db.pool());
    let _ = cache
        .store_snapshot(uuid, &player_data, None, Some(SNAPSHOT_SOURCE), username)
        .await;
}
