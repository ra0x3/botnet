use crate::{BotnetError, BotnetResult};
use std::collections::HashMap;

/// Botnet core field models.
pub mod field;

/// Botnet core key models.
pub mod key;

/// Botnet core input models.
pub mod input;

/// Bonet core extractor models.
pub mod extractor;

/// A collection of `BotnetKeyMetadata`s.
#[derive(Default, Debug)]
pub struct Metadata {
    /// A mapping of `BotnetKeyMetadata`s to the type ID for their respective `BotnetKey`s.
    items: HashMap<usize, key::BotnetKeyMetadata>,
}

impl Metadata {
    /// Create a new `Metadata`.
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    /// Add a set of `BotnetKeyMetadata` to the `Metadata`.
    pub fn insert(&mut self, ty_id: usize, meta: key::BotnetKeyMetadata) {
        self.items.insert(ty_id, meta);
    }

    /// Get a set of `BotnetKeyMetadata` from the `Metadata`.
    pub fn get(&self, ty_id: &usize) -> BotnetResult<&key::BotnetKeyMetadata> {
        self.items.get(ty_id).map_or(
            Err(BotnetError::Error(format!("metadata({ty_id}) not found"))),
            Ok,
        )
    }
}

impl<I> From<I> for Metadata
where
    I: Iterator<Item = (usize, key::BotnetKeyMetadata)>,
{
    /// Create a set of `Metadata`s `BotnetKeyMetadata` from an iterator of `(&str, ExtractorFn)`.
    fn from(value: I) -> Self {
        let items = value.collect::<HashMap<usize, key::BotnetKeyMetadata>>();

        Self { items }
    }
}
