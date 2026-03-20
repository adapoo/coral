use std::time::Duration;

use anyhow::{Result, anyhow};
use reqwest::{Client, Response};
use serde::Deserialize;
use serde::de::DeserializeOwned;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub struct CoralApiClient {
    http: Client,
    base_url: String,
    api_key: String,
}

#[derive(Deserialize)]
pub struct PlayerStatsResponse {
    pub uuid: String,
    pub username: String,
    pub hypixel: Option<serde_json::Value>,
    #[serde(default)]
    pub tags: Vec<TagInfo>,
    pub skin_url: Option<String>,
}

#[derive(Deserialize)]
pub struct TagInfo {
    pub tag_type: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GuildResponse {
    pub name: String,
    pub tag: Option<String>,
    pub tag_color: Option<String>,
    pub level: u32,
    pub members: usize,
    pub experience: u64,
    pub created: Option<String>,
    pub player: Option<GuildMemberInfo>,
}

#[derive(Deserialize)]
pub struct GuildMemberInfo {
    pub rank: Option<String>,
    pub joined: Option<String>,
    pub weekly_gexp: Option<u64>,
}

#[derive(Deserialize)]
pub struct ResolveResponse {
    pub uuid: String,
    pub username: String,
}

impl CoralApiClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let http = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to create HTTP client");

        Self {
            http,
            base_url,
            api_key,
        }
    }

    pub async fn get_player_stats(&self, identifier: &str) -> Result<PlayerStatsResponse> {
        let url = format!("{}/v1/player/stats/{}", self.base_url, identifier);
        self.get(&url).await
    }

    pub async fn get_guild(
        &self,
        identifier: &str,
        by: Option<&str>,
    ) -> Result<Option<GuildResponse>> {
        let url = match by {
            Some(by) => format!("{}/v1/guild/{}?by={}", self.base_url, identifier, by),
            None => format!("{}/v1/guild/{}", self.base_url, identifier),
        };
        self.get(&url).await
    }

    pub async fn resolve(&self, identifier: &str) -> Result<ResolveResponse> {
        let url = format!("{}/v1/resolve/{}", self.base_url, identifier);
        self.get(&url).await
    }

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .http
            .get(url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        Self::parse_response(response).await
    }

    async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("{} {}", status.as_u16(), body));
        }

        Ok(response.json().await?)
    }
}
