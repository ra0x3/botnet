/// Utilities used in anomaly detection tasks.
use crate::{database::Database, BotnetKey, BotnetParams, BotnetResult};
use bytes::{Buf, Bytes};

/// Create a new strategy with which to run anomaly detection.
pub struct Strategy {
    /// The parameters with which to execute the strategy.
    #[allow(unused)]
    pub params: BotnetParams,
}

impl Strategy {
    /// Create a new strategy with which to run anomaly detection.
    pub fn new(params: BotnetParams) -> Self {
        Self { params }
    }

    /// Return whether or not the `entity_counting` strategy is enabled.
    pub fn entity_counting_enabled(&self) -> bool {
        self.params.config().strategy().entity().enabled()
    }

    /// Return whether or not the `kanon` strategy is enabled.
    pub fn kanon_enabled(&self) -> bool {
        self.params.config().strategy().kanon().enabled()
    }

    /// Count this entity in the stratey.
    pub fn count_entity(&self, k: &BotnetKey) -> BotnetResult<u64> {
        let mut db = self.params.db().expect("Database expected.");
        db.incr_key(k)
    }

    /// Return whether or not the entity is k-anonymous.
    pub fn is_k_anonymous(&self, k: &BotnetKey) -> BotnetResult<bool> {
        let db = self.params.db().expect("Database expected.");
        let mut v = db
            .get_key(k)?
            .unwrap_or(Bytes::from(0u64.to_le_bytes().to_vec()));
        let v = v.get_u64_le();
        let k = self.params.config().strategy().kanon().k();
        Ok(v >= k)
    }

    /// Return whether or not the entity has hit the 'cliff'.
    pub fn has_hit_cliff(&self) -> bool {
        true
    }

    pub fn has_really_hit_cliff(&self) -> bool {
        true
    }
}
