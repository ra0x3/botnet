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
    name: String,

    /// Key/identifier of the field.
    key: String,

    /// Description of the field.
    description: String,
}

impl Field {
    /// Name of the field.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Key/identifier of the field.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Description of the field.
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Botnet `Key` configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Key {
    /// Name of the key.
    name: String,

    /// Fields associated with this `Key`.
    fields: Vec<Field>,
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

    /// Fields associated with this `Key`.
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

/// Botnet K-Anon configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KAnon {
    /// K-Anonimity level.
    k: KAnonimity,

    /// K-Anonimity enabled.
    enabled: bool,
}

impl KAnon {
    /// K-Anonimity level.
    pub fn k(&self) -> u64 {
        self.k.k()
    }

    /// K-Anonimity enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Botnet entity counting configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EntityCounter {
    /// Entity counting enabled.
    enabled: bool,

    /// Entity class.
    counter: EntityClass,
}

impl EntityCounter {
    /// Entity counting enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Entity class.
    pub fn counter(&self) -> &EntityClass {
        &self.counter
    }
}

/// Botnet cliff detection configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CliffDetection {
    /// Cliff detection enabled.
    enabled: bool,

    /// Cliff detector.
    detector: CliffDetector,
}

impl CliffDetection {
    /// Cliff detection enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Cliff detector.
    pub fn detector(&self) -> &CliffDetector {
        &self.detector
    }
}

/// Botnet strategy configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Strategy {
    /// Entity counting configuration.
    entity: EntityCounter,

    /// K-Anon configuration.
    kanon: KAnon,

    /// Cliff detection configuration.
    cliff: CliffDetection,
}

impl Strategy {
    /// Entity counting configuration.
    pub fn entity(&self) -> &EntityCounter {
        &self.entity
    }

    /// K-Anon configuration.
    pub fn kanon(&self) -> &KAnon {
        &self.kanon
    }

    /// Cliff detection configuration.
    pub fn cliff(&self) -> &CliffDetection {
        &self.cliff
    }
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
    db_type: DbType,

    /// Database URI.
    uri: Option<String>,
}

impl Database {
    /// Database type.
    pub fn db_type(&self) -> &DbType {
        &self.db_type
    }

    /// Database URI.
    pub fn uri(&self) -> Option<&String> {
        self.uri.as_ref()
    }
}

/// Botnet configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotnetConfig {
    /// Botnet config version.
    version: Version,

    /// Botnet config strategy.
    strategy: Strategy,

    /// Botnet config keys.
    keys: Vec<Key>,

    /// Database configuration.
    database: Database,
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

    /// Get the strategy for this configuration.
    pub fn strategy(&self) -> &Strategy {
        &self.strategy
    }

    /// Get the database for this configuration.
    pub fn database(&self) -> &Database {
        &self.database
    }
}
