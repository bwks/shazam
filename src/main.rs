mod template;
mod util;

use clap::Parser;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;

use crate::template::html;

#[derive(Serialize, Deserialize)]
struct Config {
    project: String,
    css: String,
    code_block: String,
    config_dir: String,
    data_dir: String,
    asset_dirs: Vec<String>,
    template_dirs: Vec<String>,
    content_dirs: Vec<String>,
}

struct _TailwindConfig {}

impl Config {
    fn default() -> Self {
        Self {
            project: "".to_owned(),
            css: "tailwind".to_owned(),
            code_block: "highlightjs".to_owned(),
            config_dir: "config".to_owned(),
            data_dir: "data".to_owned(),
            asset_dirs: string_vec!["css", "js", "font", "img", "favicon"],
            template_dirs: string_vec!["layouts", "includes"],
            content_dirs: string_vec!["blog"],
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of project
    #[arg(short, long, value_name = "name")]
    init: String,
}

fn create_dirs(parent_path: String, directories: Vec<String>) {
    for d in directories {
        fs::create_dir_all(format!("{}/{}", parent_path, d)).unwrap();
        fs::File::create(format!("{}/{}/.gitkeep", parent_path, d)).unwrap();
    }
}
fn main() {
    // base template
    let mut env = Environment::new();
    env.add_template("index", html::BASE).unwrap();
    let tmpl = env.get_template("index").unwrap();
    let _result = tmpl.render(context!(project => "SSG")).unwrap();

    let mut cfg = Config::default();

    let args = Args::parse();

    println!("Project Name: {}", args.init);

    cfg.project = args.init.to_owned();

    // Directories
    create_dirs(cfg.project.to_owned(), vec![cfg.config_dir.to_owned()]);
    create_dirs(cfg.project.to_owned(), vec![cfg.data_dir.to_owned()]);
    create_dirs(
        format!("{}/{}", cfg.project.to_owned(), "assets"),
        cfg.asset_dirs.to_owned(),
    );
    create_dirs(
        format!("{}/{}", cfg.project.to_owned(), "templates"),
        cfg.template_dirs.to_owned(),
    );
    create_dirs(cfg.project.to_owned(), cfg.content_dirs.to_owned());

    // Files
    std::fs::write("config.json", serde_json::to_string_pretty(&cfg).unwrap()).unwrap();

    let mut data_file =
        fs::File::create(format!("{}/{}/data.json", cfg.project, cfg.data_dir)).unwrap();
    data_file.write_all(b"{}").unwrap();

    let mut css_file = fs::File::create(format!(
        "{}/{}/tailwind.config.js",
        cfg.project, cfg.config_dir
    ))
    .unwrap();
    css_file.write_all(b"{}").unwrap();

    let mut base_html =
        fs::File::create(format!("{}/templates/layouts/base.html.jinja", cfg.project)).unwrap();
    base_html.write_all(html::BASE.as_bytes()).unwrap();
}
