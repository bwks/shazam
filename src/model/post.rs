use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::MAIN_SEPARATOR as PATH_SEP;

use anyhow::Result;
use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::model::config::Config;
use crate::util::date_time::{date_today, to_date};
use crate::util::helper::load_data_file;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    HTML,
    JINJA,
    JSON,
    MD,
    XML,
}
impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            FileType::HTML => write!(f, "html"),
            FileType::JINJA => write!(f, "jinja"),
            FileType::JSON => write!(f, "json"),
            FileType::MD => write!(f, "md"),
            FileType::XML => write!(f, "xml"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Data {
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub author: String,
    pub author_email: String,
    pub published_date: String,
    pub updated_date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub publish: bool,
    pub file_type: FileType,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub bibliography: Vec<String>,
    pub table_of_contents: Vec<String>,
    pub links: Vec<String>,
}

impl Post {
    pub fn default() -> Post {
        Post {
            author: "".to_owned(),
            author_email: "".to_owned(),
            published_date: date_today(),
            updated_date: "".to_owned(),
            title: "".to_owned(),
            description: "".to_owned(),
            category: "uncategorised".to_owned(),
            publish: false,
            file_type: FileType::HTML,
            tags: vec![],
            references: vec![],
            bibliography: vec![],
            table_of_contents: vec![],
            links: vec![],
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Posts {
    pub all: Vec<Post>,
    pub draft: Vec<Post>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub years: Vec<i32>,
    pub by_content: HashMap<String, Vec<Post>>,
    pub by_category: HashMap<String, Vec<Post>>,
    pub by_tag: HashMap<String, Vec<Post>>,
    pub by_year: HashMap<i32, Vec<Post>>,
}

impl Posts {
    pub fn init(config: &Config) -> Result<Self> {
        let mut posts = Self::default();
        let project_name = config.project.to_owned();
        let data_dir = config.data_dir.to_owned();
        let content_dirs = config.content_dirs.to_owned();

        // groups
        let mut all_categories = HashSet::new();
        let mut all_tags = HashSet::new();
        let mut all_years = HashSet::new();
        let mut all_posts: Vec<Post> = vec![];
        let mut draft_posts: Vec<Post> = vec![];
        let mut posts_by_content: HashMap<String, Vec<Post>> = HashMap::new();
        let mut posts_by_category: HashMap<String, Vec<Post>> = HashMap::new();
        let mut posts_by_tag: HashMap<String, Vec<Post>> = HashMap::new();
        let mut posts_by_year: HashMap<i32, Vec<Post>> = HashMap::new();

        for dir in &content_dirs {
            let filename = format!("{project_name}{PATH_SEP}{data_dir}{PATH_SEP}{dir}.toml");
            let mut data = load_data_file(filename)?;
            data.posts
                .sort_by_key(|x| Reverse(x.published_date.to_owned()));

            for post in data.posts {
                all_categories.insert(post.category.to_owned());
                all_years.insert(to_date(post.published_date.to_owned())?.year());
                all_posts.push(post.to_owned());
                if !post.publish {
                    draft_posts.push(post.to_owned());
                }
                posts_by_content
                    .entry(dir.to_owned())
                    .or_default()
                    .push(post.to_owned());
                posts_by_category
                    .entry(post.category.to_owned())
                    .or_default()
                    .push(post.to_owned());
                posts_by_year
                    .entry(to_date(post.published_date.to_owned())?.year())
                    .or_default()
                    .push(post.to_owned());
                for tag in &post.tags {
                    all_tags.insert(tag.to_owned());
                    posts_by_tag
                        .entry(tag.to_owned())
                        .or_default()
                        .push(post.to_owned());
                }
            }
        }
        all_posts.sort_by_key(|x| Reverse(x.published_date.to_owned()));
        draft_posts.sort_by_key(|x| Reverse(x.published_date.to_owned()));
        posts.all = all_posts;
        posts.draft = draft_posts;
        posts.by_content = posts_by_content;
        posts.by_category = posts_by_category;
        posts.by_tag = posts_by_tag;
        posts.by_year = posts_by_year;
        posts.categories = to_string_vec(all_categories);
        posts.tags = to_string_vec(all_tags);
        posts.years = all_years.into_iter().collect();

        Ok(posts)
    }
}

fn to_string_vec(hash_set: HashSet<String>) -> Vec<String> {
    let mut result = hash_set.into_iter().collect::<Vec<String>>();
    result.sort_by_key(|x| x.to_lowercase());
    result
}
