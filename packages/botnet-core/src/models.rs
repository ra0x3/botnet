/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{BotnetConfig, DbType, Field, Key},
    database::{Database, InMemory},
    extractor::*,
    AsBytes, BotnetError, BotnetResult, ExtractorFn,
};
use botnet_utils::type_id;
use bytes::Bytes;
use http::Uri;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};
use url::Url;

/// Basic botnet input type for middleware operations.
pub struct Input(Bytes);

impl Input {
    /// Create a new `Input`.
    pub fn new(value: Bytes) -> Self {
        Self(value)
    }
}

impl From<&'static str> for Input {
    /// Create a new `Input` from a `&'static str`.
    fn from(val: &'static str) -> Self {
        Self(Bytes::from(val.as_bytes()))
    }
}

impl AsBytes for Input {
    /// Create a new `Input` from a `AsBytes`.
    fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for Input {
    /// Create a new `Input` from a `AsRef<str>`.
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).expect("Bad input.")
    }
}

impl From<String> for Input {
    /// Create a new `Input` from a `String`.
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    /// Create a new `Input` from a `&Uri`.
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
    }
}

impl From<Url> for Input {
    /// Create a new `Input` from a `Url`.
    fn from(value: Url) -> Self {
        Input::from(value.to_string())
    }
}

impl From<Vec<u8>> for Input {
    /// Create a new `Input` from a `Vec<u8>`.
    fn from(value: Vec<u8>) -> Self {
        Self::new(Bytes::from(value))
    }
}

/// Metadata related to a `TransparentField` on a `BotnetKey`.
#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Hash, Clone)]
pub struct FieldMetadata {
    /// Name of the field.
    name: String,

    /// Key/identifier of the field.
    key: String,

    /// Type id of the field.
    type_id: usize,

    /// Description of the field.
    description: Option<String>,
}

impl FieldMetadata {
    /// Create a new `FieldMetadata`.
    pub fn new(name: &str, key: &str, description: Option<&String>) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            type_id: type_id(key),
            description: description.cloned(),
        }
    }
}

impl From<&Field> for FieldMetadata {
    /// Create a new `FieldMetadata` from a `Field`.
    fn from(val: &Field) -> Self {
        Self::new(&val.name, &val.key, val.description.as_ref())
    }
}

/// Metadata related to a `BotnetKey`.
#[derive(Debug, Serialize, Deserialize, Default, Eq, Clone)]
pub struct BotnetKeyMetadata {
    /// A mapping of `FieldMetadata`s to their respective field names.
    field_meta: HashMap<String, FieldMetadata>,

    /// Type ID of key
    type_id: usize,

    /// Name of the key.
    name: String,
}

impl Hash for BotnetKeyMetadata {
    /// Hash a `BotnetKeyMetadata`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        for (k, v) in self.field_meta.iter() {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for BotnetKeyMetadata {
    /// Compare two `BotnetKeyMetadata`s.
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl From<BotnetKeyMetadata> for Bytes {
    /// Serialize a `BotnetKeyMetadata` to bytes.
    fn from(val: BotnetKeyMetadata) -> Self {
        Bytes::from(bincode::serialize(&val).expect("Bad serialization."))
    }
}

impl From<(usize, &str, Vec<FieldMetadata>)> for BotnetKeyMetadata {
    /// Create a new `BotnetKeyMetadata` from a type, name, and iterator of `FieldMetadata`.    
    fn from(val: (usize, &str, Vec<FieldMetadata>)) -> Self {
        Self {
            type_id: val.0,
            field_meta: val
                .2
                .iter()
                .map(|f| (f.name.to_string(), f.to_owned()))
                .collect::<HashMap<String, FieldMetadata>>(),
            name: val.1.to_string(),
        }
    }
}

impl BotnetKeyMetadata {
    /// Create a new `BotnetKeyMetadata`.
    pub fn new(name: &str) -> Self {
        Self {
            type_id: type_id(name),
            field_meta: HashMap::default(),
            name: name.to_string(),
        }
    }

    /// Get the type id of the `BotnetKeyMetadata`.
    pub fn type_id(&self) -> usize {
        self.type_id
    }
}

/// A `TransparentField` on a `BotnetKey`.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TransparentField {
    /// Type id of the field.
    type_id: usize,

    /// Name of the field.
    name: String,

    /// Value of the field.
    value: Bytes,

    /// Metadata of the field.
    meta: FieldMetadata,
}

