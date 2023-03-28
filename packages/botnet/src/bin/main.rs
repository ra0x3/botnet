use axum::{routing::get, Router};
use botnet::{macros::*, prelude::*, BotnetConfig, BotnetMiddleware};
use std::{collections::HashMap, net::SocketAddr};

fn extract_ssl_param(input: &Input) -> BotnetResult<Field> {
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
async fn run(k: K, db: Option<D>) -> BotnetResult<Option<SerdeValue>> {
    Ok(None)
}

#[evaluator(Counter)]
async fn eval(result: serde_json::Value) -> BotnetResult<Option<SerdeValue>> {
    Ok(None)
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> BotnetResult<()> {
    let key = HttpProto::builder();

    let config = BotnetConfig {
        metadata: Metadata::from(
            [(
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
            )]
            .into_iter(),
        ),
        extractors: Extractors::from(
            [("ssl", extract_ssl_param as ExtractorFn)].into_iter(),
        ),
        db: Some(InMemory::new()),
    };
    let app = Router::new()
        .route("/", get(root))
        .layer(BotnetMiddleware::from(&config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
