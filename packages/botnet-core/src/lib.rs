#![deny(unused_crate_dependencies)]

pub mod database;
pub mod eval;
pub mod task;
pub mod utils;

pub use crate::database::{Database, DatabaseKey};
pub use async_std::sync::{Arc, Mutex};
pub use botnet_utils::type_id;
pub use bytes::Bytes;
use http::Uri;
pub use nom::AsBytes;
use serde::{Deserialize, Serialize};
pub use serde_json::Value as SerdeValue;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};
use thiserror::Error;
pub use url::Url;

pub struct Input(pub Bytes);

impl Input {
    pub fn new(s: &'static str) -> Self {
        Self(Bytes::from(s.as_bytes()))
    }
}

pub type BotnetResult<T> = Result<T, BotnetError>;
pub type ExtractorFn = fn(&Input) -> BotnetResult<Field>;

pub mod prelude {
    pub use super::{
        database::InMemory, eval::Evaluator, task::Task, type_id, utils::values_to_bytes,
        Arc, AsBytes, BotnetResult, Bytes, Database, DatabaseKey, Extractor, ExtractorFn,
        Extractors, Field, FieldMetadata, FieldType, Input, Key, KeyMetadata, KeyType,
        Metadata, Mutex, SerdeValue, Url,
    };
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.0.as_bytes()).expect("Bad input.")
    }
}

impl From<&'static str> for Input {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
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
pub enum BotnetError {
    #[error("ParseError: {0:#?}")]
    ParseError(#[from] url::ParseError),
    #[error("BincodeError: {0:#?}")]
    BincodeError(#[from] bincode::Error),
    #[error("Utf8Error: {0:#?}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[cfg(feature = "redisdb")]
    #[error("RedisError: {0:#?}")]
    RedisError(#[from] redis::RedisError),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct ValueMap {
    items: HashMap<usize, Bytes>,
}

impl ValueMap {
    pub fn from_values(v: Vec<Bytes>) -> Self {
        Self {
            items: HashMap::from_iter(
                v.iter().map(|v| (type_id(v.as_bytes()), v.clone())),
            ),
        }
    }
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

pub enum KeyType {
    Compressed,
    Decompressed,
}

pub enum FieldType {
    Compressed,
    Decompressed,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct FieldMetadata {
    name: String,
    key: String,
    value_map: ValueMap,
    type_id: usize,
    description: String,
}

impl FieldMetadata {
    pub fn new(name: &str, key: &str, values: Vec<Bytes>, description: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            value_map: ValueMap::from_values(values),
            type_id: type_id(key),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct KeyMetadata {
    field_meta: HashMap<String, FieldMetadata>,
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

    pub fn as_bytes(&self) -> BotnetResult<Bytes> {
        Ok(Bytes::from(bincode::serialize(&self)?))
    }

    pub fn field(&mut self, f: FieldMetadata) -> &mut Self {
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
    pub field_name: String,
    pub value: Bytes,
}

impl Field {
    pub fn new(field_name: &str, value: Bytes) -> Self {
        Self {
            type_id: type_id(field_name),
            field_name: field_name.to_string(),
            value,
        }
    }
}

pub trait Key {
    const NAME: &'static str;
    const TYPE_ID: usize;

    type Item;
    type Metadata;

    fn build(&self) -> Self;
    fn field(&mut self, f: Self::Item) -> &mut Self;
    fn flatten(&self) -> Bytes;
    fn get_metadata(&self) -> Self::Metadata;
    fn metadata(&mut self, meta: KeyMetadata) -> &mut Self;
    fn builder() -> Self;
    fn type_id(&self) -> usize;
    fn name(&self) -> &'static str;
}

#[derive(Clone)]
pub struct Extractor {
    #[allow(unused)]
    key: String,
    func: ExtractorFn,
}

impl Default for Extractor {
    fn default() -> Self {
        fn default_func(_input: &Input) -> BotnetResult<Field> {
            Ok(Field::default())
        }

        Self {
            key: String::default(),
            func: default_func,
        }
    }
}

impl Extractor {
    pub fn new(key: &str, func: ExtractorFn) -> Self {
        Self {
            key: key.to_string(),
            func,
        }
    }

    pub fn call(&self, input: &Input) -> BotnetResult<Field> {
        (self.func)(input)
    }
}

#[derive(Default, Clone)]
pub struct Extractors {
    pub items: HashMap<String, Extractor>,
}

impl Extractors {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    pub fn add(&mut self, key: &str, value: ExtractorFn) {
        self.items
            .insert(key.to_string(), Extractor::new(key, value));
    }

    pub fn from<'a, I>(value: I) -> Self
    where
        I: Iterator<Item = (&'a str, ExtractorFn)>,
    {
        let mut items = HashMap::new();
        for (k, f) in value {
            items.insert(k.to_string(), Extractor::new(k, f));
        }

        Self { items }
    }
}

#[derive(Default, Clone)]
pub struct Metadata {
    items: HashMap<usize, KeyMetadata>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    pub fn insert(&mut self, ty_id: usize, meta: KeyMetadata) {
        self.items.insert(ty_id, meta);
    }

    pub fn get(&self, ty_id: &usize) -> &KeyMetadata {
        self.items.get(ty_id).unwrap()
    }

    pub fn from<'a, I>(value: I) -> Self
    where
        I: Iterator<Item = (usize, KeyMetadata)>,
    {
        let mut items = HashMap::new();
        for (k, f) in value {
            items.insert(k, f);
        }

        Self { items }
    }
}
