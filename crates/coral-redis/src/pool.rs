use redis::Client;
use redis::aio::ConnectionManager;

#[derive(Clone)]
pub struct RedisPool {
    manager: ConnectionManager,
}

impl RedisPool {
    pub async fn connect(url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    pub fn connection(&self) -> ConnectionManager {
        self.manager.clone()
    }
}
