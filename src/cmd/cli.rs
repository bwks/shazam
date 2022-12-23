use std::cmp::Reverse;
use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::Result;
use tracing::{event, Level};

use crate::core::konst::TEMPLATES_DIR;
use crate::http;
use crate::model::post::{Data, Post};
use crate::template::html;
use crate::util::date_time::date_today;
use crate::util::helper::load_config;
use crate::util::text::{capitalize, parameterize};
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
    #[clap(long)]
    pub name: String,
    /// Name of the the project owner
    #[clap(long)]
    pub owner: String,
    /// Email of the project owner
    #[clap(long)]
    pub owner_email: String,
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

    /// Description
    #[clap(short, long)]
    pub description: String,

    /// Type of content <blog, about, etc..>
    #[clap(short, long)]
    pub content_type: String,
}

pub async fn init() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(init_command) => {
            app::init(
                init_command.name.to_owned(),
                init_command.owner.to_owned(),
                init_command.owner_email.to_owned(),
            )?;
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
            let config = load_config()?;
            let project_name = config.project.to_owned();
            let data_dir = config.data_dir.to_owned();
            let post_title = parameterize(generate_command.title.to_owned());
            let content_type = generate_command.content_type.to_owned();

            if config
                .content_dirs
                .iter()
                .any(|e| generate_command.content_type.eq(e))
            {
                let content_file = fs::read_to_string(format!(
                    "{project_name}{PATH_SEP}{data_dir}{PATH_SEP}{content_type}.toml"
                ))?;
                let mut data: Data = toml::from_str(content_file.as_str())?;
                let post = Post {
                    title: generate_command.title.to_owned(),
                    published_date: date_today(),
                    description: capitalize(generate_command.description.to_owned()),
                    ..Post::default()
                };
                data.posts.push(post);
                data.posts
                    .sort_by_key(|x| Reverse(x.published_date.to_owned()));

                make_file(
                    &format!("{project_name}{PATH_SEP}{data_dir}{PATH_SEP}{content_type}.toml"),
                    &toml::to_string(&data)?,
                )?;
                make_file(
                    &format!("{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{content_type}{PATH_SEP}{post_title}.jinja"),
                    &html::BLOG_POST_TEMPLATE.to_owned(),
                )?;
                Ok(())
            } else {
                event!(target: "shazam", Level::INFO, "Content type {content_type} not found!");
                Ok(())
            }
        }
    }
}
