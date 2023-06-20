#![deny(unused_crate_dependencies)]
#[macro_use]

pub mod database;
pub mod config;
pub mod eval;
pub mod models;
pub mod task;

pub use crate::{
    config::{BotnetConfig, DbType, Field as ConfigField, Key as ConfigKey},
    database::{Database, InMemory},
    models::*,
};
pub use bytes::Bytes;
pub use nom::AsBytes;
pub use serde_json::Value as SerdeValue;
pub use url::Url;

use http::Uri;
use std::{fmt::Debug, io::Error as IoError, sync::PoisonError};
use thiserror::Error;
use tokio::task::JoinError;

pub type BotnetResult<T> = Result<T, BotnetError>;
pub type ExtractorFn = fn(&Input) -> BotnetResult<Field>;

pub mod prelude {

    pub use super::{
        Botnet, BotnetKey, BotnetKeyMetadata, BotnetParams, BotnetResult, Extractor,
        ExtractorFn, Extractors, Field, FieldExtractors, FieldMetadata, Input, Metadata,
        Url,
    };

    pub use crate::database::{Database, InMemory};

    pub use crate::config::BotnetConfig;

    pub use crate::eval::Evaluator;

    pub use crate::task::Strategy;
}

#[derive(Debug, Error)]
pub enum BotnetError {
    #[error("ParseError: {0:#?}")]
    ParseError(#[from] url::ParseError),
    #[error("BincodeError: {0:#?}")]
    BincodeError(#[from] bincode::Error),
    #[error("Utf8Error: {0:#?}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[cfg(feature = "redisdb")]
    #[error("RedisError: {0:#?}")]
    RedisError(#[from] redis::RedisError),
    #[error("SerdeYamlError: {0:#?}")]
    SerdeYamlError(#[from] serde_yaml::Error),
    #[error("IoError: {0:#?}")]
    IoError(#[from] IoError),
    #[error("Error")]
    Error(#[from] Box<dyn std::error::Error>),
    #[error("JoinError: {0:#?}")]
    JoinError(#[from] JoinError),
    #[error("Lock poisoned.")]
    PoisonError,
}

impl<T> From<PoisonError<T>> for BotnetError {
    fn from(_e: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.0.as_bytes()).expect("Bad input.")
    }
}

impl From<&'static str> for Input {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
    }
}
