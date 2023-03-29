use axum::{routing::get, Router};
use botnet::{
    core::{botnet_key, config::Config, AsBytes, Bytes},
    prelude::*,
    BotnetMiddleware, BotnetStateConfig,
};
use clap::Parser;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[clap(short, long, help = "Path to configuration file.")]
    pub config: Option<PathBuf>,
}

pub mod user_lib {

    pub mod extractors {

        use botnet::core::{BotnetResult, Bytes, Field, Input, Url};
        use std::collections::HashMap;

        pub fn extract_ssl_param(input: &Input) -> BotnetResult<Field> {
            let key = "ssl";
            let url = Url::parse(input.as_ref())?;
            let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
            let value = params.get(key).unwrap().to_owned();
            Ok(Field::new(key, Bytes::from(value)))
        }
    }

    pub mod web {
        pub async fn root() -> &'static str {
            "Hello, World!"
        }
    }
}

#[tokio::main]
async fn main() -> BotnetResult<()> {
    botnet_key!(HttpProto);

    let _config = Config::default();

    let key = HttpProto::default();

    let config = BotnetStateConfig {
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
                            utils::values_to_bytes(vec![true, false, true]),
                            "description",
                        ),
                        FieldMetadata::new(
                            "mkt",
                            "qs_mkt",
                            utils::values_to_bytes(vec!["1", "2", "3"]),
                            "market",
                        ),
                        FieldMetadata::new(
                            "ua",
                            "qs_ua",
                            utils::values_to_bytes(vec!["ua1", "ua2", "ua3"]),
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
                    [(
                        "ssl",
                        user_lib::extractors::extract_ssl_param as ExtractorFn,
                    )]
                    .into_iter(),
                ),
            )]
            .into_iter(),
        ),
        db: Some(InMemory::new()),
    };

    let app = Router::new()
        .route("/", get(user_lib::web::root))
        .layer(BotnetMiddleware::from(&config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
