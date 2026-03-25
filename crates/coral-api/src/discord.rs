use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use reqwest::Client;
use serde::Deserialize;

const CACHE_TTL: Duration = Duration::from_secs(900);
const CACHE_CLEANUP_THRESHOLD: usize = 500;


#[derive(Deserialize)]
struct DiscordUser {
    username: String,
}


pub struct DiscordResolver {
    http: Client,
    token: String,
    cache: Mutex<HashMap<u64, (String, Instant)>>,
}


impl DiscordResolver {
    pub fn new(token: String) -> Self {
        Self { http: Client::new(), token, cache: Mutex::new(HashMap::new()) }
    }

    pub async fn resolve_username(&self, user_id: u64) -> Option<String> {
        if let Some(cached) = self.get_cached(user_id) {
            return Some(cached);
        }

        let user = self.http
            .get(format!("https://discord.com/api/v10/users/{user_id}"))
            .header("Authorization", format!("Bot {}", self.token))
            .send().await.ok()?
            .json::<DiscordUser>().await.ok()?;

        let mut cache = self.cache.lock().unwrap();
        if cache.len() > CACHE_CLEANUP_THRESHOLD {
            cache.retain(|_, (_, at)| at.elapsed() < CACHE_TTL);
        }
        cache.insert(user_id, (user.username.clone(), Instant::now()));
        Some(user.username)
    }

    fn get_cached(&self, user_id: u64) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        let (username, at) = cache.get(&user_id)?;
        (at.elapsed() < CACHE_TTL).then(|| username.clone())
    }
}
