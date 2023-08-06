/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{BotnetConfig, DbType, Field as ConfigField, Key as ConfigKey},
    database::{Database, InMemory},
    AsBytes, BotnetError, BotnetResult, ExtractorFn,
};
use botnet_utils::type_id;
use bytes::Bytes;
use http::Uri;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Basic botnet input type for middleware operations.
pub struct Input(Bytes);

impl From<&'static str> for Input {
    /// Create a new `Input` from a byte slice.
    fn from(val: &'static str) -> Self {
        Self(Bytes::from(val.as_bytes()))
    }
}

impl AsBytes for Input {
    /// Get the bytes of an `Input`.
    fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for Input {
    /// Return the input as a string.
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).expect("Bad input.")
    }
}

impl From<String> for Input {
    /// Create a new input from a string.
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    /// Create a new input from a string.
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
    }
}

/// Metadata related to a `Field` on a `BotnetKey`.
#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq, PartialEq, Hash)]
pub struct FieldMetadata {
    /// Name of the field.
    name: String,

    /// Key/identifier of the field.
    key: String,

    /// Type id of the field.
    type_id: usize,

    /// Description of the field.
    description: String,
}

impl FieldMetadata {
    /// Create a new `FieldMetadata`.
    pub fn new(name: &str, key: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            type_id: type_id(key),
            description: description.to_string(),
        }
    }
}

/// Metadata related to a `BotnetKey`.
#[derive(Debug, Serialize, Deserialize, Default, Clone, Eq)]
pub struct BotnetKeyMetadata {
    field_meta: HashMap<String, FieldMetadata>,
    type_id: usize,
    name: String,
}

impl From<&ConfigKey> for BotnetKey {
    /// Create a new `BotnetKey` from a `ConfigKey`.
    fn from(val: &ConfigKey) -> Self {
        let metadata = BotnetKeyMetadata::new(val.name());
        let fields = val.fields().iter().map(Field::from).collect::<Vec<Field>>();
        Self { metadata, fields }
    }
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

impl BotnetKeyMetadata {
    /// Create a new `BotnetKeyMetadata`.
    pub fn new(name: &str) -> Self {
        Self {
            type_id: type_id(name),
            field_meta: HashMap::default(),
            name: name.to_string(),
        }
    }

    /// Create a new `BotnetKeyMetadata` from a type, name, and iterator of `FieldMetadata`.    
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

    /// Get the type id of the `BotnetKeyMetadata`.
    pub fn type_id(&self) -> usize {
        self.type_id
    }
}

/// A `Field` on a `BotnetKey`.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Field {
    /// Type id of the field.
    type_id: usize,

    /// Name of the field.
    name: String,

    /// Value of the field.
    value: Bytes,

    /// Metadata of the field.
    meta: FieldMetadata,
}

impl From<&ConfigField> for Field {
    /// Create a new `Field` from a `ConfigField`.
    fn from(val: &ConfigField) -> Self {
        Self::new(val.name(), val.key(), val.description())
    }
}

impl Field {
    /// Create a new `Field`.
    pub fn new(name: &str, key: &str, description: &str) -> Self {
        Self {
            type_id: type_id(name),
            name: name.to_string(),
            value: Bytes::from(key.as_bytes().to_owned()),
            meta: FieldMetadata::new(name, key, description),
        }
    }
}

/// An extraction function used to build `BotnetKey`s. from `Input`s.
#[derive(Clone)]
pub struct Extractor {
    /// Key/identifier of the extractor.
    #[allow(unused)]
    key: String,

    /// Function used to extract a `Field` from an `Input`.
    func: ExtractorFn,
}

impl Debug for Extractor {
    /// Debug a `Extractor`.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extractor").finish()?;
        Ok(())
    }
}

impl Default for Extractor {
    /// Create a default `Extractor`.
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
    /// Create a new `Extractor`.
    pub fn new(key: &str, func: ExtractorFn) -> Self {
        Self {
            key: key.to_string(),
            func,
        }
    }

    /// Call the `Extractor` on an `Input`.
    pub fn call(&self, input: &Input) -> BotnetResult<Field> {
        (self.func)(input)
    }
}

/// A collection of `Extractor`s for a set of `Field`s.
#[derive(Default, Clone, Debug)]
pub struct FieldExtractors {
    /// A map of `Extractor`s.
    pub items: HashMap<String, Extractor>,
}

impl FieldExtractors {
    /// Create a new `FieldExtractors`.
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    /// Add an `Extractor` to the `FieldExtractors`.
    pub fn add(&mut self, key: &str, value: ExtractorFn) {
        self.items
            .insert(key.to_string(), Extractor::new(key, value));
    }

