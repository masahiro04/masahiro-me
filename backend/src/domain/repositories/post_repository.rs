use crate::domain::entities::post::Post;
use async_trait::async_trait;
use std::io::Result;

// NOTE: https://github.com/rustwasm/wasm-bindgen/issues/2409#issuecomment-754574965
#[async_trait(?Send)]
pub trait IPostRepository {
    async fn find_all<'a>(&'a self, per_page: &'a str, offset: &'a str) -> Result<Vec<Post>>;
    async fn find_related(&self, category_ids: &str) -> Result<Vec<Post>>;
    async fn find_slugs<'a>(&'a self) -> Result<Vec<String>>;
    async fn find_one<'a>(&'a self, slug: &'a str) -> Result<Option<Post>>;
}
