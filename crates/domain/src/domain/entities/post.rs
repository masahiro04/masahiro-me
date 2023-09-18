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
        title: String,
        slug: String,
        date: String,
        excerpt: String,
        content: String,
        categories: Vec<Category>,
        featured_media: String,
    ) -> Self {
        Self {
            title,
            slug,
            date,
            excerpt,
            content,
            categories,
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
    pub fn featured_media(&self) -> &str {
        &self.featured_media
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entities::post::Post;

    #[test]
    fn test() -> anyhow::Result<()> {
        let title = "title".to_string();
        let slug = "slug".to_string();
        let date = "date".to_string();
        let excerpt = "excerpt".to_string();
        let content = "content".to_string();
        let categories = vec![];
        let featured_media = "featured_media".to_string();

        // reconstruct
        let post = Post::reconstruct(
            title.clone(),
            slug.clone(),
            date.clone(),
            excerpt.clone(),
            content.clone(),
            categories.clone(),
            featured_media.clone(),
        );

        assert_eq!(post.title(), &title);
        assert_eq!(post.slug(), &slug);
        assert_eq!(post.date(), &date);
        assert_eq!(post.excerpt(), &excerpt);
        assert_eq!(post.content(), &content);
        assert_eq!(post.categories(), &categories);
        assert_eq!(post.featured_media(), &featured_media);
        Ok(())
    }
}
