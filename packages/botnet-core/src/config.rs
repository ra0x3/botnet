use crate::BotnetResult;
use botnet_utils::type_id;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum EntityClass {
    #[default]
    IpUa,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum KAnonimity {
    #[default]
    K100,
    K800,
    K8000,
}

impl KAnonimity {
    pub fn k(&self) -> u64 {
        match self {
            Self::K100 => 100,
            Self::K800 => 800,
            Self::K8000 => 8000,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum CliffDetector {
    #[default]
    V1,
    V2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Version {
    #[default]
    V1,
    V2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Field {
    pub name: String,
    pub key: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Key {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Key {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn type_id(&self) -> usize {
        type_id(self.name.as_bytes())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KAnon {
    pub k: KAnonimity,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EntityCounter {
    pub enabled: bool,
    pub counter: EntityClass,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CliffDetection {
    pub enabled: bool,
    pub detector: CliffDetector,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Strategy {
    pub entity: EntityCounter,
    pub kanon: KAnon,
    pub cliff: CliffDetection,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum DbType {
    #[default]
    InMemory,
    Redis,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Database {
    pub db_type: DbType,
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotnetConfig {
    pub version: Version,
    pub strategy: Strategy,
    pub keys: Vec<Key>,
    pub database: Database,
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
