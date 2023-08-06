/// A collection of anomaly detection compatible NoSQL databases.
use crate::models::*;
use crate::Bytes;
use async_trait::async_trait;
use bytes::Buf;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[allow(unused)]
#[cfg(feature = "redisdb")]
use redis::{Client as RedisClient, Connection as RedisConnection};

/// Used to ensure all database logic conforms to a unified interface.
#[async_trait]
pub trait Database {
    /// Set a key in the database.
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()>;

    /// Get a key from the database.
    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>>;

    /// Set a key in the database (with the key as bytes).
    fn set_bytes(&self, b: &Bytes, v: Bytes) -> BotnetResult<()>;

    /// Get a key from the database (with the key as bytes).
    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>>;

    /// Increment a key in the database.
    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64>;
}

/// Database type.
#[derive(Clone, Debug)]
enum DbType {
    /// In-memory database.
    InMemory,

    /// Redis database.
    #[allow(unused)]
    Redis,
}

/// In-memory database.
#[derive(Clone, Debug)]
pub struct InMemory {
    /// Database type.
    #[allow(unused)]
    db_type: DbType,

    /// Database items.
    items: Arc<Mutex<HashMap<Bytes, Bytes>>>,
}

impl InMemory {
    /// Create a new in-memory database.
    pub fn new() -> Self {
        Self {
            db_type: DbType::InMemory,
            items: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

impl Default for InMemory {
    /// Create a new in-memory database.
    fn default() -> Self {
        Self::new()
    }
}

impl Database for InMemory {
    /// Set a key in the database.
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        self.items.lock()?.insert(k.flatten(), v);
        Ok(())
    }

    /// Get a key from the database.
    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock()?.remove(&k.flatten()))
    }

    /// Set a key in the database (with the key as bytes).
    fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        self.items.lock()?.insert(k.clone(), v);
        Ok(())
    }

    /// Get a key from the database (with the key as bytes).
    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        Ok(self.items.lock()?.remove(k))
    }

    /// Increment a key in the database.
    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        let v = match self.items.lock()?.remove(&k.flatten()) {
            Some(mut v) => v.get_u64_le() + 1,
            None => 1,
        };
        self.set_key(k, Bytes::from(v.to_le_bytes().to_vec()))?;
        Ok(v)
    }
}

/// Redis database.
#[cfg(feature = "redisdb")]
pub struct Redis {
    /// Redis connection.
    #[allow(unused)]
    conn: Arc<Mutex<RedisConnection>>,
}

#[cfg(feature = "redisdb")]
impl Redis {
    /// Create a new redis database.
    pub fn new(url: &str) -> BotnetResult<Self> {
        let client = RedisClient::open(url)?;
        let conn = client.get_connection()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

/// Redis database.
#[cfg(feature = "redisdb")]
#[async_trait]
impl Database for Redis {
    /// Set a key in the database.
    #[allow(unused)]
    fn set_key(&mut self, k: &BotnetKey, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    /// Get a key from the database.
    #[allow(unused)]
    fn get_key(&self, k: &BotnetKey) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    /// Set a key in the database (with the key as bytes).
    #[allow(unused)]
    fn set_bytes(&self, k: &Bytes, v: Bytes) -> BotnetResult<()> {
        unimplemented!()
    }

    /// Get a key from the database (with the key as bytes).
    #[allow(unused)]
    fn get_bytes(&self, k: &Bytes) -> BotnetResult<Option<Bytes>> {
        unimplemented!()
    }

    /// Increment a key in the database.
    #[allow(unused)]
    fn incr_key(&mut self, k: &BotnetKey) -> BotnetResult<u64> {
        unimplemented!()
    }
}
