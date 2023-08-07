/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{BotnetConfig, DbType, Field, Key},
    database::{Database, InMemory},
    models::*,
    AsBytes, BotnetError, BotnetResult, ExtractorFn,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// Bontet context.
#[derive(Clone, Default, Debug)]
pub struct BotnetContext {
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

impl BotnetContext {
    /// Create a new `BotnetContext`.
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

    /// Return the set of `BotnetKey`s associated with these `Botnet`.
    pub fn keys(&self) -> Arc<HashMap<usize, BotnetKey>> {
        self.keys.clone()
    }

    /// Return the set of `Metadata`s associated with these `Botnet`.
    pub fn metadata(&self) -> Arc<Metadata> {
        self.metadata.clone()
    }

    /// Return the `Database` associated with these `Botnet`.
    pub fn db(&self) -> Option<InMemory> {
        self.db.clone()
    }

    /// Return the `BotnetConfig` associated with these `Botnet`.
    pub fn config(&self) -> Arc<BotnetConfig> {
        self.config.clone()
    }

    /// Return the `Extractors` associated with these `Botnet`.
    ///
    /// These `Extractors` are the literal extractor object, whereas the data
    /// in `BotnetKey`s are the _result_ of applying the `Extractors`.
    pub fn extractors(&self) -> Arc<Extractors> {
        self.extractors.clone()
    }

    /// Count this entity in the stratey.
    pub fn count_entity(&self, k: &BotnetKey) -> BotnetResult<usize> {
        let mut db = self.db().expect("Database expected.");
        db.incr_key(k)
    }
}
