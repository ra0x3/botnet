use bitsy_core::{database::InMemory, prelude::*, KeyMetadata};
use bitsy_macros::{key, task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_url_param(input: &Input) -> BitsyResult<Field> {
    let url = Url::parse(input.as_ref())?;
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let value = params.get("ssl").unwrap().to_owned();
    Ok(Field::new("http", "ssl", Bytes::from(value)))
}

#[key]
struct HttpKey {
    fields: Vec<Field>,
    metadata: KeyMetadata,
    type_id: usize,
}

#[task(Counter)]
async fn run(k: &'static K, db: Option<Self::Database>) -> BitsyResult<Option<Value>> {
    Ok(None)
}

#[tokio::main]
async fn main() -> BitsyResult<()> {
    let mut extractors = Extractors::new();
    extractors.add("ssl", extract_url_param);
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

    let key2 = key.clone();
    let flat = key2.flatten();

    let mut db = InMemory::new();
    db.set_key(key, Bytes::new()).await?;
    db.set_bytes(flat, Bytes::new()).await?;
    // db.set_metadata(key.clone()).await?;

    // online
    let input: Input = "http://google.com?foo=true&ssl=zoo&z=shoo&shoo=baz&baz=123&user_agent=ua1&mkt=US".into();
    let foo = HttpKey::from_input("http", input, extractors)?;
    // let _key = HttpKey::new("http")
    //     .field(UserAgentExtractor::extract(&input)?)
    //     .field(SSLExtractor::extract(&input)?)
    //     .field(MarketExtractor::extract(&input)?);

    Ok(())
}