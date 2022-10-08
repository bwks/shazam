use minijinja::{context, Environment, Source};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs;
use std::io::prelude::*;

use crate::string_vec;
use crate::template::html;
use crate::template::tailwind;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: String,
    pub css: String,
    pub code_block: String,
    pub config_dir: String,
    pub data_dir: String,
    pub output_dir: String,
    pub asset_dirs: Vec<String>,
    pub template_dirs: Vec<String>,
    pub content_dirs: Vec<String>,
}
impl Config {
    pub fn default() -> Self {
        Self {
            project: "".to_owned(),
            css: "tailwind".to_owned(),
            code_block: "highlightjs".to_owned(),
            config_dir: "config".to_owned(),
            data_dir: "data".to_owned(),
            output_dir: "_site".to_owned(),
            asset_dirs: string_vec!["css", "js", "font", "img", "favicon", "error"],
            template_dirs: string_vec!["layouts", "includes"],
            content_dirs: string_vec!["blog"],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TailwindConfig {
    pub content: Vec<String>,
}
impl TailwindConfig {
    pub fn _default() -> Self {
        Self {
            content: string_vec!["./src/**/*.{html,js}"],
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Post {
    pub published_date: String,
    pub updated_date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub bibliography: Vec<String>,
    pub toc_items: Vec<String>,
    pub links: Vec<String>,
}

fn make_dirs(parent_path: String, directories: Vec<String>) {
    for d in directories {
        fs::create_dir_all(format!("{}/{}", parent_path, d)).unwrap();
        fs::File::create(format!("{}/{}/.gitkeep", parent_path, d)).unwrap();
    }
}

fn make_file(path: String, content: String) {
    fs::write(path, content).unwrap()
}

pub fn init(name: String) -> Config {
    println!("Project `{}` initialzing...", name);
    let mut config = Config::default();
    config.project = name;

    // Directories
    let directories = vec![
        (
            // Config dir
            config.project.to_owned(),
            vec![config.config_dir.to_owned()],
        ),
        (
            // Data dir
            config.project.to_owned(),
            vec![config.data_dir.to_owned()],
        ),
        (
            // Output dir
            config.project.to_owned(),
            vec![config.output_dir.to_owned()],
        ),
        (
            // Asset dirs
            format!("{}/{}", config.project.to_owned(), "assets"),
            config.asset_dirs.to_owned(),
        ),
        (
            // Template dirs
            format!("{}/{}", config.project.to_owned(), "templates"),
            config.template_dirs.to_owned(),
        ),
        (
            // Content dirs
            config.project.to_owned(),
            config.content_dirs.to_owned(),
        ),
    ];
    for dir in directories {
        make_dirs(dir.0, dir.1)
    }

    // Files
    let files = vec![
        (
            // Config file
            "config.json".to_owned(),
            serde_json::to_string_pretty(&config).unwrap(),
        ),
        (
            // Data file
            format!("{}/{}/data.json", config.project, config.data_dir),
            serde_json::to_string_pretty(&vec![Post::default()]).unwrap(),
        ),
        (
            // Tailwind config file
            format!(
                "{}/{}/tailwind.config.js",
                config.project, config.config_dir
            ),
            tailwind::CONFIG.to_owned(),
        ),
        (
            // Base layout file
            format!("{}/templates/layouts/base.jinja", config.project),
            html::BASE.to_owned(),
        ),
        (
            // Blog layout file
            format!("{}/templates/layouts/blog.jinja", config.project),
            html::BLOG.to_owned(),
        ),
        (
            // Footer include file
            format!("{}/templates/includes/_footer.jinja", config.project),
            html::FOOTER.to_owned(),
        ),
    ];
    for file in files {
        make_file(file.0, file.1)
    }

    // Load templaes
    let mut all_templates: Vec<String> = vec![];
    for dir in &config.template_dirs {
        for entry in fs::read_dir(format!("{}/templates/{}/", config.project, dir)).unwrap() {
            let file = entry.unwrap().file_name().into_string().unwrap();
            if file.ends_with(".jinja") || file.ends_with(".j2") {
                all_templates.push(format!("{}/{}", dir, file))
            }
        }
    }

    // println!("{:#?}", all_templates);

    // Load includes

    // Build base site
    // Base template
    let mut env = Environment::new();
    let mut source = Source::new();

    for template in all_templates {
        let template_string =
            fs::read_to_string(format!("{}/templates/{}", config.project, template)).unwrap();
        source.add_template(template, template_string).unwrap();
        env.set_source(source.to_owned());
    }

    let tmpl = env.get_template("layouts/base.jinja").unwrap();
    let base_tmpl = tmpl.render(context!(project => config.project)).unwrap();

    let mut index_file = fs::File::create(format!(
        "{}/{}/index.html",
        config.project, config.output_dir
    ))
    .unwrap();
    index_file.write_all(base_tmpl.as_bytes()).unwrap();

    //
    make_dirs(
        format!("{}/{}", config.project, config.output_dir),
        vec!["blog".to_owned()],
    );
    let tmpl = env.get_template("layouts/blog.jinja").unwrap();
    let blog_tmpl = tmpl.render(context!(project => config.project)).unwrap();

    let mut blog_file = fs::File::create(format!(
        "{}/{}/blog/index.html",
        config.project, config.output_dir
    ))
    .unwrap();
    blog_file.write_all(blog_tmpl.as_bytes()).unwrap();

    return config;
}
