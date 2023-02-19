use crate::prelude::*;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Task<K, D>
where
    K: Key + std::cmp::Eq + std::hash::Hash + Send + Sync,
    D: Database<K> + Send + Sync,
{
    type Database;

    async fn run(
        k: &'static K,
        db: Option<Self::Database>,
    ) -> AnomalyDetectorResult<Option<Value>>;
}
