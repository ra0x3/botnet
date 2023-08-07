use crate::context::BotnetContext;
/// Utilities used in anomaly detection tasks.
use crate::database::Database;
use crate::models::*;
use bytes::{Buf, Bytes};
use std::rc::Rc;

/// Create a new strategy with which to run anomaly detection.
pub struct Strategy {
    /// The parameters with which to execute the strategy.
    pub context: Rc<BotnetContext>,
}

impl Strategy {
    /// Create a new strategy with which to run anomaly detection.
    pub fn new(context: Rc<BotnetContext>) -> Self {
        Self { context }
    }

    /// Return whether or not the `entity_counting` strategy is enabled.
    pub fn entity_counting_enabled(&self) -> bool {
        self.context.config().plan.entity.enabled
    }

    /// Return whether or not the `kanon` strategy is enabled.
    pub fn kanon_enabled(&self) -> bool {
        self.context.config().plan.kanon.enabled
    }

    /// Count this entity in the stratey.
    pub fn count_entity(&self, k: &BotnetKey) -> BotnetResult<usize> {
        self.context.count_entity(k)
    }

    /// Return whether or not the entity is k-anonymous.
    pub fn is_k_anonymous(&self, k: &BotnetKey) -> BotnetResult<bool> {
        let db = self.context.db().expect("Database expected.");
        let v = db
            .get_key(k)?
            .unwrap_or(Bytes::from(0usize.to_le_bytes().to_vec()))
            .get_u64_le() as usize;
        Ok(self.context.config().plan.kanon.is_k_anonymous(v))
    }

    /// Return whether or not the entity has hit the 'cliff'.
    pub fn has_hit_cliff(&self) -> bool {
        true
    }

    pub fn has_really_hit_cliff(&self) -> bool {
        unimplemented!()
    }
}
