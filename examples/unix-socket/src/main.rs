use botnet::prelude::*;
use std::{env, io, path::PathBuf, str::FromStr};
use tokio::{io::Interest, net::UnixListener};
use tracing_subscriber::filter::EnvFilter;

const RUST_LOG: &str = "RUST_LOG";
const HUMAN_LOGGING: &str = "HUMAN_LOGGING";

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter = match env::var_os(RUST_LOG) {
        Some(_) => {
            EnvFilter::try_from_default_env().expect("Invalid `RUST_LOG` provided")
        }
        None => EnvFilter::new("info"),
    };

    let human_logging = env::var_os(HUMAN_LOGGING)
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    let context = Arc::new(context);
    let listener = UnixListener::bind("/tmp/botnet.sock").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let ready = stream
                    .ready(Interest::READABLE | Interest::WRITABLE)
                    .await?;
                if ready.is_readable() {
                    let mut data = vec![0; 1024];
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_read(&mut data) {
                        Ok(n) => {
                            tracing::debug!("read {} bytes", n);
                            let _ = botnet(context.clone(), data).await?;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }

                if ready.is_writable() {
                    // Try to write data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_write(b"hello world") {
                        Ok(n) => {
                            tracing::debug!("write {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("accept failed: {e:?}");
            }
        }
    }
}
