mod cmd;
mod core;
mod http;
mod model;
mod template;
mod util;

use crate::cmd::cli;

#[tokio::main]
async fn main() {
    cli::init().await;
}
