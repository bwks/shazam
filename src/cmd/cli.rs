use std::fs;

use anyhow::Result;

use crate::http;
use crate::model::config::Config;
use crate::model::post::Post;
use crate::template::html;
use crate::util::text::dasherize;
use crate::{core::app, util::file_sys::make_file};

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
    Init(InitCmd),

    /// Start the development web server
    Serve(ServeCmd),

    /// (Re)Build the site
    Build,

    /// Generate a post
    Generate(GenerateCmd),
}

#[derive(Args, Debug)]
pub struct InitCmd {
    /// Name of the project
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ServeCmd {
    /// IPv4 Address of the development server
    #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
    pub ipv4_address: String,

    /// Port of the development server
    #[clap(short, long, default_value_t = 3000)]
    pub port: u16,
}

#[derive(Args, Debug)]
pub struct GenerateCmd {
    /// Title of post
    #[clap(short, long)]
    pub title: String,

    /// Content Type
    #[clap(short, long)]
    pub content: String,
}

pub async fn init() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(init_command) => {
            app::init(init_command.name.to_owned())?;
            Ok(())
        }
        Commands::Serve(serve_command) => {
            http::server::serve(serve_command.ipv4_address.to_owned(), serve_command.port).await?;
            Ok(())
        }
        Commands::Build => {
            app::build()?;
            Ok(())
        }
        Commands::Generate(generate_command) => {
            let config_file = fs::read_to_string("config.json")?;
            let config: Config = serde_json::from_str(config_file.as_str())?;
            let project_name = config.project.to_owned();
            let data_dir = config.data_dir.to_ascii_lowercase();
            let post_title = dasherize(generate_command.title.to_owned());
            let post_type = generate_command.content.to_owned();

            if config
                .content_dirs
                .iter()
                .any(|e| generate_command.content.eq(e))
            {
                // Update content{
                let content_file =
                    fs::read_to_string(format!("{project_name}/{data_dir}/{post_type}.json"))?;
                let mut content: Vec<Post> = serde_json::from_str(content_file.as_str())?;
                let mut post = Post::default();
                post.title = generate_command.title.to_owned();
                content.push(post);

                make_file(
                    &format!("{project_name}/{data_dir}/{post_type}.json"),
                    &serde_json::to_string_pretty(&content)?,
                )?;
                make_file(
                    &format!("{project_name}/{post_type}/{post_title}.jinja"),
                    &html::BLOG_POST.to_owned(),
                )?;
                Ok(())
                // Create file
            } else {
                println!("Sorry, content not found!");
                Ok(())
            }
        }
    }
}
