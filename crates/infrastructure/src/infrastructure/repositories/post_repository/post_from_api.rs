use super::category_from_api::CategoryFromApi;
use domain::entities::{category::Category, post::Post};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostFromApi {
    pub title: String,
    pub slug: String,
    pub date: String,
    pub excerpt: String,
    pub content: String,
    pub categories: Vec<CategoryFromApi>,
    pub tags: Vec<String>,
    pub featured_media: String,
}

impl PostFromApi {
    pub fn into_post(&self) -> anyhow::Result<Post> {
        let categories_from_api = self.categories.clone();
        let categories = categories_from_api
            .into_iter()
            .map(|category_from_api| category_from_api.into_category().unwrap())
            .collect::<Vec<Category>>();
        Ok(Post::reconstruct(
            self.title.clone(),
            self.slug.clone(),
            self.date.clone(),
            self.excerpt.clone(),
            self.content.clone(),
            categories,
            self.featured_media.clone(),
        ))
    }
}
