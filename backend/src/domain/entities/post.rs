use crate::domain::entities::category::Category;
use serde::{Deserialize, Serialize};
use std::fmt::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    title: String,
    slug: String,
    date: String,
    excerpt: String,
    content: String,
    categories: Vec<Category>,
    tags: Vec<String>,
    featured_media: String,
}

impl Post {
    pub fn reconstruct(
        title: String,
        slug: String,
        date: String,
        excerpt: String,
        content: String,
        categories: Vec<Category>,
        tags: Vec<String>,
        featured_media: String,
    ) -> Self {
        Self {
            title,
            slug,
            date,
            excerpt,
            content,
            categories,
            tags,
            featured_media,
        }
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn slug(&self) -> &str {
        &self.slug
    }
    pub fn date(&self) -> &str {
        &self.date
    }
    pub fn excerpt(&self) -> &str {
        &self.excerpt
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn categories(&self) -> &Vec<Category> {
        &self.categories
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn featured_media(&self) -> &str {
        &self.featured_media
    }
}
