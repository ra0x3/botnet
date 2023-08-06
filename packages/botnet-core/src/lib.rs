#![deny(unused_crate_dependencies)]

/// Botnet core lib.

#[macro_use]

pub mod database;
pub mod config;
pub mod eval;
pub mod extractor;
pub mod models;
pub mod task;

pub use crate::{
    config::{BotnetConfig, DbType, Field as ConfigField, Key as ConfigKey},
    database::{Database, InMemory},
    extractor::*,
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

/// Result type for used in botnet core operations.
pub type BotnetResult<T> = Result<T, BotnetError>;

/// Function used to exctract a field from an input.
pub type ExtractorFn = fn(&Input) -> BotnetResult<Field>;

/// `botnet_core` module prelude.
pub mod prelude {

    /// Re-exports all `botnet_core` exports.
    pub use super::*;

    /// Re-exports `crate::database::{Database, InMemory}`.
    pub use crate::database::{Database, InMemory};

    /// Re-exports `crate::config::BotnetConfig`.
    pub use crate::config::BotnetConfig;

    /// Re-exports `crate::eval::Evaluator`.
    pub use crate::eval::Evaluator;

    /// Re-exports `crate::task::Strategy`.
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

impl AsRef<str> for Input {
    /// Return the input as a string.
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.0.as_bytes()).expect("Bad input.")
    }
}

impl From<&'static str> for Input {
    /// Create a new input from a string.
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Input {
    /// Create a new input from a string.
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    /// Create a new input from a string.
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
    }
}
