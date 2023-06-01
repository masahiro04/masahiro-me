use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderedContent {
    pub rendered: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFromApi {
    pub slug: String,
    pub date: String,
    pub title: RenderedContent,
    pub content: RenderedContent,
    pub excerpt: RenderedContent,
    pub categories: Vec<i32>,
    pub featured_media: i32,
}
