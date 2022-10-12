use minijinja::{context, Source};
use std::fs;

use anyhow::Result;

use crate::core::konst::{
    ASSETS_DIR, BLOG_DATA_FILE, BLOG_DIR, CONFIG_DIR, CONFIG_FILE, CSS_DIR, DATA_DIR,
    HTML_INDEX_FILE, INCLUDES_DIR, LAYOUTS_DIR, OUTPUT_DIR, PROC_FILE, PROC_FILE_DEV,
    TAILWIND_CONFIG_FILE, TAILWIND_INPUT_FILE, TEMPLATES_DIR,
};

use crate::model::config::Config;
use crate::model::post::Post;
use crate::template::html;
use crate::template::proc;
use crate::template::tailwind;
use crate::util::date_time::date_today;
use crate::util::file_sys::{make_dirs, make_file};
use crate::util::template::{init_env, load_templates, render_template};
use crate::util::text::dasherize;

/// Initial site directories and files
pub fn init(project_name: String) -> Result<Config> {
    println!("Project: `{project_name}` => initialzing ...");
    let mut config = Config::default();
    config.project = project_name.to_owned();
    let asset_dirs = config.asset_dirs.to_owned();
    let template_dirs = config.template_dirs.to_owned();
    let content_dirs = config.content_dirs.to_owned();

    // Template environment
    let mut env = init_env();
    let mut source = Source::new();

    // Build project
    // Directories
    make_dirs(&project_name, vec![CONFIG_DIR.to_owned()])?;
    make_dirs(&project_name, vec![DATA_DIR.to_owned()])?;
    make_dirs(&project_name, vec![OUTPUT_DIR.to_owned()])?;
    make_dirs(
        &format!("{project_name}/{ASSETS_DIR}"),
        asset_dirs.to_owned(),
    )?;
    make_dirs(
        &format!("{project_name}/{TEMPLATES_DIR}"),
        template_dirs.to_owned(),
    )?;
    make_dirs(&project_name, content_dirs.to_owned())?;
    make_dirs(&format!("{project_name}/{OUTPUT_DIR}"), content_dirs)?;
    make_dirs(&format!("{project_name}/{OUTPUT_DIR}"), asset_dirs)?;

    let mut blog_post = Post::default();
    blog_post.title = "test blog".to_owned();
    blog_post.published_date = date_today();

    // Files
    make_file(
        &CONFIG_FILE.to_owned(),
        &serde_json::to_string_pretty(&config)?,
    )?;
    make_file(
        &format!("{project_name}/{DATA_DIR}/{BLOG_DATA_FILE}"),
        &serde_json::to_string_pretty(&vec![blog_post])?,
    )?;
    make_file(
        &format!("{project_name}/{ASSETS_DIR}/{CSS_DIR}/{TAILWIND_INPUT_FILE}"),
        &tailwind::CSS.to_owned(),
    )?;
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{LAYOUTS_DIR}/base.jinja"),
        &html::BASE.to_owned(),
    )?;
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{LAYOUTS_DIR}/blog.jinja"),
        &html::BLOG.to_owned(),
    )?;
    make_file(
        &format!("{project_name}/{BLOG_DIR}/test-blog.jinja"),
        &html::BLOG_POST.to_owned(),
    )?;
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{INCLUDES_DIR}/footer.jinja"),
        &html::FOOTER.to_owned(),
    )?;

    // Tailwind config file
    source.add_template(TAILWIND_CONFIG_FILE, tailwind::CONFIG)?;
    env.set_source(source.to_owned());
    let tailwind_tmpl = render_template(
        &env,
        TAILWIND_CONFIG_FILE,
        context!(project => project_name, output_dir => OUTPUT_DIR),
    )?;
    make_file(&TAILWIND_CONFIG_FILE.to_owned(), &tailwind_tmpl)?;

    // Procfile
    source.add_template(PROC_FILE, proc::PROCFILE)?;
    env.set_source(source.to_owned());
    let procfile_tmpl = render_template(
        &env,
        PROC_FILE,
        context!(project => project_name, output_dir => OUTPUT_DIR),
    )?;
    make_file(&PROC_FILE.to_owned(), &procfile_tmpl)?;

    // Procfile.dev
    source.add_template(PROC_FILE_DEV, proc::PROCFILE_DEV)?;
    env.set_source(source.to_owned());
    let procfile_dev_tmpl = render_template(
        &env,
        PROC_FILE_DEV,
        context!(project => project_name, output_dir => OUTPUT_DIR),
    )?;
    make_file(&PROC_FILE_DEV.to_owned(), &procfile_dev_tmpl)?;

    load_templates(&mut env, &mut source, &config)?;

    // Site index file
    let base_tmpl = render_template(
        &env,
        &format!("{LAYOUTS_DIR}/base.jinja"),
        context!(project => project_name),
    )?;
    make_file(
        &format!("{project_name}/{OUTPUT_DIR}/{HTML_INDEX_FILE}"),
        &base_tmpl,
    )?;

    // Blog index file
    let blog_file = fs::read_to_string(format!("{project_name}/{DATA_DIR}/{BLOG_DATA_FILE}"))?;
    let blog_posts: Vec<Post> = serde_json::from_str(blog_file.as_str())?;

    let blog_tmpl = render_template(
        &env,
        &format!("{LAYOUTS_DIR}/blog.jinja"),
        context!(project => project_name, posts => blog_posts),
    )?;
    make_file(
        &format!("{project_name}/{OUTPUT_DIR}/{BLOG_DIR}/{HTML_INDEX_FILE}"),
        &blog_tmpl,
    )?;

    println!("Project: `{project_name}` => initialization complete");
    build()?;
    Ok(config)
}

/// Build site
pub fn build() -> Result<()> {
    let config_file = fs::read_to_string(CONFIG_FILE)?;
    let config: Config = serde_json::from_str(config_file.as_str())?;
    let project_name = config.project.to_owned();
    let output_dir = config.output_dir.to_owned();
    let data_dir = config.data_dir.to_owned();
    let content_dirs = config.content_dirs.to_owned();

    println!("Project: `{project_name}` => building ...");

    // Template environment
    let mut env = init_env();
    let mut source = Source::new();
    load_templates(&mut env, &mut source, &config)?;

    for dir in content_dirs {
        let data_file = fs::read_to_string(format!("{project_name}/{data_dir}/{dir}.json"))?;
        let posts: Vec<Post> = serde_json::from_str(data_file.as_str())?;

        for post in posts {
            let post_title = dasherize(post.title.to_owned());
            let file_name = format!("{post_title}.jinja");
            make_dirs(
                &format!("{project_name}/{output_dir}/{dir}"),
                vec![post_title.to_owned()],
            )?;

            let template_string = fs::read_to_string(format!("{project_name}/{dir}/{file_name}"))?;
            source.add_template(format!("{dir}/{file_name}"), template_string)?;
            env.set_source(source.to_owned());
            let tmpl = render_template(
                &env,
                format!("{dir}/{file_name}").as_str(),
                context!(project => project_name, post => post),
            )?;
            make_file(
                &format!("{project_name}/{output_dir}/{dir}/{post_title}/{HTML_INDEX_FILE}"),
                &tmpl,
            )?;
        }
        println!("Project: `{project_name}` => build complete");
    }
    Ok(())
}
