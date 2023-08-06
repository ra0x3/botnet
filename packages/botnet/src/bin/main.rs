use axum::{http::Method, routing::get, Router};
use botnet::{prelude::*, service::BotnetMiddleware};
use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

#[derive(Parser)]
#[clap(name = "botnet", about = "BotnetParams example.")]
struct Args {
    #[clap(short, long, help = "Path to configuration file.")]
    pub config: Option<PathBuf>,
}

pub mod user_lib {

    use std::{env, str::FromStr};
    use tracing_subscriber::filter::EnvFilter;

    const RUST_LOG: &str = "RUST_LOG";
    const HUMAN_LOGGING: &str = "HUMAN_LOGGING";

    pub mod web {
        pub async fn root() -> &'static str {
            "Hello, World!"
        }
    }

    pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
        let filter = match env::var_os(RUST_LOG) {
            Some(_) => {
                EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided")
            }
            None => EnvFilter::new("info"),
        };

        let human_logging = env::var_os(HUMAN_LOGGING)
            .map(|s| {
                bool::from_str(s.to_str().unwrap()).expect(
                    "Expected `true` or `false` to be provided for `HUMAN_LOGGING`",
                )
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
}

#[tokio::main]
async fn main() -> BotnetResult<()> {
    user_lib::init_logging()?;

    let opts = Args::parse();
    let config = BotnetConfig::from_path(opts.config).unwrap();

    let state = BotnetParams::from(config);

    tracing::info!("{state:?}");

    let app = Router::new()
        .route("/", get(user_lib::web::root))
        .layer(BotnetMiddleware::from(state))
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
                .allow_origin(Any {}),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
