use crate::{database::Database, BotnetKey, BotnetParams, BotnetResult};
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
    pub params: BotnetParams,
}

impl Strategy {
    pub fn new(params: BotnetParams) -> Self {
        Self { params }
    }

    pub fn entity_counting_enabled(&self) -> bool {
        self.params.config.strategy.entity.enabled
    }

    pub fn count_entity(&self, k: &BotnetKey) -> BotnetResult<u64> {
        let mut db = self.params.db.clone().expect("Database expected.");
        db.incr_key(k)
    }

    pub fn is_k_anonymous(&self) -> bool {
        true
    }

    pub fn has_hit_cliff(&self) -> bool {
        true
    }

    pub fn has_really_hit_cliff(&self) -> bool {
        true
    }
}
