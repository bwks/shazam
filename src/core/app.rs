use minijinja::{context, Environment, Source};
use std::fs;
use std::io::prelude::*;

use crate::core::konst::{
    ASSETS_DIR, BLOG_DATA_FILE, BLOG_DIR, CONFIG_DIR, CONFIG_FILE, CSS_DIR, DATA_DIR,
    HTML_INDEX_FILE, INCLUDES_DIR, LAYOUTS_DIR, OUTPUT_DIR, PROC_FILE, TAILWIND_CONFIG_FILE,
    TAILWIND_INPUT_FILE, TEMPLATES_DIR,
};

use crate::model::config::Config;
use crate::model::post::Post;
use crate::template::html;
use crate::template::proc;
use crate::template::tailwind;
use crate::util::file_sys::{make_dirs, make_file};
use crate::util::template::render_template;
use crate::util::text::dasherize;

// Initial Build of site
pub fn init(project_name: String) -> Config {
    println!("Project `{}` initialzing...", project_name);
    let mut config = Config::default();
    config.project = project_name.to_owned();
    let asset_dirs = config.asset_dirs.to_owned();
    let template_dirs = config.template_dirs.to_owned();
    let content_dirs = config.content_dirs.to_owned();

    // Template environment
    let mut env = Environment::new();
    let mut source = Source::new();

    // Directories
    make_dirs(&project_name, vec![CONFIG_DIR.to_owned()]);
    make_dirs(&project_name, vec![DATA_DIR.to_owned()]);
    make_dirs(&project_name, vec![OUTPUT_DIR.to_owned()]);
    make_dirs(
        &format!("{project_name}/{ASSETS_DIR}"),
        asset_dirs.to_owned(),
    );
    make_dirs(
        &format!("{project_name}/{TEMPLATES_DIR}"),
        template_dirs.to_owned(),
    );
    make_dirs(&project_name, content_dirs.to_owned());

    let mut blog_post = Post::default();
    blog_post.title = "Test Blog".to_owned();
    blog_post.published_date = "2022/10/09".to_owned();

    // Files
    make_file(
        &CONFIG_FILE.to_owned(),
        &serde_json::to_string_pretty(&config).unwrap(),
    );
    make_file(
        &format!("{project_name}/{DATA_DIR}/{BLOG_DATA_FILE}"),
        &serde_json::to_string_pretty(&vec![blog_post]).unwrap(),
    );
    make_file(
        &format!("{project_name}/{ASSETS_DIR}/{CSS_DIR}/{TAILWIND_INPUT_FILE}"),
        &tailwind::CSS.to_owned(),
    );
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{LAYOUTS_DIR}/base.jinja"),
        &html::BASE.to_owned(),
    );
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{LAYOUTS_DIR}/blog.jinja"),
        &html::BLOG.to_owned(),
    );
    make_file(
        &format!("{project_name}/{BLOG_DIR}/test-blog.jinja"),
        &html::BLOG_POST.to_owned(),
    );
    make_file(
        &format!("{project_name}/{TEMPLATES_DIR}/{INCLUDES_DIR}/_footer.jinja"),
        &html::FOOTER.to_owned(),
    );

    // Build base site
    // Site assets
    make_dirs(&format!("{project_name}/{OUTPUT_DIR}"), asset_dirs);

    // Tailwind template
    source
        .add_template(TAILWIND_CONFIG_FILE, tailwind::CONFIG)
        .unwrap();
    env.set_source(source.to_owned());
    let tailwind_tmpl = render_template(
        &env,
        TAILWIND_CONFIG_FILE,
        context!(project => project_name, output_dir => OUTPUT_DIR),
    );
    make_file(&TAILWIND_CONFIG_FILE.to_owned(), &tailwind_tmpl);

    // Procfile
    source.add_template(PROC_FILE, proc::PROCFILE).unwrap();
    env.set_source(source.to_owned());
    let procfile_tmpl = render_template(
        &env,
        PROC_FILE,
        context!(project => project_name, output_dir => OUTPUT_DIR),
    );
    make_file(&PROC_FILE.to_owned(), &procfile_tmpl);

    // Load all templaes
    let mut all_templates: Vec<String> = vec![];
    for dir in &template_dirs {
        for entry in fs::read_dir(format!("{project_name}/{TEMPLATES_DIR}/{dir}/")).unwrap() {
            let file = entry.unwrap().file_name().into_string().unwrap();
            if file.ends_with(".jinja") || file.ends_with(".j2") {
                all_templates.push(format!("{dir}/{file}"))
            }
        }
    }
    for template in all_templates {
        let template_string =
            fs::read_to_string(format!("{project_name}/{TEMPLATES_DIR}/{template}")).unwrap();
        source.add_template(template, template_string).unwrap();
        env.set_source(source.to_owned());
    }

    // Site index file
    let base_tmpl = render_template(
        &env,
        &format!("{LAYOUTS_DIR}/base.jinja"),
        context!(project => project_name),
    );
    make_file(
        &format!("{project_name}/{OUTPUT_DIR}/{HTML_INDEX_FILE}"),
        &base_tmpl,
    );

    // Blog index file
    make_dirs(&format!("{project_name}/{OUTPUT_DIR}"), content_dirs);
    let blog_file =
        fs::read_to_string(format!("{project_name}/{DATA_DIR}/{BLOG_DATA_FILE}")).unwrap();
    let blog_posts: Vec<Post> = serde_json::from_str(blog_file.as_str()).unwrap();

    let blog_tmpl = render_template(
        &env,
        &format!("{LAYOUTS_DIR}/blog.jinja"),
        context!(project => project_name, blog_posts => blog_posts),
    );
    make_file(
        &format!("{project_name}/{OUTPUT_DIR}/{BLOG_DIR}/{HTML_INDEX_FILE}"),
        &blog_tmpl,
    );

    build();
    return config;
}

