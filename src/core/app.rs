use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;
use tracing::event;

use anyhow::{bail, Result};
use tera::Context;
use toml;
use tracing::Level;

use crate::core::konst::{
    ASSETS_DIR, BLOG_DATA_FILE, BLOG_DIR, CONFIG_DIR, CONFIG_FILE, CSS_DIR, DATA_DIR,
    HTML_INDEX_FILE, INCLUDES_DIR, LAYOUTS_DIR, MACROS_DIR, OUTPUT_DIR, PROC_FILE, PROC_FILE_DEV,
    RSS_FEED_FILE, TAILWIND_CONFIG_FILE, TAILWIND_INPUT_FILE, TEMPLATES_DIR,
    TEMPLATE_HASHES_FILENAME,
};
use crate::model::config::Config;
use crate::model::post::{Data, FileType, Post, Posts};
use crate::string_vec;
use crate::template::proc;
use crate::template::tailwind;
use crate::template::{html, rss};
use crate::util::date_time::date_today;
use crate::util::file_sys::{copy_recursively, current_dir, make_dirs, make_file};
use crate::util::helper::load_config;
use crate::util::template::{init_env, render_template, template_hasher};
use crate::util::text::parameterize;

/// Initial site directories and files
#[tracing::instrument]
pub fn init(project: String, owner: String, owner_email: String) -> Result<Config> {
    event!(target: "shazam", Level::INFO, "Project: `{project}` => initialzing ...");
    let current_dir = current_dir()?;
    let config = Config::init(project, owner, owner_email);
    let project_name = config.project.to_owned();
    let asset_dirs = config.asset_dirs.to_owned();
    let template_dirs = config.template_dirs.to_owned();
    let content_dirs = config.content_dirs.to_owned();
    let jinja_file = FileType::Jinja;

    // Template environment
    let mut env = init_env(&current_dir, &project_name)?;

    // Build project
    // Directories
    make_dirs(&format!(".{PATH_SEP}"), vec![CONFIG_DIR.to_owned()])?;
    make_dirs(
        &project_name,
        vec![DATA_DIR.to_owned(), OUTPUT_DIR.to_owned()],
    )?;
    make_dirs(
        &format!("{project_name}{PATH_SEP}{ASSETS_DIR}"),
        asset_dirs.to_owned(),
    )?;
    make_dirs(
        &format!("{project_name}{PATH_SEP}{TEMPLATES_DIR}"),
        template_dirs,
    )?;
    make_dirs(
        &format!("{project_name}{PATH_SEP}{TEMPLATES_DIR}"),
        content_dirs.to_owned(),
    )?;
    make_dirs(
        &format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"),
        content_dirs,
    )?;
    make_dirs(&format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"), asset_dirs)?;

    let blog_post = Post {
        author: config.owner.to_owned(),
        author_email: config.owner_email.to_owned(),
        title: "test blog".to_owned(),
        description: "This is a sample blog post".to_owned(),
        publish: true,
        published_date: date_today(),
        tags: string_vec!["stuff", "things"],
        ..Post::default()
    };
    let rss_feed = Post {
        author: config.owner.to_owned(),
        author_email: config.owner_email.to_owned(),
        title: "feed".to_owned(),
        description: format!("{project_name} RSS Feed"),
        published_date: date_today(),
        file_type: FileType::Xml,
        ..Post::default()
    };

    let data = Data {
        posts: vec![blog_post, rss_feed],
    };

    // Files
    // Config files
    make_file(
        &format!("{CONFIG_DIR}{PATH_SEP}{CONFIG_FILE}"),
        &toml::to_string(&config)?,
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{DATA_DIR}{PATH_SEP}{BLOG_DATA_FILE}"),
        &toml::to_string(&data)?,
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{ASSETS_DIR}{PATH_SEP}{CSS_DIR}{PATH_SEP}{TAILWIND_INPUT_FILE}"),
        &tailwind::CSS.to_owned(),
    )?;
    // Templsate files
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{LAYOUTS_DIR}{PATH_SEP}site.{jinja_file}"
        ),
        &html::SITE_LAYOUT.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{LAYOUTS_DIR}{PATH_SEP}blog.{jinja_file}"
        ),
        &html::BLOG_LAYOUT.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{INCLUDES_DIR}{PATH_SEP}footer.{jinja_file}"
        ),
        &html::FOOTER_INCLUDE.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{INCLUDES_DIR}{PATH_SEP}lorem-ipsum.{jinja_file}"
        ),
        &html::LOREM_IPSUM_INCLUDE.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}page-header.{jinja_file}"
        ),
        &html::PAGE_HEADER_MACRO.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}link-to.{jinja_file}"
        ),
        &html::LINK_TO_MACRO.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}tags.{jinja_file}"
        ),
        &html::TAGS_MACRO.to_owned(),
    )?;

    // Site files
    make_file(
        &format!("{project_name}{PATH_SEP}index.{jinja_file}"),
        &html::SITE_INDEX_TEMPLATE.to_owned(),
    )?;
    // blog index file
    make_file(
        &format!("{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{BLOG_DIR}{PATH_SEP}index.{jinja_file}"),
        &html::BLOG_INDEX_TEMPLATE.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{BLOG_DIR}{PATH_SEP}test-blog.{jinja_file}"
        ),
        &html::BLOG_POST_TEMPLATE.to_owned(),
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{BLOG_DIR}{PATH_SEP}feed.{jinja_file}"),
        &rss::RSS_FEED_TEMPLATE.to_owned(),
    )?;

    // Render Files
    // Tailwind config file
    env.add_raw_template(TAILWIND_CONFIG_FILE, tailwind::CONFIG)?;
    let mut tailwind_ctx = Context::new();
    tailwind_ctx.insert("project", &project_name);
    tailwind_ctx.insert("path_sep", &PATH_SEP);
    tailwind_ctx.insert("output_dir", OUTPUT_DIR);
    let tailwind_tmpl = render_template(&env, TAILWIND_CONFIG_FILE, &tailwind_ctx)?;
    make_file(
        &format!("{CONFIG_DIR}{PATH_SEP}{TAILWIND_CONFIG_FILE}"),
        &tailwind_tmpl,
    )?;

    // Procfile
    env.add_raw_template(PROC_FILE, proc::PROCFILE)?;
    let mut procfile_ctx = Context::new();
    procfile_ctx.insert("project", &project_name);
    procfile_ctx.insert("path_sep", &PATH_SEP);
    procfile_ctx.insert("output_dir", OUTPUT_DIR);
    procfile_ctx.insert("config_dir", CONFIG_DIR);
    procfile_ctx.insert("tailwind_config_file", TAILWIND_CONFIG_FILE);
    let procfile_tmpl = render_template(&env, PROC_FILE, &procfile_ctx)?;
    make_file(&PROC_FILE.to_owned(), &procfile_tmpl)?;

    // Procfile.dev
    env.add_raw_template(PROC_FILE_DEV, proc::PROCFILE_DEV)?;
    let mut procfile_dev_ctx = Context::new();
    procfile_dev_ctx.insert("project", &project_name);
    procfile_dev_ctx.insert("path_sep", &PATH_SEP);
    procfile_dev_ctx.insert("output_dir", OUTPUT_DIR);
    procfile_dev_ctx.insert("config_dir", CONFIG_DIR);
    procfile_dev_ctx.insert("tailwind_config_file", TAILWIND_CONFIG_FILE);
    let procfile_dev_tmpl = render_template(&env, PROC_FILE_DEV, &procfile_dev_ctx)?;
    make_file(&PROC_FILE_DEV.to_owned(), &procfile_dev_tmpl)?;

    event!(target: "shazam", Level::INFO, "Project: `{project_name}` => initialization complete");
    build()?;
    Ok(config)
}

