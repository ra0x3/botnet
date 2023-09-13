/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{BotnetConfig, DbType, Field, Key},
    database::{InMemory, Store},
    models::{
        extractor::FieldExtractors,
        field::{ExtractedField, FieldMetadata},
        input::Input,
        Metadata,
    },
    AsBytes, BotnetError, BotnetResult,
};
use botnet_utils::type_id;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};

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
                .map(|f| (f.name().to_string(), f.to_owned()))
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

    /// Get the name of the `BotnetKeyMetadata`.
    pub fn field_meta(&self) -> &HashMap<String, FieldMetadata> {
        &self.field_meta
    }
}

/// The primary abstraction used used in anomaly detection.
///
/// A `BotnetKey` is constructed from a set of `ExtractedField`s, which are extracted from an `Input` using
/// a set of `FieldExtractor`s.
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct BotnetKey {
    /// A set of metadata related to the `BotnetKey`.
    metadata: BotnetKeyMetadata,

    /// A set of `ExtractedField`s extracted from the `BotnetKey`.
    fields: Vec<ExtractedField>,
}

impl BotnetKey {
    /// Create a new `BotnetKey`.
    pub fn new(metadata: BotnetKeyMetadata, fields: Vec<ExtractedField>) -> Self {
        Self { metadata, fields }
    }

    /// Flatten a `BotnetKey` to bytes.
    pub fn flatten(&self) -> Bytes {
        let fields = Bytes::from(
            self.fields
                .iter()
                .map(|f| usize::to_le_bytes(f.type_id()).to_vec())
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

    /// A set of `ExtractedField`s extracted from the `BotnetKey`.
    pub fn fields(&self) -> &Vec<ExtractedField> {
        &self.fields
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

impl From<(&Input, &FieldExtractors, &BotnetKeyMetadata)> for BotnetKey {
    /// Create a `BotnetKey` from an `Input`, set of `FieldExtractors`, and `BotnetKeyMetadata`.
    fn from(val: (&Input, &FieldExtractors, &BotnetKeyMetadata)) -> Self {
        let keys = val.2.field_meta.keys().collect::<Vec<&String>>();
        let fields = keys
            .iter()
            .zip(val.1.items.values())
            .map(|(k, e)| {
                e.func()
                    .extract(k, val.0)
                    .expect("Failed to call on input.")
            })
            .collect::<Vec<ExtractedField>>();
        Self {
            fields,
            metadata: val.2.to_owned(),
        }
    }
}