    /// Create a set of `FieldExtractors`s `Extractor` from an iterator of `(&str, ExtractorFn)`.
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

/// A collection of `BotnetKeyMetadata`s.
#[derive(Default, Clone, Debug)]
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
            Err(BotnetError::Error("metadata({ty_id}) not found".into())),
            Ok,
        )
    }

    /// Create a set of `Metadata`s `BotnetKeyMetadata` from an iterator of `(&str, ExtractorFn)`.
    pub fn from<I>(value: I) -> Self
    where
        I: Iterator<Item = (usize, BotnetKeyMetadata)>,
    {
        let items = value.collect::<HashMap<usize, BotnetKeyMetadata>>();

        Self { items }
    }
}

/// A collection of `FieldExtractors`s.
#[derive(Default, Clone, Debug)]
pub struct Extractors {
    /// A mapping of `FieldExtractors`s to the type ID for their respective `BotnetKey`s.
    items: HashMap<usize, FieldExtractors>,
}

impl Extractors {
    /// Create new `Extractors`.
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    /// Add a  set of `FieldExtractors` to the `Extractors`.
    pub fn insert(&mut self, ty_id: usize, exts: FieldExtractors) {
        self.items.insert(ty_id, exts);
    }

    /// Get a set of `FieldExtractors` from the `Extractors`.
    pub fn get(&self, ty_id: &usize) -> BotnetResult<&FieldExtractors> {
        self.items.get(ty_id).map_or(
            Err(BotnetError::Error("extractor({ty_id}) not found".into())),
            Ok,
        )
    }

    /// Create a set of `Extractors`s `FieldExtractors` from an iterator of `(&str, ExtractorFn)`.
    pub fn from<I>(value: I) -> Self
    where
        I: Iterator<Item = (usize, FieldExtractors)>,
    {
        let items = value.collect::<HashMap<usize, FieldExtractors>>();

        Self { items }
    }
}

/// The primary abstraction used used in anomaly detection.
///
/// A `BotnetKey` is constructed from a set of `Field`s, which are extracted from an `Input` using
/// a set of `Extractor`s.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct BotnetKey {
    /// A set of metadata related to the `BotnetKey`.
    metadata: BotnetKeyMetadata,

    /// A set of `Field`s extracted from the `BotnetKey`.
    fields: Vec<Field>,
}

impl BotnetKey {
    /// Create a new `BotnetKey`.
    pub fn new(metadata: BotnetKeyMetadata, fields: Vec<Field>) -> Self {
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
    pub fn name(&self) -> String {
        self.metadata.name.clone()
    }

    /// A set of `Field`s extracted from the `BotnetKey`.
    pub fn fields(&self) -> Vec<Field> {
        self.fields.clone()
    }

    /// Create a `BotnetKey` from an `Input`, set of `FieldExtractors`, and `BotnetKeyMetadata`.
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

    /// Create a `BotnetKey` from bytes and `BotnetKeyMetadata`.
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

/// A set of `Botnet` configuration parameters.
#[derive(Clone, Default, Debug)]
pub struct BotnetParams {
    /// Botnet keys.
    keys: Arc<HashMap<usize, BotnetKey>>,

    /// Botnet metadata.
    metadata: Arc<Metadata>,

    /// Botnet extractors.
    extractors: Arc<Extractors>,

    /// Botnet database.
    db: Option<InMemory>,

    /// Botnet configuration.
    config: Arc<BotnetConfig>,
}

impl From<BotnetConfig> for BotnetParams {
    /// Create a new `BotnetParams` from a `BotnetConfig`.
    fn from(val: BotnetConfig) -> Self {
        let db = match val.database().db_type() {
            DbType::InMemory => InMemory::new(),
            _ => unimplemented!(),
        };

        let keys: HashMap<usize, BotnetKey> = HashMap::from_iter(
            val.keys().iter().map(|k| (k.type_id(), BotnetKey::from(k))),
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
    /// Create a new `BotnetParams`.
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

    /// Return the set of `BotnetKey`s associated with these `BotnetParams`.
    pub fn keys(&self) -> Arc<HashMap<usize, BotnetKey>> {
        self.keys.clone()
    }

    /// Return the set of `Metadata`s associated with these `BotnetParams`.
    pub fn metadata(&self) -> Arc<Metadata> {
        self.metadata.clone()
    }

    /// Return the `Database` associated with these `BotnetParams`.
    pub fn db(&self) -> Option<InMemory> {
        self.db.clone()
    }

    /// Return the `BotnetConfig` associated with these `BotnetParams`.
    pub fn config(&self) -> Arc<BotnetConfig> {
        self.config.clone()
    }

    /// Return the `Extractors` associated with these `BotnetParams`.
    pub fn extractors(&self) -> Arc<Extractors> {
        self.extractors.clone()
    }
}

/// A `Botnet` context with its configuration.
#[derive(Clone, Default)]
#[allow(unused)]
pub struct Botnet {
    /// Whether or not the k-anonimity settings is enabled.
    is_k_anonymous: bool,

    /// Number of entities to count before triggering anomaly detection.
    entity_count: u64,
}