/// Build site
#[tracing::instrument]
pub fn build() -> Result<()> {
    let current_dir = current_dir()?;
    let config = load_config()?;
    let project_name = config.project.to_owned();
    let output_dir = config.output_dir.to_owned();
    let content_dirs = config.content_dirs.to_owned();
    let data_dir = config.data_dir.to_owned();
    let mut posts = Posts::init(&config)?;
    let jinja_file = FileType::Jinja;

    event!(target: "shazam", Level::INFO, "Project: `{project_name}` => building ...");

    // Template environment
    let mut env = init_env(&current_dir, &project_name)?;

    let mut template_hashes: HashMap<String, String> =
        match fs::read_to_string(TEMPLATE_HASHES_FILENAME) {
            Ok(s) => serde_json::from_str(&s)?,
            Err(_) => HashMap::new(),
        };

    // Project index template file
    let index_tmpl_name = format!("{project_name}{PATH_SEP}index.{jinja_file}");

    env.add_template_file(&index_tmpl_name, None)?;
    let mut index_ctx = Context::new();
    index_ctx.insert("config", &config);
    index_ctx.insert("posts", &posts);

    let tmpl = render_template(&env, &index_tmpl_name, &index_ctx)?;

    // TODO: refactor to function
    template_hashes
        .entry(index_tmpl_name.to_owned())
        .or_insert_with(|| "".to_owned());

    let current_hash = template_hashes[&index_tmpl_name].to_owned();
    let this_hash = template_hasher(&tmpl);

    if current_hash != this_hash {
        event!(target: "shazam", Level::INFO, "File: `{index_tmpl_name}` has changed, rebuilding...");
        template_hashes.insert(index_tmpl_name, this_hash);
        make_file(
            &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{HTML_INDEX_FILE}"),
            &tmpl,
        )?;
    }
    //

    for dir in content_dirs {
        let mut dir_posts: Vec<Post> = match posts.by_content.get(&dir) {
            Some(posts) => posts.to_owned(),
            None => vec![],
        };
        for mut post in dir_posts.iter_mut() {
            if post.author.is_empty() {
                post.author = config.owner.to_owned()
            }
            if post.author_email.is_empty() {
                post.author_email = config.owner_email.to_owned()
            }
        }
        let mut clean_posts: HashMap<String, Vec<Post>> = HashMap::new();

        clean_posts.insert("posts".to_owned(), dir_posts.to_owned());

        make_file(
            &format!("{project_name}{PATH_SEP}{data_dir}{PATH_SEP}{dir}.toml"),
            &toml::to_string(&clean_posts)?,
        )?;

        let mut dir_ctx = Context::new();
        dir_ctx.insert("config", &config);
        dir_ctx.insert("posts", &posts);

        let dir_tmpl_name = format!("{dir}{PATH_SEP}index.{jinja_file}");
        let dir_tmpl = render_template(&env, &dir_tmpl_name, &dir_ctx)?;

        // TODO: refactor to function
        template_hashes
            .entry(dir_tmpl_name.to_owned())
            .or_insert_with(|| "".to_owned());

        let current_hash = template_hashes[&dir_tmpl_name].to_owned();
        let this_hash = template_hasher(&dir_tmpl);

        if current_hash != this_hash {
            event!(target: "shazam", Level::INFO, "File: `{dir_tmpl_name}` has changed, rebuilding...");
            template_hashes.insert(dir_tmpl_name, this_hash);
            make_file(
                &format!(
                    "{project_name}{PATH_SEP}{OUTPUT_DIR}{PATH_SEP}{dir}{PATH_SEP}{HTML_INDEX_FILE}"
                ),
                &dir_tmpl,
            )?;
        }
        //

        // Get the rendered body content from posts
        let re = Regex::new(r"^[\s\S]*<body[^>]*>([\s\S]*)</body>[\s\S]*$")?;
        let mut body_content: HashMap<String, String> = HashMap::new();
        for post in &dir_posts {
            let post_title = parameterize(post.title.to_owned());
            let file_name = format!("{post_title}.{jinja_file}");

            let mut post_ctx = Context::new();
            post_ctx.insert("config", &config);
            post_ctx.insert("post", &post);
            post_ctx.insert("posts", &posts);
            let tmpl = render_template(&env, &format!("{dir}{PATH_SEP}{file_name}"), &post_ctx)?;

            let content = match re.captures(&tmpl) {
                Some(s) => s[1].to_string(),
                None => "".to_string(),
            };
            body_content.insert(post_title.to_owned(), content.to_owned());
        }
        posts.content = body_content;

        for post in &dir_posts {
            let post_title = parameterize(post.title.to_owned());
            let file_name = format!("{post_title}.{jinja_file}");
            let (file_type, file_path) = match post.file_type {
                FileType::Html => (
                    HTML_INDEX_FILE,
                    format!(
                        "{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}{PATH_SEP}{post_title}"
                    ),
                ),
                FileType::Xml => (
                    RSS_FEED_FILE,
                    format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}"),
                ),
                _ => bail!("unsupported file type"),
            };
            // Only make directories for HTML files.
            if post.file_type == FileType::Html {
                make_dirs(
                    &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}"),
                    vec![post_title.to_owned()],
                )?;
            }

            let mut post_ctx = Context::new();
            post_ctx.insert("config", &config);
            post_ctx.insert("post", &post);
            post_ctx.insert("posts", &posts);
            let template_name = format!("{dir}{PATH_SEP}{file_name}");
            let post_tmpl = render_template(&env, &template_name, &post_ctx)?;

            template_hashes
                .entry(template_name.to_owned())
                .or_insert_with(|| "".to_owned());

            let current_hash = template_hashes[&template_name].to_owned();
            let this_hash = template_hasher(&post_tmpl);

            if current_hash != this_hash {
                event!(target: "shazam", Level::INFO, "File: `{template_name}` has changed, rebuilding...");
                template_hashes.insert(template_name, this_hash);
                make_file(&format!("{file_path}{PATH_SEP}{file_type}"), &post_tmpl)?;
            }
        }
    }

    // Write tempalate file hashses to file for future reading
    make_file(
        &TEMPLATE_HASHES_FILENAME.to_owned(),
        &serde_json::to_string(&template_hashes)?,
    )?;

    // Move assets
    copy_recursively(
        format!("{project_name}{PATH_SEP}{ASSETS_DIR}"),
        format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"),
    )?;

    event!(target: "shazam", Level::INFO, "Project: `{project_name}` => build complete");
    Ok(())
}
