use crate::domain::{
    repositories::category_repository::ICategoryRepository,
    repositories::post_repository::IPostRepository,
};
use crate::infrastructure::api::ApiClient;
use crate::infrastructure::kv::KvClient;
use crate::infrastructure::repositories::converter::posts_from_api_to_posts_converter::posts_from_api_to_posts_converver;
use crate::infrastructure::repositories::types::post_from_api::PostFromApi;
use crate::{
    domain::entities::post::Post,
    infrastructure::repositories::{
        category_repository::CategoryRepository, media_repository::MediaRepository,
    },
};
use async_trait::async_trait;
use std::io::Result;
use worker::console_log;
use worker::kv::KvStore;

#[derive(Clone)]
pub struct PostRepository<'a> {
    api_url: String,
    store: &'a KvStore,
}
impl<'a> PostRepository<'a> {
    pub fn new(api_url: String, store: &'a KvStore) -> Self {
        Self { api_url, store }
    }
}
#[async_trait(?Send)]
impl<'b> IPostRepository for PostRepository<'b> {
    async fn find_all<'a>(&'a self, per_page: &'a str, offset: &'a str) -> Result<Vec<Post>> {
        let cache_key = format!("posts/{}/{}", per_page, offset);
        let kv_client = KvClient::new(&self.store);
        let kv_data = kv_client.get(&cache_key).await;
        let posts_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<PostFromApi>>().await.unwrap(),
            None => {
                let url = format!(
                    "{}/posts?_embed&per_page={}&offset={}",
                    self.api_url.clone(),
                    per_page,
                    offset
                );
                let api = ApiClient::new();
                let _posts = match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                };
                let data: Vec<u8> = serde_json::to_vec(&_posts)?;
                let posts_string = String::from_utf8(data.clone()).unwrap();
                match kv_client.put(&cache_key, posts_string).await {
                    Ok(_) => console_log!("{} cache saved", cache_key),
                    Err(_) => console_log!("{} cache saved failed", cache_key),
                };
                _posts
            }
        };
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.store);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };
        let media_repository = MediaRepository::new(self.api_url.clone(), self.store);
        let posts =
            posts_from_api_to_posts_converver(posts_from_api, categories, media_repository).await;
        Ok(posts)
    }

    async fn find_related(&self, category_ids: &str) -> Result<Vec<Post>> {
        let cache_key = format!("posts/related/{}", category_ids);
        let kv_client = KvClient::new(self.store);
        let kv_data = kv_client.get(&cache_key).await;
        let posts_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<PostFromApi>>().await.unwrap(),
            None => {
                let url = format!("{}/posts?_embed&categories={}", self.api_url, category_ids);
                let api = ApiClient::new();
                let _posts = match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                };
                let posts_string = serde_json::to_string(&_posts).unwrap();
                match kv_client.put(&cache_key, posts_string).await {
                    Ok(_) => console_log!("{} cache saved", cache_key),
                    Err(_) => console_log!("{} cache saved failed", cache_key),
                };
                _posts
            }
        };
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.store);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };
        let media_repository = MediaRepository::new(self.api_url.clone(), self.store);
        let posts =
            posts_from_api_to_posts_converver(posts_from_api, categories, media_repository).await;
        Ok(posts)
    }

    async fn find_slugs<'a>(&'a self) -> Result<Vec<String>> {
        let per_page = 10;
        let mut offset: i32 = 0;
        let mut slugs: Vec<String> = vec![];
        loop {
            let posts = match self
                .find_all(&per_page.to_string(), &offset.to_string())
                .await
            {
                Ok(posts) => posts,
                Err(_) => Vec::new(),
            };
            let new_slugs = posts
                .clone()
                .into_iter()
                .map(|post| post.slug().to_string())
                .collect::<Vec<String>>();
            slugs.extend(new_slugs);
            // ポストの数が10より小さければ、ループを終了
            if posts.len() < 10 {
                break;
            }
            // オフセットを更新
            offset += 10;
        }
        Ok(slugs)
    }

    async fn find_one<'a>(&'a self, slug: &'a str) -> Result<Option<Post>> {
        let cache_key = format!("post-detail/{}", slug);
        let kv_client = KvClient::new(self.store);
        let kv_data = kv_client.get(&cache_key).await;
        let posts_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<PostFromApi>>().await.unwrap(),
            None => {
                let url = format!("{}/posts?_embed&slug={}", self.api_url, slug);
                let api = ApiClient::new();
                let _posts = match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                };
                let post_string = serde_json::to_string(&_posts).unwrap();
                match kv_client.put(&cache_key, post_string).await {
                    Ok(_) => console_log!("{} cache saved", cache_key),
                    Err(_) => console_log!("{} cache saved failed", cache_key),
                };
                _posts
            }
        };
        if posts_from_api.is_empty() {
            return Ok(None);
        }
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.store);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };

        let media_repository = MediaRepository::new(self.api_url.clone(), self.store);
        let posts =
            posts_from_api_to_posts_converver(posts_from_api, categories, media_repository).await;
        let post = posts
            .first()
            .map(|post| post.clone())
            .map(|post| Some(post))
            .unwrap_or(None);
        Ok(post)
    }
}
