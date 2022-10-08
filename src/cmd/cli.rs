use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of project
    #[arg(short, long, value_name = "name")]
    pub init: String,
}

pub fn cli() -> Cli {
    let cli = Cli::parse();
    return cli;
}
