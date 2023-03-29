use crate::BotnetResult;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Field {
    name: String,
    key: Vec<u8>,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Key {
    name: String,
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    keys: Vec<Key>,
}

impl Config {
    pub fn from_path(value: impl AsRef<Path>) -> BotnetResult<Self> {
        let mut file = File::open(&value)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Config = serde_yaml::from_str(&content)?;

        Ok(config)
    }
}
