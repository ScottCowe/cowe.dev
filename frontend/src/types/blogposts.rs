use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlogpostData {
    pub id: String,
    pub title: String,
    pub format: String,
    pub created_on: NaiveDateTime,
    pub updated_on: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Blogpost {
    pub id: String,
    pub title: String,
    pub format: String,
    pub created_on: NaiveDateTime,
    pub updated_on: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
    pub content: String,
}
