mod cmd;
mod core;
mod http;
mod template;
mod util;

use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::cmd::cli;
use crate::core::build;

#[tokio::main]
async fn main() {
    let args = cli::cli();

    let config = build::init(args.init);

    // HTTP Server
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // `SpaRouter` is the easiest way to serve assets at a nested route like `/assets`
    let app = Router::new()
        .route(
            "/",
            get_service(ServeDir::new(format!(
                "./{}/{}/",
                config.project, config.output_dir
            )))
            .handle_error(handle_error),
        )
        .merge(axum_extra::routing::SpaRouter::new(
            "/blog",
            "./test/_site/blog",
        ))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
