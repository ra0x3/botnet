#![deny(unused_crate_dependencies)]

#[macro_use]

/// A collection of anomaly detection compatible NoSQL databases.
pub mod database;

/// Botnet configuration.
pub mod config;

/// Utilities used in anomaly detection evaluation.
pub mod eval;

/// Utilities used in anomaly detection feature extraction.
pub mod extractor;

/// A collection of models used used in anomaly detection evaluation.
pub mod models;

/// Utilities used in anomaly detection tasks.
pub mod task;

pub use bytes::Bytes;
pub use nom::AsBytes;
pub use serde_json::Value as SerdeValue;
pub use url::Url;

use std::{fmt::Debug, io::Error as IoError, sync::PoisonError};
use thiserror::Error;
use tokio::task::JoinError;

use crate::models::{Field, Input};

/// Result type for used in botnet core operations.
pub type BotnetResult<T> = Result<T, BotnetError>;

/// Function used to exctract a field from an input.
pub type ExtractorFn = fn(&Input) -> BotnetResult<Field>;

/// `botnet_core` module prelude.
pub mod prelude {

    /// Re-exports all `botnet_core` models.
    pub use super::models::*;

    /// Re-exports all `botnet_core` database utils.
    pub use crate::database::*;

    /// Re-exports `BotnetConfig`.
    pub use crate::config::BotnetConfig;

    /// Re-exports botnet evaluators.
    pub use crate::eval::Evaluator;

    /// Re-exports botnet strategies.
    pub use crate::task::Strategy;
}

/// Error type for used in botnet core operations.
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
    /// Return a `BotnetError` from a `PoisonError`.
    fn from(_e: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}
