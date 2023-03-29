use crate::prelude::*;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Task<K, D>
where
    K: DatabaseKey,
    D: Database + Send + Sync,
{
    async fn run(k: K, db: Option<D>) -> BotnetResult<Option<Value>>;
}

pub enum EntityCounter {
    IpUa,
    Other,
}

pub enum KAnonimity {
    K800,
    K8000,
}

pub enum Rate {
    V1,
    V2,
}

pub fn count_entity() {
    // User defines what an 'entity' is
    //      or just use a simple function to start
    //  literally just use that entity function to get the entity then
    // increment db counter
}

pub fn is_k_anonymous() {
    // has met some k-anonimity according to some counter
    //
    // counter function can be static/dynamic
}

pub fn has_programmatic_rate() {
    // Use a small set of pre-defined rate functions
}

pub fn has_really_programmatic_rate() {
    // has_programmatic_rate() ** N and such
}
