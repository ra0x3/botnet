/// Utilities used in anomaly detection feature extraction.
use crate::{BotnetResult, Field, Input, Url};
use std::collections::HashMap;

/// Used to ensure all extractor logic conforms to a unified interface.
pub trait Extractor {
    /// Extract a field from an input.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<Field>;
}

/// URL extractor.
///
/// A built in botnet extractor.
struct UrlExtractor;

impl Extractor for UrlExtractor {
    /// Extract a field from a URL.
    fn extract(&self, key: &str, input: &Input) -> BotnetResult<Field> {
        let url = Url::parse(input.as_ref())?;
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        let value = params.get(key).unwrap().to_owned();
        Ok(Field::new(key, &value, ""))
    }
}
