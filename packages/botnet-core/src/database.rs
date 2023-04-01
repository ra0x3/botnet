use crate::{BotnetKey, BotnetResult, Bytes};
use async_trait::async_trait;
use bytes::Buf;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[allow(unused)]
#[cfg(feature = "redisdb")]
use redis::{Client as RedisClient, Connection as RedisConnection};

#[async_trait]
pub trait Database {
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()>;
    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>>;
    fn set_bytes(&self, b: &Bytes, v: Bytes) -> BotnetResult<()>;
    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>>;
    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64>;
}

#[derive(Clone, Debug)]
enum DbType {
    InMemory,
    #[allow(unused)]
    Redis,
}

#[derive(Clone, Debug)]
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

impl Database for InMemory {
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        self.items.lock()?.insert(k.flatten(), v);
        Ok(())
    }

    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock()?.remove(&k.flatten()))
    }

    fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        self.items.lock()?.insert(k.clone(), v);
        Ok(())
    }

    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock()?.remove(k))
    }

    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        let v = match self.items.lock()?.remove(&k.flatten()) {
            Some(mut v) => v.get_u64_le() + 1,
            None => 1,
        };
        self.set_key(k, Bytes::from(v.to_le_bytes().to_vec()))?;
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
    pub fn new(url: &str) -> BotnetResult<Self> {
        let client = RedisClient::open(url)?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[cfg(feature = "redisdb")]
#[async_trait]
impl Database for Redis {
    #[allow(unused)]
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    #[allow(unused)]
    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    #[allow(unused)]
    fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    #[allow(unused)]
    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    #[allow(unused)]
    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        unimplemented!()
    }
}
