use std::sync::Arc;

use domain::entities::post::Post;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Post not found")]
    NotFound,
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait::async_trait(?Send)]
pub trait PostRepository {
    async fn find_posts(&self, per_page: i32, offset: i32) -> Result<Vec<Post>>;
    async fn find_post(&self, slug: String) -> Result<Option<Post>>;
    async fn find_related_posts(&self, category_ids: &str) -> Result<Vec<Post>>;
}

pub trait ProvidePostRepository {
    fn post_repository(&self) -> Arc<dyn PostRepository + Send + Sync>;
}
