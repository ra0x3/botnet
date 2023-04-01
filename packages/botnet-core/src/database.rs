use crate::{BotnetKey, BotnetResult, Bytes};
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use bytes::Buf;
use std::collections::HashMap;

#[allow(unused)]
#[cfg(feature = "redisdb")]
use redis::{aio::Connection as RedisConnection, AsyncCommands, Client as RedisClient};

#[async_trait]
pub trait Database {
    async fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()>;
    async fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>>;
    async fn set_bytes(&self, b: &Bytes, v: Bytes) -> BotnetResult<()>;
    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>>;
    async fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64>;
}

#[derive(Clone)]
enum DbType {
    InMemory,
    #[allow(unused)]
    Redis,
}

#[derive(Clone)]
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
    async fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        self.items.lock().await.insert(k.flatten(), v);
        Ok(())
    }

    async fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock().await.remove(&k.flatten()))
    }

    async fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        self.items.lock().await.insert(k.clone(), v);
        Ok(())
    }

    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock().await.remove(k))
    }

    async fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        let mut v = self.items.lock().await.remove(&k.flatten()).unwrap();
        let v = v.get_u64_le() + 1;
        self.set_key(k, Bytes::from(v.to_le_bytes().to_vec()))
            .await?;
        Ok(v)
    }
}

#[cfg(feature = "redisdb")]
pub struct Redis {
    #[allow(unused)]
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
    #[allow(unused)]
    async fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    #[allow(unused)]
    async fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    #[allow(unused)]
    async fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    #[allow(unused)]
    async fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    #[allow(unused)]
    async fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        unimplemented!()
    }
}
