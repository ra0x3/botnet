/// Utilities used in anomaly detection tasks.
use crate::{
    config::{Anonimity, EntityCounter, RateLimit},
    context::BotnetContext,
    BotnetResult,
};
use async_std::sync::Arc;

/// A strategy for running anomaly detection.
pub trait Strategy {
    /// Execute the provided `Strategy`.
    fn execute(&self) -> BotnetResult<()>;
}

/// Create a new strategy with which to run anomaly detection.
pub struct BotnetStrategy<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// The parameters with which to execute the strategy.
    pub context: Arc<BotnetContext<E, A, C>>,
}

impl<E, A, C> BotnetStrategy<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Create a new strategy with which to run anomaly detection.
    pub fn new(context: Arc<BotnetContext<E, A, C>>) -> Self {
        Self { context }
    }
}

impl<E, A, C> Strategy for BotnetStrategy<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    fn execute(&self) -> BotnetResult<()> {
        todo!()
    }
}
