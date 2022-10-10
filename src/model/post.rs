use serde::{Deserialize, Serialize};

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
