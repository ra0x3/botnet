use anomaly_detector_core::{database::InMemory, prelude::*, KeyMetadata};
use anomaly_detector_macros::{extractor, task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_url_param(param: &str, input: &Input) -> AnomalyDetectorResult<Field> {
    let url = Url::parse(input.as_ref())?;
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let value = params.get(param).unwrap().to_owned();
    Ok(Field::new(param, Bytes::from(value)))
}

#[extractor(UserAgent)]
fn extract(input: &Input) -> AnomalyDetectorResult<Field> {
    extract_url_param("user_agent", input)
}

#[extractor(SSL)]
fn extract(input: &Input) -> AnomalyDetectorResult<Field> {
    extract_url_param("ssl", input)
}

#[extractor(Market)]
fn extract(input: &Input) -> AnomalyDetectorResult<Field> {
    extract_url_param("mkt", input)
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct HttpKey {
    fields: Vec<Field>,
    metadata: KeyMetadata,
    type_id: usize,
}

impl Key for HttpKey {
    type Item = Field;
    type Metadata = KeyMetadata;
    type TypeId = usize;

    fn new(name: &str) -> Self {
        Self {
            fields: Vec::new(),
            metadata: KeyMetadata::default(),
            type_id: type_id(name),
        }
    }

    fn metadata(&mut self, meta: KeyMetadata) -> &mut Self {
        self.metadata = KeyMetadata::with_key_code(meta, self.type_id);
        self
    }

    fn field(&mut self, f: Self::Item) -> &mut Self {
        self.fields.push(f);
        self
    }

    fn build(&self) -> Self {
        self.clone()
    }

    fn flatten(&self) -> Bytes {
        Bytes::from(
            self.fields
                .iter()
                .map(|f| usize::to_le_bytes(f.type_id).to_vec())
                .collect::<Vec<Vec<u8>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
        )
    }

    fn get_metadata(&self) -> Self::Metadata {
        self.metadata.clone()
    }

    fn type_id(&self) -> Self::TypeId {
        self.type_id
    }
}

#[task(Counter)]
async fn run(
    k: &'static K,
    db: Option<Self::Database>,
) -> AnomalyDetectorResult<Option<Value>> {
    Ok(None)
}

#[tokio::main]
async fn main() -> AnomalyDetectorResult<()> {
    // offline
    let key = HttpKey::new("http")
        .metadata(
            KeyMetadata::new()
                .field(FieldMeta::new(
                    "ssl",
                    "qs_ss",
                    values_to_bytes(vec![true, false, true]),
                    "description",
                ))
                .field(FieldMeta::new(
                    "mkt",
                    "qs_mkt",
                    values_to_bytes(vec!["1", "2", "3"]),
                    "market",
                ))
                .field(FieldMeta::new(
                    "ua",
                    "qs_ua",
                    values_to_bytes(vec!["ua1", "ua2", "ua3"]),
                    "user_agent",
                ))
                .build(),
        )
        .build();

    let mut db = InMemory::<HttpKey>::new();
    db.set_metadata(key.clone()).await?;
    db.set(key, Bytes::default()).await?;

    // online
    let input = Input::new(
        "http://google.com?foo=true&ssl=zoo&z=shoo&shoo=baz&baz=123&user_agent=ua1&mkt=US",
    );
    let _key = HttpKey::new("http")
        .field(UserAgentExtractor::extract(&input)?)
        .field(SSLExtractor::extract(&input)?)
        .field(MarketExtractor::extract(&input)?);

    Ok(())
}
