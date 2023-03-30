use axum::{routing::get, Router};
use botnet::{
    core::config::BotnetConfig,
    prelude::*,
    service::{Botnet, BotnetMiddleware},
};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[clap(name = "botnet", about = "Botnet example.")]
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
    let opts = Args::parse();
    let config = BotnetConfig::from_path(opts.config).unwrap_or_default();

    let state = Botnet::from(config);

    let app = Router::new()
        .route("/", get(user_lib::web::root))
        .layer(BotnetMiddleware::from(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
