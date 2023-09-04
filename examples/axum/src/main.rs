/// axum
///
/// A simple example of how `Botnet` can be plugged into an axum web server middleware.
use axum::{http::Method, routing::get, Router};
use botnet::prelude::*;
use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::filter::EnvFilter;

pub async fn root() -> &'static str {
    "Hello, World!"
}

#[extractor(Path)]
async fn extractor(input: &input::Input) -> BotnetResult<field::ExtractedField> {
    // FIXME: extract actual path from url
    Ok(field::ExtractedField::new("test", Bytes::from("test")))
}

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter = match env::var_os("RUST_LOG") {
        Some(_) => {
            EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided")
        }
        None => EnvFilter::new("info"),
    };

    let human_logging = env::var_os("HUMAN_LOGGING")
        .map(|s| {
            bool::from_str(s.to_str().unwrap())
                .expect("Expected `true` or `false` to be provided for `HUMAN_LOGGING`")
        })
        .unwrap_or(true);

    let sub = tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_env_filter(filter);

    if human_logging {
        sub.with_ansi(true)
            .with_level(true)
            .with_line_number(true)
            .init();
    } else {
        sub.with_ansi(false)
            .with_level(true)
            .with_line_number(true)
            .json()
            .init();
    }
    Ok(())
}

#[botnet::main(config = "config.yaml")]
async fn main() -> BotnetResult<()> {
    init_logging()?;

    tracing::info!("{context:?}");

    let app = Router::new()
        .route("/", get(root))
        .layer(BotnetMiddleware::from(Arc::new(context)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_origin(Any {})
                .allow_headers(Any {}),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
