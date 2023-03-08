use crate::prelude::*;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Task<K, D>
where
    K: DatabaseKey,
    D: Database + Send + Sync,
{
    async fn run(k: K, db: Option<D>) -> BitsyResult<Option<Value>>;
}
