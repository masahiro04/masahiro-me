use serde::{Deserialize, Serialize};

use crate::domain::entities::category::Category;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    slug: String,
    date: String,
    excerpt: String,
    content: String,
    categories: Vec<Category>,
    featured_media: String,
}

impl Post {
    pub fn reconstruct(
        title: &str,
        slug: &str,
        date: &str,
        excerpt: &str,
        content: &str,
        categories: Vec<Category>,
        featured_media: &str,
    ) -> Self {
        Self {
            title: title.to_string(),
            slug: slug.to_string(),
            date: date.to_string(),
            excerpt: excerpt.to_string(),
            content: content.to_string(),
            categories,
            featured_media: featured_media.to_string(),
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
    pub fn featured_media(&self) -> &str {
        &self.featured_media
    }
}
