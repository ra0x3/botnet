/// A collection of models used used in anomaly detection evaluation.
use crate::AsBytes;
use bytes::Bytes;
use http::Uri;
use url::Url;

/// Basic botnet input type for middleware operations.
pub struct Input(Bytes);

impl Input {
    /// Create a new `Input`.
    pub fn new(value: Bytes) -> Self {
        Self(value)
    }
}

impl From<&'static str> for Input {
    /// Create a new `Input` from a `&'static str`.
    fn from(val: &'static str) -> Self {
        Self(Bytes::from(val.as_bytes()))
    }
}

impl AsBytes for Input {
    /// Create a new `Input` from a `AsBytes`.
    fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for Input {
    /// Create a new `Input` from a `AsRef<str>`.
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).expect("Bad input.")
    }
}

impl From<String> for Input {
    /// Create a new `Input` from a `String`.
    fn from(value: String) -> Self {
        Self(Bytes::from(value))
    }
}

impl From<&Uri> for Input {
    /// Create a new `Input` from a `&Uri`.
    fn from(value: &Uri) -> Self {
        Input::from(value.to_string())
    }
}

impl From<Url> for Input {
    /// Create a new `Input` from a `Url`.
    fn from(value: Url) -> Self {
        Input::from(value.to_string())
    }
}

impl From<Vec<u8>> for Input {
    /// Create a new `Input` from a `Vec<u8>`.
    fn from(value: Vec<u8>) -> Self {
        Self::new(Bytes::from(value))
    }
}
