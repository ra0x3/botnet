/// A collection of models used used in anomaly detection evaluation.
pub use crate::{
    config::{Anonimity, BotnetConfig, DbType, EntityCounter, Field, Key, RateLimit},
    database::{InMemory, Store},
    models::{
        extractor::{Extractors, FieldExtractors},
        key::{BotnetKey, BotnetKeyMetadata},
        Metadata,
    },
    AsBytes, BotnetError, BotnetResult,
};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

/// Bontet context.
///
/// The interface through which the botnet is configured and used.
#[derive(Clone, Default, Debug)]
pub struct BotnetContext<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Botnet keys.
    keys: Arc<HashMap<usize, Key>>,

    /// Botnet metadata.
    metadata: Arc<Metadata>,

    /// Botnet extractors.
    extractors: Arc<Extractors>,

    /// Botnet database.
    db: Option<InMemory>,

    /// Botnet configuration.
    config: Arc<BotnetConfig<E, A, C>>,
}

impl<E, A, C> BotnetContext<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Create a new `BotnetContext`.
    pub fn new(
        metadata: Metadata,
        extractors: Extractors,
        db: Option<InMemory>,
        config: BotnetConfig<E, A, C>,
    ) -> Self {
        let keys = config.keys.iter().fold(HashMap::new(), |mut acc, k| {
            acc.insert(k.type_id(), k.clone());
            acc
        });
        Self {
            keys: Arc::new(keys),
            metadata: Arc::new(metadata),
            extractors: Arc::new(extractors),
            db,
            config: Arc::new(config),
        }
    }

    /// Return the set of `Key`s associated with these `Botnet`.
    pub fn keys(&self) -> &Arc<HashMap<usize, Key>> {
        &self.keys
    }

    /// Return the `Key` associated with a particular type ID.
    pub fn get_key(&self, ty_id: &usize) -> Option<&Key> {
        self.keys.get(ty_id)
    }

    /// Return the `Store` associated with these `Botnet`.
    pub fn db(&self) -> Option<InMemory> {
        self.db.clone()
    }

    /// Return the `BotnetConfig` associated with these `Botnet`.
    pub fn config(&self) -> Arc<BotnetConfig<E, A, C>> {
        self.config.clone()
    }

    /// Return the `Extractors` for a particular `BotnetKey`.
    pub fn get_extractors(&self, ty_id: &usize) -> BotnetResult<&FieldExtractors> {
        self.extractors.get(ty_id)
    }

    /// Return metadata for a particular `BotnetKey`.
    pub fn get_metadata(&self, ty_id: &usize) -> BotnetResult<&BotnetKeyMetadata> {
        self.metadata.get(ty_id)
    }
}
