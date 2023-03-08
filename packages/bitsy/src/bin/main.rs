use bitsy_core::{database::InMemory, prelude::*, KeyMetadata};
use bitsy_macros::{key, task};
use std::collections::HashMap;

fn extract_ssl_param(input: &Input) -> BitsyResult<Field> {
    let key = "ssl";
    let url = Url::parse(input.as_ref())?;
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let value = params.get(key).unwrap().to_owned();
    Ok(Field::new(key, Bytes::from(value)))
}

#[key]
struct HttpProto {
    fields: Vec<Field>,
    metadata: KeyMetadata,
}

#[task(Counter)]
async fn run(k: K, db: Option<D>) -> BitsyResult<Option<Value>> {
    Ok(None)
}

#[tokio::main]
async fn main() -> BitsyResult<()> {
    // setup
    let key = HttpProto::builder();

    // extractors are separate from key
    let mut extractors = Extractors::new();
    extractors.add("ssl", extract_ssl_param);

    // metadata is separate from key
    let mut metadata = Metadata::new();
    metadata.insert(
        key.type_id(),
        KeyMetadata::new()
            .field(FieldMetadata::new(
                "ssl",
                "qs_ss",
                values_to_bytes(vec![true, false, true]),
                "description",
            ))
            .field(FieldMetadata::new(
                "mkt",
                "qs_mkt",
                values_to_bytes(vec!["1", "2", "3"]),
                "market",
            ))
            .field(FieldMetadata::new(
                "ua",
                "qs_ua",
                values_to_bytes(vec!["ua1", "ua2", "ua3"]),
                "user_agent",
            ))
            .build(),
    );

    let mut db = InMemory::new();

    // online
    let input: Input = "http://google.com?foo=true&ssl=zoo&z=shoo&shoo=baz&baz=123&user_agent=ua1&mkt=US".into();
    let key = HttpProto::from_input(input, &extractors, &metadata)?;

    db.set_key(key.clone(), Bytes::new()).await?;

    // run tasks
    let _ = CounterTask::run(key, Some(db)).await?;

    Ok(())
}