impl From<&Field> for TransparentField {
    /// Create a new `TransparentField` from a `Field`.
    fn from(val: &Field) -> Self {
        Self::new(&val.name, &val.key, val.description.as_ref())
    }
}

impl TransparentField {
    /// Create a new `TransparentField`.
    pub fn new(name: &str, key: &str, description: Option<&String>) -> Self {
        Self {
            type_id: type_id(name),
            name: name.to_string(),
            value: Bytes::default(),
            meta: FieldMetadata::new(name, key, description),
        }
    }
}

/// A collection of `BotnetKeyMetadata`s.
#[derive(Default, Debug)]
pub struct Metadata {
    /// A mapping of `BotnetKeyMetadata`s to the type ID for their respective `BotnetKey`s.
    items: HashMap<usize, BotnetKeyMetadata>,
}

impl Metadata {
    /// Create a new `Metadata`.
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    /// Add a set of `BotnetKeyMetadata` to the `Metadata`.
    pub fn insert(&mut self, ty_id: usize, meta: BotnetKeyMetadata) {
        self.items.insert(ty_id, meta);
    }

    /// Get a set of `BotnetKeyMetadata` from the `Metadata`.
    pub fn get(&self, ty_id: &usize) -> BotnetResult<&BotnetKeyMetadata> {
        self.items.get(ty_id).map_or(
            Err(BotnetError::Error(format!("metadata({ty_id}) not found"))),
            Ok,
        )
    }
}

impl<I> From<I> for Metadata
where
    I: Iterator<Item = (usize, BotnetKeyMetadata)>,
{
    /// Create a set of `Metadata`s `BotnetKeyMetadata` from an iterator of `(&str, ExtractorFn)`.
    fn from(value: I) -> Self {
        let items = value.collect::<HashMap<usize, BotnetKeyMetadata>>();

        Self { items }
    }
}

/// The primary abstraction used used in anomaly detection.
///
/// A `BotnetKey` is constructed from a set of `TransparentField`s, which are extracted from an `Input` using
/// a set of `FieldExtractor`s.
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct BotnetKey {
    /// A set of metadata related to the `BotnetKey`.
    metadata: BotnetKeyMetadata,

    /// A set of `TransparentField`s extracted from the `BotnetKey`.
    fields: Vec<TransparentField>,
}

impl From<&Key> for BotnetKey {
    /// Create a new `BotnetKey` from a `Key`.
    fn from(val: &Key) -> Self {
        let metadata = BotnetKeyMetadata::new(&val.name);
        let fields = val
            .fields
            .iter()
            .map(TransparentField::from)
            .collect::<Vec<TransparentField>>();
        Self { metadata, fields }
    }
}

impl BotnetKey {
    /// Create a new `BotnetKey`.
    pub fn new(metadata: BotnetKeyMetadata, fields: Vec<TransparentField>) -> Self {
        Self { metadata, fields }
    }

    /// Flatten a `BotnetKey` to bytes.
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

    /// Get the type id of the `BotnetKey`.
    pub fn type_id(&self) -> usize {
        self.metadata.type_id()
    }

    /// Get the name of the `BotnetKey`.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// A set of `TransparentField`s extracted from the `BotnetKey`.
    pub fn fields(&self) -> &Vec<TransparentField> {
        &self.fields
    }

    /// Create a `BotnetKey` from an `Input`, set of `FieldExtractors`, and `BotnetKeyMetadata`.
    pub fn from_input(
        value: &Input,
        extractors: &FieldExtractors,
        meta: &BotnetKeyMetadata,
    ) -> BotnetResult<Self> {
        let keys = meta.field_meta.keys().collect::<Vec<&String>>();
        let fields = keys
            .iter()
            .zip(extractors.items.values())
            .map(|(k, e)| {
                e.func()
                    .extract(k, value)
                    .expect("Failed to call on input.")
            })
            .collect::<Vec<TransparentField>>();

        Ok(Self {
            fields,
            metadata: meta.to_owned(),
        })
    }

    /// Create a `BotnetKey` from bytes and `BotnetKeyMetadata`.
    pub fn from_bytes(b: Bytes, metadata: &Metadata) -> BotnetResult<Self> {
        let mut parts = b.chunks_exact(64);
        let mut buff: [u8; 8] = [0u8; 8];

        buff.copy_from_slice(parts.next().unwrap());
        let key_ty_id = usize::from_le_bytes(buff);
        let meta = metadata.get(&key_ty_id)?;

        Ok(Self {
            metadata: meta.to_owned(),
            // TODO: finish
            fields: Vec::new(),
        })
    }
}
