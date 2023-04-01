#![deny(unused_crate_dependencies)]
#[macro_use]

pub mod database;
pub mod config;
pub mod eval;
pub mod task;
pub mod utils;

pub use crate::{
    config::{BotnetConfig, DbType, Field as ConfigField, Key as ConfigKey},
    database::{Database, InMemory},
};
pub use botnet_utils::type_id;
pub use bytes::Bytes;
use http::Uri;
pub use nom::AsBytes;
use serde::{Deserialize, Serialize};
pub use serde_json::Value as SerdeValue;
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    io::Error as IoError,
    sync::{Arc, PoisonError},
};
use thiserror::Error;
use tokio::task::JoinError;
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
        Botnet, BotnetKey, BotnetKeyMetadata, BotnetParams, BotnetResult, Extractor,
        ExtractorFn, Extractors, Field, FieldExtractors, FieldMetadata, Input, Metadata,
        Url,
    };

    pub use crate::utils;

    pub use crate::database::{Database, InMemory};

    pub use crate::config::BotnetConfig;

    pub use crate::eval::Evaluator;

    pub use crate::task::Strategy;
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
    #[error("SerdeYamlError: {0:#?}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("IoError: {0:#?}")]
    IoError(#[from] IoError),
    #[error("Error")]
    Error(#[from] Box<dyn std::error::Error>),
    #[error("JoinError: {0:#?}")]
    JoinError(#[from] JoinError),
    #[error("Lock poisoned.")]
    PoisonError,
}

impl<T> From<PoisonError<T>> for BotnetError {
    fn from(_e: PoisonError<T>) -> Self {
        Self::PoisonError
    }
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

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct FieldMetadata {
    name: String,
    key: String,
    type_id: usize,
    description: String,
}

impl FieldMetadata {
    pub fn new(name: &str, key: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            type_id: type_id(key),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct BotnetKeyMetadata {
    field_meta: HashMap<String, FieldMetadata>,
    type_id: usize,
    name: String,
}

impl From<&ConfigKey> for BotnetKey {
    fn from(val: &ConfigKey) -> Self {
        let metadata = BotnetKeyMetadata::new(&val.name);
        let fields = val.fields.iter().map(Field::from).collect::<Vec<Field>>();
        Self { metadata, fields }
    }
}

impl Hash for BotnetKeyMetadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        for (k, v) in self.field_meta.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for BotnetKeyMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl From<BotnetKeyMetadata> for Bytes {
    fn from(val: BotnetKeyMetadata) -> Self {
        Bytes::from(bincode::serialize(&val).expect("Bad serialization."))
    }
}

impl BotnetKeyMetadata {
    pub fn new(name: &str) -> Self {
        Self {
            type_id: type_id(name),
            field_meta: HashMap::default(),
            name: name.to_string(),
        }
    }

    pub fn from<I>(type_id: usize, name: &str, value: I) -> Self
    where
        I: Iterator<Item = FieldMetadata>,
    {
        let field_meta = value
            .map(|f| (f.name.clone(), f))
            .collect::<HashMap<String, FieldMetadata>>();

        Self {
            type_id,
            field_meta,
            name: name.to_string(),
        }
    }

    pub fn type_id(&self) -> usize {
        self.type_id
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    pub type_id: usize,
    pub name: String,
    pub value: Bytes,
    pub meta: FieldMetadata,
}

impl From<&ConfigField> for Field {
    fn from(val: &ConfigField) -> Self {
        Self::new(&val.name, &val.key, &val.description)
    }
}

impl Field {
    pub fn new(name: &str, key: &str, description: &str) -> Self {
        Self {
            type_id: type_id(name),
            name: name.to_string(),
            value: Bytes::from(key.as_bytes().to_owned()),
            meta: FieldMetadata::new(name, key, description),
        }
    }
}

#[derive(Clone)]
pub struct Extractor {
    #[allow(unused)]
    key: String,
    func: ExtractorFn,
}

impl Debug for Extractor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extractor").finish()?;
        Ok(())
    }
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

#[derive(Default, Clone, Debug)]
pub struct FieldExtractors {
    pub items: HashMap<String, Extractor>,
}

impl FieldExtractors {
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
        let items = value
            .map(|(k, v)| (k.to_string(), Extractor::new(k, v)))
            .collect::<HashMap<String, Extractor>>();

        Self { items }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Metadata {
    items: HashMap<usize, BotnetKeyMetadata>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    pub fn insert(&mut self, ty_id: usize, meta: BotnetKeyMetadata) {
        self.items.insert(ty_id, meta);
    }

    pub fn get(&self, ty_id: &usize) -> BotnetResult<&BotnetKeyMetadata> {
        self.items.get(ty_id).map_or(
            Err(BotnetError::Error("metadata({ty_id}) not found".into())),
            Ok,
        )
    }

    pub fn from<I>(value: I) -> Self
    where
        I: Iterator<Item = (usize, BotnetKeyMetadata)>,
    {
        let items = value.collect::<HashMap<usize, BotnetKeyMetadata>>();

        Self { items }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Extractors {
    items: HashMap<usize, FieldExtractors>,
}

impl Extractors {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    pub fn insert(&mut self, ty_id: usize, exts: FieldExtractors) {
        self.items.insert(ty_id, exts);
    }

    pub fn get(&self, ty_id: &usize) -> BotnetResult<&FieldExtractors> {
        self.items.get(ty_id).map_or(
            Err(BotnetError::Error("extractor({ty_id}) not found".into())),
            Ok,
        )
    }

    pub fn from<I>(value: I) -> Self
    where
        I: Iterator<Item = (usize, FieldExtractors)>,
    {
        let items = value.collect::<HashMap<usize, FieldExtractors>>();

        Self { items }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct BotnetKey {
    metadata: BotnetKeyMetadata,
    fields: Vec<Field>,
}

impl BotnetKey {
    pub fn new(metadata: BotnetKeyMetadata, fields: Vec<Field>) -> Self {
        Self { metadata, fields }
    }

    pub fn flatten(&self) -> Bytes {
        let fields = Bytes::from(
            self.fields
                .iter()
                .map(|f| usize::to_le_bytes(f.type_id).to_vec())
                .collect::<Vec<Vec<u8>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
        );

        let id = Bytes::from(usize::to_le_bytes(self.type_id()).to_vec());
        Bytes::from([id, fields].concat())
    }

    pub fn type_id(&self) -> usize {
        self.metadata.type_id()
    }

    pub fn name(&self) -> String {
        self.metadata.name.clone()
    }

    pub fn from_input(
        value: &Input,
        extractors: &FieldExtractors,
        meta: &BotnetKeyMetadata,
    ) -> BotnetResult<Self> {
        let fields = extractors
            .items
            .iter()
            .map(|e| e.1.call(value).expect("Failed to call on input."))
            .collect::<Vec<Field>>();

        // TODO: use builder pattern
        Ok(BotnetKey {
            fields,
            metadata: meta.clone(),
        })
    }

    pub fn from_bytes(b: Bytes, metadata: &Metadata) -> BotnetResult<BotnetKey> {
        let mut parts = b.chunks_exact(64);
        let mut buff: [u8; 8] = [0u8; 8];

        buff.copy_from_slice(parts.next().unwrap());
        let key_ty_id = usize::from_le_bytes(buff);
        let meta = metadata.get(&key_ty_id)?;

        Ok(BotnetKey {
            metadata: meta.clone(),
            // TODO: finish
            fields: Vec::new(),
        })
    }
}

#[macro_export]
macro_rules! botnet_key {
    ($name: ident) => {
        use botnet_macros::key;

        #[key]
        pub struct $name {
            fields: Vec<Field>,
            metadata: BotnetKeyMetadata,
        }
    };
}

#[derive(Clone, Default, Debug)]
pub struct BotnetParams {
    pub keys: Arc<HashMap<usize, BotnetKey>>,
    pub metadata: Arc<Metadata>,
    pub extractors: Arc<Extractors>,
    pub db: Option<InMemory>,
    pub config: Arc<BotnetConfig>,
}

impl From<BotnetConfig> for BotnetParams {
    fn from(val: BotnetConfig) -> Self {
        let db = match val.database.db_type {
            DbType::InMemory => InMemory::new(),
            _ => unimplemented!(),
        };

        let keys: HashMap<usize, BotnetKey> = HashMap::from_iter(
            val.keys.iter().map(|k| (k.type_id(), BotnetKey::from(k))),
        );

        Self {
            config: val.into(),
            db: Some(db),
            keys: Arc::new(keys),
            ..Self::default()
        }
    }
}

impl BotnetParams {
    pub fn new(
        keys: HashMap<usize, BotnetKey>,
        metadata: Metadata,
        extractors: Extractors,
        db: Option<InMemory>,
        config: BotnetConfig,
    ) -> Self {
        Self {
            keys: Arc::new(keys),
            metadata: Arc::new(metadata),
            extractors: Arc::new(extractors),
            db,
            config: Arc::new(config),
        }
    }
}

#[derive(Clone, Default)]
#[allow(unused)]
pub struct Botnet {
    is_k_anonymous: bool,
    entity_count: u64,
}
