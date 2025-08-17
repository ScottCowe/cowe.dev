use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct BlogpostData {
    pub id: String,
    pub title: String,
    pub format: String,
}

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Blogpost {
    pub id: String,
    pub title: String,
    pub format: String,
    pub content: String,
}
