use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogpostData {
    pub id: String,
    pub title: String,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Blogpost {
    pub id: String,
    pub title: String,
    pub format: String,
    pub content: String,
}
