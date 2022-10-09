use crate::core::build;
use crate::http;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a project
    Init(InitCommand),

    /// Start the development web server
    Serve(ServeCommand),
}

#[derive(Args, Debug)]
pub struct InitCommand {
    /// Name of the project
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ServeCommand {
    /// IPv4 Address of the development server
    #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
    pub ipv4_address: String,

    /// Port of the development server
    #[clap(short, long, default_value_t = 3000)]
    pub port: u16,
}

pub async fn init() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(init_command) => {
            build::init(init_command.name.to_owned());
        }
        Commands::Serve(serve_command) => {
            http::server::serve(serve_command.ipv4_address.to_owned(), serve_command.port).await;
        }
    }
}
