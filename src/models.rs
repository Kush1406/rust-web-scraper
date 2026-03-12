use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub title: String,
    pub url: Option<String>,
    pub points: Option<u32>,
    pub comments: Option<u32>,
    pub author: Option<String>,
    pub time: Option<String>,
}
