use crate::{database::Database, BotnetKey, BotnetMeta, BotnetResult};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Task<D>
where
    D: Database + Send + Sync,
{
    async fn run(k: BotnetKey, db: Option<D>) -> BotnetResult<Option<Value>>;
}

pub struct Strategy {
    #[allow(unused)]
    meta: BotnetMeta,
}

impl Strategy {
    pub fn new(meta: BotnetMeta) -> Self {
        Self { meta }
    }

    pub fn count_entity() -> u64 {
        1
    }

    pub fn is_k_anonymous() -> bool {
        true
    }

    pub fn has_hit_cliff() -> bool {
        true
    }

    pub fn has_really_hit_cliff() -> bool {
        true
    }
}
