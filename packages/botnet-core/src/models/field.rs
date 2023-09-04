/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{BotnetConfig, DbType, Field, Key},
    database::{InMemory, Store},
    models::extractor::{Extractors, FieldExtractors},
    AsBytes, BotnetError, BotnetResult,
};
use botnet_utils::type_id;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Metadata related to a `ExtractedField` on a `BotnetKey`.
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

    /// Get the name of the `FieldMetadata`.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&Field> for FieldMetadata {
    /// Create a new `FieldMetadata` from a `Field`.
    fn from(val: &Field) -> Self {
        Self::new(&val.name, &val.key, val.description.as_ref())
    }
}

/// A `ExtractedField` on a `BotnetKey`.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct ExtractedField {
    /// Type id of the field.
    type_id: usize,

    /// Name of the field.
    key: String,

    /// Value of the field.
    value: Bytes,

    /// Metadata of the field.
    meta: FieldMetadata,
}

// impl From<&Field> for ExtractedField {
//     /// Create a new `ExtractedField` from a `Field`.
//     fn from(val: &Field) -> Self {
//         Self::new(&val.name, Bytes::default())
//     }
// }

impl ExtractedField {
    /// Create a new `ExtractedField`.
    pub fn new(key: &str, value: Bytes) -> Self {
        Self {
            type_id: type_id(key),
            key: key.to_string(),
            value,
            meta: FieldMetadata::default(),
        }
    }

    /// Get the type id of the `ExtractedField`.
    pub fn type_id(&self) -> usize {
        self.type_id
    }

    /// Get the key of the `ExtractedField`.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the value of the `ExtractedField`.
    pub fn value(&self) -> &Bytes {
        &self.value
    }
}
