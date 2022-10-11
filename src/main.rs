mod cmd;
mod core;
mod http;
mod model;
mod template;
mod util;

use std::process::ExitCode;

use crate::cmd::cli;

#[tokio::main]
async fn main() {
    match cli::init().await {
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(1)
        }
    };
}
