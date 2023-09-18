mod category_from_api;
pub mod post_from_api;
use self::post_from_api::PostFromApi;
use async_trait::async_trait;
use domain::{entities::post::Post, repositories::post_repository::IPostRepository};

#[derive(Clone)]
pub struct PostRepository {
    api_url: String,
}

impl PostRepository {
    pub fn new(api_url: String) -> Self {
        Self { api_url }
    }
}

#[async_trait(?Send)]
impl IPostRepository for PostRepository {
    async fn find_posts(&self, per_page: i32, offset: i32) -> anyhow::Result<Vec<Post>> {
        let url = format!(
            "{}/posts?per_page={}&offset={}",
            &self.api_url, per_page, offset
        );
        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let posts_from_api = response
            .json::<Vec<PostFromApi>>()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let posts = posts_from_api
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>();
        Ok(posts)
    }

    async fn find_related_posts(&self, category_ids: &str) -> anyhow::Result<Vec<Post>> {
        let url = format!("{}/posts?category_ids={}", &self.api_url, category_ids);
        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let posts_from_api = response
            .json::<Vec<PostFromApi>>()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let posts = posts_from_api
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>();
        if posts.len() < 3 {
            return Ok(posts);
        }
        let posts = posts[0..3].to_vec();
        Ok(posts)
    }

    async fn find_post(&self, slug: String) -> anyhow::Result<Option<Post>> {
        let url = format!("{}/posts/{}", self.api_url, slug);
        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let post_from_api = response
            .json::<PostFromApi>()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let post = post_from_api
            .into_post()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(Some(post))
    }
}
