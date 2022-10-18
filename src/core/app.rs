use std::fs;
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::Result;
use minijinja::{context, Source};

use crate::core::konst::{
    ASSETS_DIR, BLOG_DATA_FILE, BLOG_DIR, CONFIG_DIR, CONFIG_FILE, CSS_DIR, DATA_DIR,
    HTML_INDEX_FILE, INCLUDES_DIR, LAYOUTS_DIR, MACROS_DIR, OUTPUT_DIR, PROC_FILE, PROC_FILE_DEV,
    TAILWIND_CONFIG_FILE, TAILWIND_INPUT_FILE, TEMPLATES_DIR,
};
use crate::model::config::Config;
use crate::model::post::Post;
use crate::string_vec;
use crate::template::html;
use crate::template::proc;
use crate::template::tailwind;
use crate::util::date_time::date_today;
use crate::util::file_sys::{make_dirs, make_file};
use crate::util::template::{init_env, load_templates, render_template};
use crate::util::text::parameterize;

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
    make_dirs(
        &project_name,
        vec![
            CONFIG_DIR.to_owned(),
            DATA_DIR.to_owned(),
            OUTPUT_DIR.to_owned(),
        ],
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
        &CONFIG_FILE.to_owned(),
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
    source.add_template(TAILWIND_CONFIG_FILE, tailwind::CONFIG)?;
    env.set_source(source.to_owned());
    let tailwind_tmpl = render_template(
        &env,
        TAILWIND_CONFIG_FILE,
        context!(project => project_name, path_sep => PATH_SEP, output_dir => OUTPUT_DIR),
    )?;
    make_file(&TAILWIND_CONFIG_FILE.to_owned(), &tailwind_tmpl)?;

    // Procfile
    source.add_template(PROC_FILE, proc::PROCFILE)?;
    env.set_source(source.to_owned());
    let procfile_tmpl = render_template(
        &env,
        PROC_FILE,
        context!(project => project_name, path_sep => PATH_SEP, output_dir => OUTPUT_DIR),
    )?;
    make_file(&PROC_FILE.to_owned(), &procfile_tmpl)?;

    // Procfile.dev
    source.add_template(PROC_FILE_DEV, proc::PROCFILE_DEV)?;
    env.set_source(source.to_owned());
    let procfile_dev_tmpl = render_template(
        &env,
        PROC_FILE_DEV,
        context!(project => project_name, path_sep => PATH_SEP, output_dir => OUTPUT_DIR),
    )?;
    make_file(&PROC_FILE_DEV.to_owned(), &procfile_dev_tmpl)?;

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

    // Site index file
    let index_template_string =
        fs::read_to_string(format!("{project_name}{PATH_SEP}{project_name}.jinja"))?;
    source.add_template(
        format!("{project_name}{PATH_SEP}{project_name}.jinja"),
        index_template_string,
    )?;
    env.set_source(source.to_owned());
    let tmpl = render_template(
        &env,
        format!("{project_name}{PATH_SEP}{project_name}.jinja").as_str(),
        context!(project => project_name, config => config),
    )?;
    make_file(
        &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{HTML_INDEX_FILE}"),
        &tmpl,
    )?;

    for dir in content_dirs {
        let data_file = fs::read_to_string(format!(
            "{project_name}{PATH_SEP}{data_dir}{PATH_SEP}{dir}.json"
        ))?;
        let posts: Vec<Post> = serde_json::from_str(data_file.as_str())?;

        let dir_tmpl = render_template(
            &env,
            &format!("{LAYOUTS_DIR}{PATH_SEP}{dir}.jinja"),
            context!(project => project_name, posts => posts),
        )?;
        make_file(
            &format!(
                "{project_name}{PATH_SEP}{OUTPUT_DIR}{PATH_SEP}{dir}{PATH_SEP}{HTML_INDEX_FILE}"
            ),
            &dir_tmpl,
        )?;

        for post in posts {
            let post_title = parameterize(post.title.to_owned());
            let file_name = format!("{post_title}.jinja");
            make_dirs(
                &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}"),
                vec![post_title.to_owned()],
            )?;

            let template_string = fs::read_to_string(format!(
                "{project_name}{PATH_SEP}{dir}{PATH_SEP}{file_name}"
            ))?;
            source.add_template(format!("{dir}{PATH_SEP}{file_name}"), template_string)?;
            env.set_source(source.to_owned());
            let tmpl = render_template(
                &env,
                format!("{dir}{PATH_SEP}{file_name}").as_str(),
                context!(project => project_name, post => post),
            )?;
            make_file(
                &format!("{project_name}{PATH_SEP}{output_dir}{PATH_SEP}{dir}{PATH_SEP}{post_title}{PATH_SEP}{HTML_INDEX_FILE}"),
                &tmpl,
            )?;
        }
        println!("Project: `{project_name}` => build complete");
    }
    Ok(())
}
