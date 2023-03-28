use crate::{AsBytes, BotnetResult, Bytes, Key};
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use std::collections::HashMap;

#[cfg(feature = "redisdb")]
use redis::{aio::Connection as RedisConnection, AsyncCommands, Client as RedisClient};

pub trait DatabaseKey:
    Key + AsBytes + std::cmp::Eq + std::hash::Hash + Send + Sync
{
}

#[async_trait]
pub trait Database {
    async fn set_key(&mut self, k: impl DatabaseKey, v: Bytes) -> BotnetResult<()>;
    async fn get_key(&self, k: impl DatabaseKey) -> BotnetResult<Option<Bytes>>;
    async fn set_bytes(&self, b: Bytes, v: Bytes) -> BotnetResult<()>;
    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>>;
}

enum DbType {
    InMemory,
    #[allow(unused)]
    Redis,
}

pub struct InMemory {
    #[allow(unused)]
    db_type: DbType,
    items: Arc<Mutex<HashMap<Bytes, Bytes>>>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            db_type: DbType::InMemory,
            items: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

impl Default for InMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Database for InMemory {
    async fn set_key(&mut self, k: impl DatabaseKey, v: Bytes) -> BotnetResult<()> {
        self.items.lock().await.insert(k.flatten(), v);
        Ok(())
    }

    async fn get_key(&self, k: impl DatabaseKey) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock().await.remove(&k.flatten()))
    }

    async fn set_bytes(&self, k: Bytes, v: Bytes) -> BotnetResult<()> {
        self.items.lock().await.insert(k, v);
        Ok(())
    }

    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock().await.remove(k))
    }
}

#[cfg(feature = "redisdb")]
pub struct Redis {
    conn: Arc<Mutex<RedisConnection>>,
}

#[cfg(feature = "redisdb")]
impl Redis {
    pub async fn new(url: &str) -> BotnetResult<Self> {
        let client = RedisClient::open(url)?;
        let conn = client.get_tokio_connection().await?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[cfg(feature = "redisdb")]
#[async_trait]
impl Database for Redis {
    async fn set_key(&mut self, k: impl DatabaseKey, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    async fn get_key(&self, k: impl DatabaseKey) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    async fn set_bytes(&self, k: Bytes, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }
}
