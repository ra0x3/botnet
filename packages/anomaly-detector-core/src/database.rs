use crate::{AnomalyDetectorResult, Bytes, Key, KeyMetadata};
use async_std::sync::{Arc, Mutex};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Database<K>
where
    K: Key + std::cmp::Eq + std::hash::Hash + Send + Sync,
{
    async fn set(&mut self, k: K, v: Bytes) -> AnomalyDetectorResult<()>;
    async fn get(&self, k: K) -> AnomalyDetectorResult<Option<Arc<Mutex<Bytes>>>>;
    async fn set_metadata(&self, k: K) -> AnomalyDetectorResult<()>;
}

enum DbType {
    InMemory,
    #[allow(unused)]
    Redis,
}

pub struct InMemory<K> {
    #[allow(unused)]
    db_type: DbType,
    items: Arc<Mutex<HashMap<K, Arc<Mutex<Bytes>>>>>,
}

impl<K> InMemory<K> {
    pub fn new() -> Self {
        Self {
            db_type: DbType::InMemory,
            items: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

impl<K> Default for InMemory<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<K> Database<K> for InMemory<K>
where
    K: Key + std::cmp::Eq + std::hash::Hash + Send + Sync,
    K: Key<Metadata = KeyMetadata, TypeId = usize>,
{
    async fn set(&mut self, k: K, v: Bytes) -> AnomalyDetectorResult<()> {
        self.items.lock().await.insert(k, Arc::new(Mutex::new(v)));
        Ok(())
    }

    async fn get(&self, k: K) -> AnomalyDetectorResult<Option<Arc<Mutex<Bytes>>>> {
        Ok(self.items.lock().await.remove(&k))
    }

    async fn set_metadata(&self, k: K) -> AnomalyDetectorResult<()> {
        let v = k.get_metadata().as_bytes()?;
        self.items.lock().await.insert(k, Arc::new(Mutex::new(v)));
        Ok(())
    }
}
