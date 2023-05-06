use super::{
    fetch_post_slugs_usecase::FetchSlugsUsecase, fetch_post_usecase::FetchPostUsecase,
    fetch_posts_usecase::FetchPostsUsecase, fetch_related_posts_usecase::FetchRelatedPostsUsecase,
};
use crate::{
    domain::entities::post::Post, infrastructure::repositories::post_repository::PostRepository,
};
use std::io::Result;
use worker::{kv::KvStore, *};

pub async fn fetch_posts_usecase(
    store: &KvStore,
    per_page: &str,
    offset: &str,
) -> Result<Vec<Post>> {
    let api_url = format!("{}", "https://mokubo.website/wp-json/wp/v2");
    let repo = PostRepository::new(api_url, store);
    let usecase = FetchPostsUsecase::new(repo);
    usecase.execute(per_page, offset).await
}
pub async fn fetch_slugs_usecase(store: &KvStore) -> Result<Vec<String>> {
    let api_url = format!("{}", "https://mokubo.website/wp-json/wp/v2");
    let repo = PostRepository::new(api_url, store);
    let usecase = FetchSlugsUsecase::new(repo);
    usecase.execute().await
}
pub async fn fetch_related_posts_usecase(store: &KvStore, category_ids: &str) -> Result<Vec<Post>> {
    let api_url = format!("{}", "https://mokubo.website/wp-json/wp/v2");
    let repo = PostRepository::new(api_url, store);
    let usecase = FetchRelatedPostsUsecase::new(repo);
    usecase.execute(category_ids).await
}
pub async fn fetch_post_usecase(store: &KvStore, slug: &str) -> Result<Option<Post>> {
    let api_url = format!("{}", "https://mokubo.website/wp-json/wp/v2");
    let repo = PostRepository::new(api_url, store);
    let usecase = FetchPostUsecase::new(repo);
    usecase.execute(slug).await
}
