use crate::BotnetResult;
/// Botnet configuration.
use botnet_utils::type_id;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::{fmt, fs::File, io::Read, marker::PhantomData, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CliffDetector {
    /// Version of the cliff detector.
    pub version: String,

    pub enabled: bool,

    pub class: String,
}

impl RateLimit for CliffDetector {
    /// Register a hit.
    fn register(&self) -> BotnetResult<()> {
        Ok(())
    }

    /// Check if the cliff has been hit.
    fn has_hit_cliff(&self) -> BotnetResult<bool> {
        Ok(false)
    }

    /// Check if the rate limit is enabled.
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn class(&self) -> &str {
        &self.class
    }
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

/// Botnet `ExtractedField` configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Field {
    /// Name of the field.
    pub name: String,

    /// Key/identifier of the field.
    pub key: String,

    /// Description of the field.
    pub description: Option<String>,

    /// Extractor for this field.
    pub extractor: String,
}

impl Field {
    /// Get the type ID of the field.
    pub fn type_id(&self) -> usize {
        type_id(self.name.as_bytes())
    }
}

/// Botnet `Key` configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Key {
    /// Name of the key.
    pub name: String,

    /// Fields associated with this `Key`.
    pub fields: Vec<Field>,
}

impl Key {
    /// Get the type ID of the key.
    pub fn type_id(&self) -> usize {
        type_id(self.name.as_bytes())
    }
}

/// Botnet K-Anon configuration.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct KAnonimity {
    /// K-Anonimity level.
    pub k: u64,

    /// K-Anonimity enabled.
    pub enabled: bool,

    pub class: String,
}

impl Anonimity for KAnonimity {
    fn class(&self) -> &str {
        &self.class
    }

    /// Check if the value is anonymous.
    fn is_anonymous(&self, x: u64) -> bool {
        x >= self.k
    }

    /// Check if the anonimity is enabled.
    fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Botnet entity counting configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IPUAEntityCounter {
    /// Entity counting enabled.
    pub enabled: bool,

    pub class: String,
}

impl EntityCounter for IPUAEntityCounter {
    fn class(&self) -> &str {
        &self.class
    }

    /// Count this entity.
    fn count(&self) -> BotnetResult<u64> {
        Ok(0)
    }

    /// Check if the entity counting is enabled.
    fn enabled(&self) -> bool {
        self.enabled
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum BotnetAnonimity<A: Anonimity> {
    KAnonimity(A),
}

impl<'de, A: Anonimity + Deserialize<'de>> serde::Deserialize<'de>
    for BotnetAnonimity<A>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FooAnonimityVisitor<A: Anonimity>(PhantomData<A>);

        impl<'de, A: Anonimity + Deserialize<'de>> Visitor<'de> for FooAnonimityVisitor<A> {
            type Value = BotnetAnonimity<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum BotnetAnonimity")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    if key == "kanonimity" {
                        let value = map.next_value()?;
                        return Ok(BotnetAnonimity::KAnonimity(value));
                    }
                }

                Err(de::Error::custom("Unexpected keys in map"))
            }
        }

        deserializer.deserialize_map(FooAnonimityVisitor(PhantomData))
    }
}

impl<A: Anonimity + Default> Default for BotnetAnonimity<A> {
    fn default() -> Self {
        Self::KAnonimity(A::default())
    }
}

impl<A: Anonimity> Anonimity for BotnetAnonimity<A> {
    fn is_anonymous(&self, x: u64) -> bool {
        match self {
            Self::KAnonimity(a) => a.is_anonymous(x),
        }
    }

    fn enabled(&self) -> bool {
        match self {
            Self::KAnonimity(a) => a.enabled(),
        }
    }

    fn class(&self) -> &str {
        match self {
            Self::KAnonimity(a) => a.class(),
        }
    }
}

/// Botnet plan configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Plan<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Entity counting configuration.
    pub entity: E,

    /// K-Anon configuration.
    pub anonimity: BotnetAnonimity<A>,

    /// Cliff detection configuration.
    pub limiter: C,
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

/// Store configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Database {
    /// Store type.
    pub db_type: DbType,

    /// Store URI.
    pub uri: Option<String>,
}

/// Botnet configuration.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotnetConfig<E, A, C>
where
    E: EntityCounter,
    A: Anonimity + Default,
    C: RateLimit,
{
    /// Botnet config version.
    pub version: Version,

    /// Botnet config plan.
    pub plan: Plan<E, A, C>,

    /// Botnet config keys.
    pub keys: Vec<Key>,

    /// Store configuration.
    pub database: Database,
}

impl<E, A, C> From<PathBuf> for BotnetConfig<E, A, C>
where
    E: EntityCounter + for<'a> Deserialize<'a>,
    A: Anonimity + for<'a> Deserialize<'a> + Default,
    C: RateLimit + for<'a> Deserialize<'a>,
{
    /// Create a new botnet configuration.
    fn from(value: PathBuf) -> Self {
        let mut file = File::open(value).expect("Unable to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Unable to read file.");

        let config: BotnetConfig<E, A, C> =
            serde_yaml::from_str(&content).expect("Bad config.");

        config
    }
}

/// Applied anonimity for privacy protection.
pub trait Anonimity {
    /// Check if the value is anonymous.
    fn is_anonymous(&self, x: u64) -> bool;

    /// Check if the anonimity is enabled.
    fn enabled(&self) -> bool;

    fn class(&self) -> &str;
}

/// Entity counting mechanism.
pub trait EntityCounter {
    /// Count this entity.
    fn count(&self) -> BotnetResult<u64>;

    /// Check if the entity counting is enabled.
    fn enabled(&self) -> bool;

    fn class(&self) -> &str;
}

/// A cliff detection mechanism to prevent abuse.
pub trait RateLimit {
    /// Register a hit.
    fn register(&self) -> BotnetResult<()>;

    /// Check if the cliff has been hit.
    fn has_hit_cliff(&self) -> BotnetResult<bool>;

    /// Check if the rate limit is enabled.
    fn enabled(&self) -> bool;

    fn class(&self) -> &str;
}
