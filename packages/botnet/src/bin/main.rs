use axum::{routing::get, Router};
use botnet::{core::botnet_key, prelude::*, BotnetConfig, BotnetMiddleware};
use std::{collections::HashMap, net::SocketAddr};

fn extract_ssl_param(input: &Input) -> BotnetResult<Field> {
    let key = "ssl";
    let url = Url::parse(input.as_ref())?;
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let value = params.get(key).unwrap().to_owned();
    Ok(Field::new(key, Bytes::from(value)))
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> BotnetResult<()> {
    botnet_key!(HttpProto);

    let key = HttpProto::default();

    let config = BotnetConfig {
        keys: HashMap::from([(key.type_id(), Box::new(key.clone()))]),
        metadata: Metadata::from(
            [(
                key.type_id(),
                KeyMetadata::from(
                    key.type_id(),
                    [
                        FieldMetadata::new(
                            "ssl",
                            "qs_ss",
                            values_to_bytes(vec![true, false, true]),
                            "description",
                        ),
                        FieldMetadata::new(
                            "mkt",
                            "qs_mkt",
                            values_to_bytes(vec!["1", "2", "3"]),
                            "market",
                        ),
                        FieldMetadata::new(
                            "ua",
                            "qs_ua",
                            values_to_bytes(vec!["ua1", "ua2", "ua3"]),
                            "user_agent",
                        ),
                    ]
                    .into_iter(),
                ),
            )]
            .into_iter(),
        ),
        extractors: Extractors::from(
            [(
                key.type_id(),
                KeyExtractors::from(
                    [("ssl", extract_ssl_param as ExtractorFn)].into_iter(),
                ),
            )]
            .into_iter(),
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
