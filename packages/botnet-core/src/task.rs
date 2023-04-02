use crate::{database::Database, BotnetKey, BotnetParams, BotnetResult};
use bytes::{Buf, Bytes};

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

    pub fn kanon_enabled(&self) -> bool {
        self.params.config.strategy.kanon.enabled
    }

    pub fn count_entity(&self, k: &BotnetKey) -> BotnetResult<u64> {
        let mut db = self.params.db.clone().expect("Database expected.");
        db.incr_key(k)
    }

    pub fn is_k_anonymous(&self, k: &BotnetKey) -> BotnetResult<bool> {
        let db = self.params.db.clone().expect("Database expected.");
        let mut v = db
            .get_key(k)?
            .unwrap_or(Bytes::from(0u64.to_le_bytes().to_vec()));
        let v = v.get_u64_le();
        let k = self.params.config.strategy.kanon.k.k();
        Ok(v >= k)
    }

    pub fn has_hit_cliff(&self) -> bool {
        true
    }

    pub fn has_really_hit_cliff(&self) -> bool {
        true
    }
}
