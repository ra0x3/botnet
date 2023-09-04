/// Utilities used in anomaly detection feature extraction.
use crate::models::*;
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Formatter},
};
use url::Url;

/// Used to ensure all extractor logic conforms to a unified interface.
pub trait Extractor: Send + Sync {
    /// Extract a field from an `Input`.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField>;
}

/// URL extractor.
///
/// A built in botnet extractor.
pub struct UrlExtractor;

impl Extractor for UrlExtractor {
    /// Extract a field from a URL.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
        let url = Url::parse(input.as_ref())?;
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let value = params.get(key).unwrap().to_owned();
        Ok(TransparentField::new(key, &value, None))
    }
}

impl Extractor for Box<dyn Extractor> {
    /// Extract a field from an `Input`.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
        (**self).extract(key, input)
    }
}

impl Extractor for &Box<dyn Extractor> {
    /// Extract a field from an `Input`.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
        (**self).extract(key, input)
    }
}

/// IP extractor.
pub struct IPExtractor;

impl Extractor for IPExtractor {
    /// Extract an IP address from an `Input`.
    ///
    /// `Input` is assumed to be some type of valid URI.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
        let url = Url::parse(input.as_ref())?;

        // FIXME: don't panic
        let value = url.host().unwrap().to_string();
        Ok(TransparentField::new(key, &value, None))
    }
}

/// Default extractor.
pub struct DefaultExtractor;

impl Extractor for DefaultExtractor {
    /// Extract a field from an `Input`.
    ///
    /// This extractor is just used to satisfy the compiler, and should never
    /// actually be instantiated by a user.
    #[allow(unused)]
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<TransparentField> {
        unimplemented!("DefaultExtractor is not implemented.")
    }
}

/// An extraction function used to build `BotnetKey`s. from `Input`s.
pub struct FieldExtractor<T: Extractor> {
    /// Key/identifier of the extractor.
    pub key: String,

    /// Function used to extract a `TransparentField` from an `Input`.
    pub func: T,
}

impl<T: Extractor> fmt::Debug for FieldExtractor<T> {
    /// Debug a `FieldExtractor`.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldExtractor").finish()?;
        Ok(())
    }
}

impl<T: Extractor + Default> Default for FieldExtractor<T> {
    /// Create a default `FieldExtractor`.
    fn default() -> Self {
        Self {
            key: String::default(),
            func: T::default(),
        }
    }
}

impl<T: Extractor> FieldExtractor<T> {
    /// Create a new `FieldExtractor`.
    pub fn new(key: &str, func: T) -> Self {
        Self {
            key: key.to_string(),
            func,
        }
    }
}

impl<T: Extractor> FieldExtractor<T> {
    /// Get function of the extractor.
    pub fn func(&self) -> &dyn Extractor {
        &self.func
    }
}

/// A collection of `FieldExtractor`s for a set of `TransparentField`s.
#[derive(Default, Debug)]
pub struct FieldExtractors {
    /// A map of `FieldExtractor`s.
    pub items: HashMap<String, FieldExtractor<Box<dyn Extractor>>>,
}

impl FieldExtractors {
    /// Create a new `FieldExtractors`.
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    pub fn insert(&mut self, key: String, ext: FieldExtractor<Box<dyn Extractor>>) {
        self.items.insert(key, ext);
    }
}

impl From<Vec<(String, FieldExtractor<Box<dyn Extractor>>)>> for FieldExtractors {
    /// Create a new `FieldExtractors` from a vector of `FieldExtractor`s.
    fn from(items: Vec<(String, FieldExtractor<Box<dyn Extractor>>)>) -> Self {
        let items = HashMap::from_iter(items);
        Self { items }
    }
}

/// A collection of `FieldExtractors`s.
#[derive(Default, Debug)]
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
            Err(BotnetError::Error(format!("extractor({ty_id}) not found"))),
            Ok,
        )
    }
}

impl From<Vec<(usize, FieldExtractors)>> for Extractors {
    /// Create a new `Extractors` from a vector of `FieldExtractors`.
    fn from(items: Vec<(usize, FieldExtractors)>) -> Self {
        let items = HashMap::from_iter(items);
        Self { items }
    }
}
