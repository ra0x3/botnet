/// Botnet configuration.
use crate::BotnetResult;
use botnet_utils::type_id;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};

/// Class of entity.
///
/// A fundamental concept in botnet is the entity. An entity can be associated with one or more `BotnetKey`s.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum EntityClass {
    /// Entities identified by IP address and user agent.
    #[default]
    IpUa,

    /// Entities identified by none of the above (i.e. other).
    Other,
}

/// Level of k-anonimity.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum KAnonimity {
    /// 100-anonimity.
    ///
    /// At least 100 entities must be present in this botnet key in order for anomaly detection to be performed.
    #[default]
    K100,

    /// 800-anonimity.
    ///
    /// At least 800 entities must be present in this botnet key in order for anomaly detection to be performed.
    K800,

    /// 8000-anonimity.
    ///
    /// At least 8000 entities must be present in this botnet key in order for anomaly detection to be performed.
    K8000,
}

impl KAnonimity {
    /// Get the k value.
    pub fn k(&self) -> u64 {
        match self {
            Self::K100 => 100,
            Self::K800 => 800,
            Self::K8000 => 8000,
        }
    }
}

/// Anomaly detection cliff detector.
///
/// Used to catch traffic with a high rate of change, over a short period of time.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum CliffDetector {
    /// Version 1 of the cliff detector.
    #[default]
    V1,

    /// Version 2 of the cliff detector.
    V2,
}

/// Botnet configuration version.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Version {
    /// Version 1 of the botnet configuration.
    #[default]
    V1,

    /// Version 2 of the botnet configuration.
    V2,
}

/// Botnet `Field` configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Field {
    /// Name of the field.
    pub name: String,

    /// Key/identifier of the field.
    pub key: String,

    /// Description of the field.
    pub description: String,
}

/// Botnet `Key` configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Key {
    /// Name of the key.
    pub name: String,

    /// Description of the key.
    pub fields: Vec<Field>,
}

impl Key {
    /// Get the name of the key.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the type ID of the key.
    pub fn type_id(&self) -> usize {
        type_id(self.name.as_bytes())
    }
}

/// Botnet K-Anon configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KAnon {
    /// K-Anonimity level.
    pub k: KAnonimity,

    /// K-Anonimity enabled.
    pub enabled: bool,
}

/// Botnet entity counting configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EntityCounter {
    /// Entity counting enabled.
    pub enabled: bool,

    /// Entity class.
    pub counter: EntityClass,
}

/// Botnet cliff detection configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CliffDetection {
    /// Cliff detection enabled.
    pub enabled: bool,

    /// Cliff detector.
    pub detector: CliffDetector,
}

/// Botnet strategy configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Strategy {
    /// Entity counting configuration.
    pub entity: EntityCounter,

    /// K-Anon configuration.
    pub kanon: KAnon,

    /// Cliff detection configuration.
    pub cliff: CliffDetection,
}

/// Botnet database type configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum DbType {
    /// In-memory database.
    #[default]
    InMemory,

    /// Redis database.
    Redis,
}

/// Database configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Database {
    /// Database type.
    pub db_type: DbType,

    /// Database URI.
    pub uri: Option<String>,
}

/// Botnet configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotnetConfig {
    /// Botnet config version.
    pub version: Version,

    /// Botnet config strategy.
    pub strategy: Strategy,

    /// Botnet config keys.
    pub keys: Vec<Key>,

    /// Database configuration.
    pub database: Database,
}

impl BotnetConfig {
    /// Create a new botnet configuration.
    pub fn from_path(value: Option<PathBuf>) -> BotnetResult<Self> {
        let value = value.unwrap_or(PathBuf::from(""));
        let mut file = File::open(value)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: BotnetConfig = serde_yaml::from_str(&content)?;

        Ok(config)
    }

    /// Get the keys for this configuration.
    pub fn keys(&self) -> &Vec<Key> {
        &self.keys
    }
}
