mod cmd;
mod core;
mod http;
mod model;
mod template;
mod util;

use std::process::ExitCode;
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::cmd::cli;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "shazam=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    match cli::init().await {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            match e.source() {
                Some(s) => {
                    event!(target: "shazam", Level::ERROR, "{s}")
                }
                None => event!(target: "shazam", Level::ERROR, "{e}"),
            }
            ExitCode::from(1)
        }
    };
}