// Rebuild site
pub fn build() {
    println!("Project building...");
    let config_file = fs::read_to_string(CONFIG_FILE).unwrap();
    let config: Config = serde_json::from_str(config_file.as_str()).unwrap();
    let project_name = config.project;

    let blog_file = fs::read_to_string(format!(
        "{}/{}/{}",
        project_name, config.data_dir, BLOG_DATA_FILE
    ))
    .unwrap();
    let blog_posts: Vec<Post> = serde_json::from_str(blog_file.as_str()).unwrap();

    // Load templaes
    let mut all_templates: Vec<String> = vec![];
    for dir in &config.template_dirs {
        for entry in fs::read_dir(format!("{}/templates/{}/", project_name, dir)).unwrap() {
            let file = entry.unwrap().file_name().into_string().unwrap();
            if file.ends_with(".jinja") || file.ends_with(".j2") {
                all_templates.push(format!("{}/{}", dir, file))
            }
        }
    }

    // Template environment
    let mut env = Environment::new();
    let mut source = Source::new();
    for template in all_templates {
        let template_string =
            fs::read_to_string(format!("{}/templates/{}", project_name, template)).unwrap();
        source.add_template(template, template_string).unwrap();
        env.set_source(source.to_owned());
    }

    for post in blog_posts {
        let file_name = format!("{}.jinja", dasherize(post.title.to_owned()));
        make_dirs(
            &format!("{}/{}/blog", project_name, config.output_dir),
            vec![dasherize(post.title.to_owned())],
        );
        let template_string =
            fs::read_to_string(format!("{}/blog/{}", project_name, file_name)).unwrap();
        source
            .add_template(format!("blog/{}", file_name), template_string)
            .unwrap();
        env.set_source(source.to_owned());

        let tmpl = env
            .get_template(format!("blog/{}", file_name).as_str())
            .unwrap();
        let blog_tmpl = tmpl
            .render(context!(project => project_name, post => post))
            .unwrap();
        let mut blog_file = fs::File::create(format!(
            "{}/{}/blog/{}/{}",
            project_name,
            config.output_dir,
            dasherize(post.title.to_owned()),
            HTML_INDEX_FILE,
        ))
        .unwrap();
        blog_file.write_all(blog_tmpl.as_bytes()).unwrap();
    }
}
