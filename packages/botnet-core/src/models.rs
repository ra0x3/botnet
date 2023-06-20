pub use crate::{
    config::{BotnetConfig, DbType, Field as ConfigField, Key as ConfigKey},
    database::{Database, InMemory},
    BotnetError, BotnetResult, ExtractorFn,
};
use botnet_utils::type_id;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

pub struct Input(pub Bytes);

impl Input {
    pub fn new(s: &'static str) -> Self {
        Self(Bytes::from(s.as_bytes()))
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
