use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::Result;
use tera::Context;

use crate::core::konst::{
    ASSETS_DIR, BLOG_DATA_FILE, BLOG_DIR, CONFIG_DIR, CONFIG_FILE, CSS_DIR, DATA_DIR,
    HTML_INDEX_FILE, INCLUDES_DIR, LAYOUTS_DIR, MACROS_DIR, OUTPUT_DIR, PROC_FILE, PROC_FILE_DEV,
    TAILWIND_CONFIG_FILE, TAILWIND_INPUT_FILE, TEMPLATES_DIR,
};
use crate::model::config::Config;
use crate::model::post::{Post, Posts};
use crate::string_vec;
use crate::template::html;
use crate::template::proc;
use crate::template::tailwind;
use crate::util::date_time::date_today;
use crate::util::file_sys::{copy_recursively, current_dir, make_dirs, make_file};
use crate::util::helper::load_config;
use crate::util::template::{init_env, render_template};
use crate::util::text::parameterize;

/// Initial site directories and files
pub fn init(project_name: String) -> Result<Config> {
    println!("Project: `{project_name}` => initialzing ...");
    let current_dir = current_dir()?;
    let mut config = Config::default();
    config.project = project_name.to_owned();
    let asset_dirs = config.asset_dirs.to_owned();
    let template_dirs = config.template_dirs.to_owned();
    let content_dirs = config.content_dirs.to_owned();

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
    make_dirs(&project_name, content_dirs.to_owned())?;
    make_dirs(
        &format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"),
        content_dirs,
    )?;
    make_dirs(&format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"), asset_dirs)?;

    let blog_post = Post {
        title: "test blog".to_owned(),
        description: "This is a sample blog post".to_owned(),
        published_date: date_today(),
        tags: string_vec!["stuff", "things"],
        ..Default::default()
    };

    // Files
    // Config files
    make_file(
        &format!("{CONFIG_DIR}{PATH_SEP}{CONFIG_FILE}"),
        &serde_json::to_string_pretty(&config)?,
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{DATA_DIR}{PATH_SEP}{BLOG_DATA_FILE}"),
        &serde_json::to_string_pretty(&vec![blog_post])?,
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{ASSETS_DIR}{PATH_SEP}{CSS_DIR}{PATH_SEP}{TAILWIND_INPUT_FILE}"),
        &tailwind::CSS.to_owned(),
    )?;
    // Templsate files
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{LAYOUTS_DIR}{PATH_SEP}site.jinja"
        ),
        &html::SITE_LAYOUT.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{LAYOUTS_DIR}{PATH_SEP}blog.jinja"
        ),
        &html::BLOG_LAYOUT.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{INCLUDES_DIR}{PATH_SEP}footer.jinja"
        ),
        &html::FOOTER_INCLUDE.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{INCLUDES_DIR}{PATH_SEP}lorem-ipsum.jinja"
        ),
        &html::LOREM_IPSUM_INCLUDE.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}page-header.jinja"
        ),
        &html::PAGE_HEADER_MACRO.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}link-to.jinja"
        ),
        &html::LINK_TO_MACRO.to_owned(),
    )?;
    make_file(
        &format!(
            "{project_name}{PATH_SEP}{TEMPLATES_DIR}{PATH_SEP}{MACROS_DIR}{PATH_SEP}tags.jinja"
        ),
        &html::TAGS_MACRO.to_owned(),
    )?;

    // Site files
    make_file(
        &format!("{project_name}{PATH_SEP}{project_name}.jinja"),
        &html::SITE_INDEX_TEMPLATE.to_owned(),
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{BLOG_DIR}{PATH_SEP}test-blog.jinja"),
        &html::BLOG_POST_TEMPLATE.to_owned(),
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

    println!("Project: `{project_name}` => initialization complete");
    build()?;
    Ok(config)
}

/// Build site
pub fn build() -> Result<()> {
    let current_dir = current_dir()?;
    let config = load_config()?;
    let project_name = config.project.to_owned();
    let output_dir = config.output_dir.to_owned();
    let content_dirs = config.content_dirs.to_owned();
    let posts = Posts::init(&config)?;

    println!("Project: `{project_name}` => building ...");

    // Template environment
    let mut env = init_env(&current_dir, &project_name)?;

    // Project index template file
    env.add_template_file(
        format!("{project_name}{PATH_SEP}{project_name}.jinja"),
        None,
    )?;
    let mut index_ctx = Context::new();
    index_ctx.insert("project", &project_name);
    index_ctx.insert("config", &config);
    index_ctx.insert("posts", &posts);
    let tmpl = render_template(
        &env,
        &format!("{project_name}{PATH_SEP}{project_name}.jinja"),
        &index_ctx,
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{HTML_INDEX_FILE}"),
        &tmpl,
    )?;

    for dir in content_dirs {
        let dir_posts: Vec<Post> = match posts.by_content.get(&dir) {
            Some(posts) => posts.to_owned(),
            None => vec![],
        };
        let mut dir_ctx = Context::new();
        dir_ctx.insert("project", &project_name);
        dir_ctx.insert("posts", &posts);
        let dir_tmpl = render_template(
            &env,
            &format!("{LAYOUTS_DIR}{PATH_SEP}{dir}.jinja"),
            &dir_ctx,
        )?;
        make_file(
            &format!(
                "{project_name}{PATH_SEP}{OUTPUT_DIR}{PATH_SEP}{dir}{PATH_SEP}{HTML_INDEX_FILE}"
            ),
            &dir_tmpl,
        )?;

        for post in dir_posts {
            let post_title = parameterize(post.title.to_owned());
            let file_name = format!("{post_title}.jinja");
            make_dirs(
                &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}"),
                vec![post_title.to_owned()],
            )?;
            env.add_template_file(
                format!("{project_name}{PATH_SEP}{dir}{PATH_SEP}{file_name}"),
                Some(&format!("{dir}{PATH_SEP}{file_name}")),
            )?;

            let mut post_ctx = Context::new();
            post_ctx.insert("project", &project_name);
            post_ctx.insert("post", &post);
            post_ctx.insert("posts", &posts);
            let tmpl = render_template(&env, &format!("{dir}{PATH_SEP}{file_name}"), &post_ctx)?;
            make_file(
                &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}{PATH_SEP}{post_title}{PATH_SEP}{HTML_INDEX_FILE}"),
                &tmpl,
            )?;
        }
    }

    // Move assets
    copy_recursively(
        format!("{project_name}{PATH_SEP}{ASSETS_DIR}"),
        format!("{project_name}{PATH_SEP}{OUTPUT_DIR}"),
    )?;

    println!("Project: `{project_name}` => build complete");
    Ok(())
}
