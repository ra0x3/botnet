use crate::{AnomalyDetectorResult, AsBytes, Bytes, Key, KeyMetadata};
use async_std::sync::{Arc, Mutex, RwLock};
use async_trait::async_trait;
use std::collections::HashMap;

pub trait FoobarZoo:
    Key + AsBytes + std::cmp::Eq + std::hash::Hash + Send + Sync
{
}

#[async_trait]
pub trait Database {
    async fn set_key<K: FoobarZoo>(
        &mut self,
        k: K,
        v: Bytes,
    ) -> AnomalyDetectorResult<()>;
    async fn get_key<K: FoobarZoo>(&self, k: K) -> AnomalyDetectorResult<Option<K>>;
    async fn set_bytes(&self, b: Bytes, v: Bytes) -> AnomalyDetectorResult<()>;
    async fn get_bytes(&self, k: Bytes) -> AnomalyDetectorResult<Option<Bytes>>;
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

/**
 *
 * Cache needs to store bytes as keys and bytes as values
 *
 * bytes for storing key metadata
 *
 * and bytes for storing the literal flattened key
 *
 */

#[async_trait]
impl Database for InMemory {
    async fn set_key<K: FoobarZoo>(
        &mut self,
        k: K,
        v: Bytes,
    ) -> AnomalyDetectorResult<()> {
        let k = Bytes::from(k.as_bytes().to_owned());
        self.items.lock().await.insert(k, v);
        Ok(())
    }

    async fn get_key<K: FoobarZoo>(&self, k: K) -> AnomalyDetectorResult<Option<K>> {
        Ok(None)
    }

    async fn set_bytes(&self, b: Bytes, v: Bytes) -> AnomalyDetectorResult<()> {
        Ok(())
    }

    async fn get_bytes(&self, k: Bytes) -> AnomalyDetectorResult<Option<Bytes>> {
        Ok(None)
    }

    // async fn get(&self, k: Bytes) -> AnomalyDetectorResult<Option<Bytes>> {
    //     Ok(self.items.lock().await.remove(&k))
    // }

    // // TODO: both types of K (Decompress, Compress) require different K types/sized

    // async fn set_metadata< K: FoobarZoo>(&self, k: K) -> AnomalyDetectorResult<()> {
    //     let key_meta = k.get_metadata();
    //     let ty_id = usize::to_le_bytes(key_meta.type_id).to_vec();
    //     let bytes = vec![
    //         Bytes::from(ty_id),
    //         key_meta.as_bytes()?,
    //     ]
    //     .concat();
    //     self.items
    //         .lock()
    //         .await
    //         .insert(k, Bytes::from(bytes));
    //     Ok(())
    // }

    // async fn get_metadata(&self, k: K) -> AnomalyDetectorResult<()> {
    //     let bytes = self.items.lock().await.get(&k).to_owned().unwrap();
    //     let key_ty_id = &bytes[..64];
    //     Ok(())
    // }
}
