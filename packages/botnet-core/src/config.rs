use crate::{
    task::{EntityCounter, KAnonimity, Rate},
    BotnetResult,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Version {
    #[default]
    V1,
    V2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Field {
    name: String,
    key: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Key {
    name: String,
    fields: Vec<Field>,
}

impl Key {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct KAnonConfig {
    k: KAnonimity,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Strategy {
    entity_counter: EntityCounter,
    kanon: KAnonConfig,
    rate: Rate,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotnetConfig {
    version: Version,
    strategy: Strategy,
    keys: Vec<Key>,
}

impl BotnetConfig {
    pub fn from_path(value: Option<PathBuf>) -> BotnetResult<Self> {
        let value = value.unwrap_or(PathBuf::from(""));
        let mut file = File::open(value)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: BotnetConfig = serde_yaml::from_str(&content)?;

        Ok(config)
    }

    pub fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}
