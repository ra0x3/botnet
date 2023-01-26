use bytes::Bytes;
use nom::AsBytes;
use serde_json::Value as SerdeValue;
use thiserror::Error;

pub type AnomalyDetectorResult<T> = Result<T, AnomalyDetectorError>;

#[derive(Debug, Error)]
pub enum AnomalyDetectorError {
    #[error("ParseError: {0:#?}")]
    ParseError(#[from] url::ParseError),
}

pub type Key = &'static str;
pub type Value = &'static str;

#[derive(Debug, Default)]
pub struct Input(pub Bytes);

impl Input {
    fn new(data: Bytes) -> Self {
        Self(data)
    }
}

impl From<Bytes> for Input {
    fn from(b: Bytes) -> Self {
        Self::new(b)
    }
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Self {
        Self(Bytes::from(s))
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.0.as_ref()).unwrap()
    }
}

pub trait AnomalyKey {
    type Input;
    fn set(&mut self, key: &str, value: &str, description: &str) -> Self;
    fn as_json() -> SerdeValue;
}

pub trait Extractor {
    type Input;
    fn extract(&self) -> AnomalyDetectorResult<(Key, Value)>;
}
