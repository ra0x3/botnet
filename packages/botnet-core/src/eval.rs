/// A collection of utilities in anomaly detection evaluation.
use crate::prelude::*;
use async_trait::async_trait;
use serde_json::Value;

/// Used to ensure all evaluator logic conforms to a unified interface.
#[async_trait]
pub trait Evaluator {
    /// Evaluate a result.
    async fn eval(result: Value) -> BotnetResult<Option<Value>>;
}
