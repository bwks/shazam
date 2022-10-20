use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Post {
    pub author: String,
    pub author_email: String,
    pub published_date: String,
    pub updated_date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub publish: bool,
    pub tags: Vec<String>,
    pub references: Vec<String>,
    pub bibliography: Vec<String>,
    pub table_of_contents: Vec<String>,
    pub links: Vec<String>,
}
