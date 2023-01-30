use anomaly_detector_core::{database::InMemory, prelude::*, KeyMetadata};
use anomaly_detector_macros::{extractor, key, task};
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


#[key]
struct HttpKey {
    fields: Vec<Field>,
    metadata: KeyMetadata,
    type_id: usize,
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
