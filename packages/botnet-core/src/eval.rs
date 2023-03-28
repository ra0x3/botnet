use crate::prelude::*;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Evaluator {
    async fn eval(result: Value) -> BotnetResult<Option<Value>>;
}
