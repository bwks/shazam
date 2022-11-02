use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::path::MAIN_SEPARATOR as PATH_SEP;
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::util::helper::load_config;

pub async fn serve(ipv4_address: String, port: u16) -> Result<()> {
    let config = load_config()?;
    let project_name = config.project;
    let output_dir = config.output_dir;

    // HTTP Server
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route(
        "/*path",
        get_service(ServeDir::new(format!(
            "{project_name}{PATH_SEP}{output_dir}{PATH_SEP}"
        )))
        .handle_error(handle_error)
        .layer(TraceLayer::new_for_http()),
    );
    let addr: SocketAddr = format!("{ipv4_address}:{port}").parse()?;

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
