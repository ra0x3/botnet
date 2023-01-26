pub mod database;
pub mod task;
pub mod utils;

pub use crate::database::Database;
pub use async_std::sync::{Arc, Mutex};
pub use bytes::Bytes;
pub use nom::AsBytes;
use serde::{Deserialize, Serialize};
pub use serde_json::Value;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use thiserror::Error;
pub use url::Url;
use utils::type_id;

pub struct Input(pub Bytes);

impl Input {
    pub fn new(s: &'static str) -> Self {
        Self(Bytes::from(s.as_bytes()))
    }
}

pub type AnomalyDetectorResult<T> = Result<T, AnomalyDetectorError>;

pub mod prelude {
    pub use super::{
        task::Task,
        utils::{type_id, values_to_bytes},
        AnomalyDetectorResult, Arc, AsBytes, Bytes, Database, Extractor, Field,
        FieldMeta, Input, Key, Mutex, Url, Value,
    };
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.0.as_bytes()).expect("Bad input.")
    }
}
pub trait AsValue {
    fn as_value(&self) -> Bytes;
}

impl AsValue for bool {
    fn as_value(&self) -> Bytes {
        match self {
            true => Bytes::from("1"),
            false => Bytes::from("0"),
        }
    }
}

impl AsValue for u64 {
    fn as_value(&self) -> Bytes {
        Bytes::from(u64::to_le_bytes(*self).to_vec())
    }
}

impl AsValue for &'static str {
    fn as_value(&self) -> Bytes {
        Bytes::from(self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum AnomalyDetectorError {
    #[error("ParseError: {0:#?}")]
    ParseError(#[from] url::ParseError),
    #[error("BincodeError: {0:#?}")]
    BincodeError(#[from] bincode::Error),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct ValueMap {
    items: HashMap<usize, Bytes>,
}

impl Hash for ValueMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in self.items.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for ValueMap {
    fn eq(&self, other: &Self) -> bool {
        for k in self.items.keys() {
            if other.items.contains_key(k) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct FieldMeta {
    name: String,
    key: String,
    value_map: ValueMap,
    type_id: usize,
    description: String,
}

impl FieldMeta {
    pub fn new(name: &str, key: &str, values: Vec<Bytes>, description: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            value_map: ValueMap {
                items: HashMap::from_iter(
                    values.iter().map(|v| (type_id(v.as_bytes()), v.clone())),
                ),
            },
            type_id: utils::type_id(key),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct KeyMetadata {
    field_meta: HashMap<String, FieldMeta>,
    type_id: usize,
}

impl Hash for KeyMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        for (k, v) in self.field_meta.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for KeyMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl KeyMetadata {
    pub fn new() -> Self {
        Self {
            type_id: 0,
            field_meta: HashMap::default(),
        }
    }

    pub fn as_bytes(&self) -> AnomalyDetectorResult<Bytes> {
        Ok(Bytes::from(bincode::serialize(&self)?))
    }

    pub fn field(&mut self, f: FieldMeta) -> &mut Self {
        self.field_meta.insert(f.name.clone(), f);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }

    pub fn with_key_code(meta: Self, type_id: usize) -> Self {
        Self {
            field_meta: meta.field_meta,
            type_id,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub type_id: usize,
    pub key: String,
    pub value: Bytes,
}

impl Field {
    pub fn new(key: &str, value: Bytes) -> Self {
        Self {
            type_id: utils::type_id(key),
            key: key.to_string(),
            value,
        }
    }
}

pub trait Key {
    type Item;
    type Metadata;
    type TypeId;

    fn build(&self) -> Self;
    fn field(&mut self, f: Self::Item) -> &mut Self;
    fn flatten(&self) -> Bytes;
    fn get_metadata(&self) -> Self::Metadata;
    fn metadata(&mut self, meta: KeyMetadata) -> &mut Self;
    fn new(name: &str) -> Self;
    fn type_id(&self) -> Self::TypeId;
}

pub trait Extractor {
    fn extract(input: &Input) -> AnomalyDetectorResult<Field>;
}
