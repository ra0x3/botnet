use async_std::sync::{Arc, Mutex};
use axum::{
    extract::{Extension, Json},
    routing::post,
    Router,
};
use bitsy::{
    bitsy_diesel::{
        prelude::*,
    },
    database::{Database},
    models::{Account},
    tables::{accounts::dsl::*},
};
use clap::Parser;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;

#[derive(Debug, Parser)]
#[clap(name = "Bitsy API Service", about = "bitsy <3")]
pub struct Args {
    #[clap(short, long, help = "Bitsy Server config.")]
    config: Option<PathBuf>,
    #[clap(long, help = "Webserver IP", default_value = "127.0.0.1")]
    host: String,
    #[clap(long, help = "Webserver port", default_value = "8080")]
    port: String,
    #[clap(long, help = "Posgres host", default_value = "127.0.0.1")]
    pg_host: String,
    #[clap(long, help = "Posgres port", default_value = "5432")]
    pg_port: String,
    #[clap(long, help = "Posgres user", default_value = "postgres")]
    pg_user: String,
    #[clap(long, help = "Posgres password", default_value = "")]
    pg_password: String,
    #[clap(long, help = "Posgres database", default_value = "bitsy")]
    pg_database: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct AccessRequest {
    pub cid: String,
    pub address: String,
    pub token: String,
}

#[axum_macros::debug_handler]
async fn third_party_access_request(
    Json(data): Json<AccessRequest>,
    Extension(db): Extension<Arc<Mutex<Database>>>,
) -> Json<Value> {
    let db = db.lock().await;
    let account = accounts
        .filter(address.eq(&data.address))
        .first::<Account>(&*db.conn)
        .expect("Failed to find account");

    Json(json!({"data": serde_json::to_value(&account).unwrap() }))
}

#[tokio::main]
async fn main() {
    let filter = match std::env::var_os("RUST_LOG") {
        Some(_) => EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided"),
        None => EnvFilter::new("info"),
    };

    tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_env_filter(filter)
        .init();

    let opt = Args::from_args();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        opt.pg_user, opt.pg_password, opt.pg_host, opt.pg_port, opt.pg_database
    );
    let db = Arc::new(Mutex::new(Database::new(&db_url).unwrap()));

    let hostname = format!("{}:{}", opt.host, opt.port);

    info!("Starting server at {}", &hostname);

    let app = Router::new()
        .route("/access", post(third_party_access_request))
        .layer(Extension(db.clone()));

    axum::Server::bind(&hostname.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Service failed to start");
}
