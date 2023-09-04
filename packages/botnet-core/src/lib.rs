#![deny(unused_crate_dependencies)]

#[macro_use]

/// A collection of anomaly detection compatible NoSQL databases.
pub mod database;

/// Botnet configuration.
pub mod config;

/// Utilities used in anomaly detection evaluation.
pub mod eval;

/// A collection of models used used in anomaly detection evaluation.
pub mod models;

/// Utilities used in anomaly detection tasks.
pub mod task;

/// Botnet context.
pub mod context;

pub use bytes::Bytes;
pub use nom::AsBytes;
pub use serde_json::Value as SerdeValue;
pub use url::Url;

use std::{fmt::Debug, io::Error as IoError, sync::PoisonError};
use thiserror::Error;
use tokio::task::JoinError;

/// Result type for used in botnet core operations.
pub type BotnetResult<T> = Result<T, BotnetError>;

/// Extension collections and utils for `botnet_core`.
pub mod ext {
    pub use std::{collections::HashMap, rc::Rc};

    pub use async_std::sync::Arc;

    pub use serde;

    pub use super::Bytes;
}

/// `botnet_core` module prelude.
pub mod prelude {

    pub use super::{BotnetError, BotnetResult};

    /// Re-exports all `botnet_core` models.
    pub use super::models::*;

    /// Re-exports all `botnet_core` database utils.
    pub use super::database::*;

    /// Re-exports `BotnetConfig`.
    pub use super::config::*;

    /// Re-exports botnet evaluators.
    pub use super::eval::Evaluator;

    /// Re-exports botnet strategies.
    pub use super::task::Strategy;

    /// Re-exports botnet context.
    pub use super::context::BotnetContext;

    /// Re-exports `botnet_core::ext`
    pub use super::ext::*;

    /// Re-exports `botnet_core::models`.
    pub use super::models::*;
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
    #[error("Error: {0:#?}")]
    Error(String),
    #[error("JoinError: {0:#?}")]
    JoinError(#[from] JoinError),
    #[error("Lock poisoned.")]
    PoisonError,
    #[error("Error: {0:#?}")]
    UnknownError(#[from] Box<dyn std::error::Error>),
}

impl<T> From<PoisonError<T>> for BotnetError {
    /// Return a `BotnetError` from a `PoisonError`.
    fn from(_e: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}
