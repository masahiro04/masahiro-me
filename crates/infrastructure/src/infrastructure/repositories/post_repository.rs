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

#[cfg(test)]
mod tests {
    use super::PostRepository;
    use crate::repositories::post_repository::post_from_api::PostFromApi;
    use domain::entities::post::Post;
    use domain::repositories::post_repository::IPostRepository;

    #[tokio::test]
    async fn test_find_all() -> anyhow::Result<()> {
        let posts_from_api = vec![PostFromApi {
            title: "title".to_string(),
            slug: "slug".to_string(),
            date: "date".to_string(),
            excerpt: "excerpt".to_string(),
            content: "content".to_string(),
            categories: vec![],
            tags: vec![],
            featured_media: "featured_media".to_string(),
        }];
        let posts = posts_from_api
            .clone()
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>();
        let json_string = serde_json::to_string(&posts_from_api).unwrap();
        let mock = create_mock_server("/posts?per_page=1&offset=1".to_string(), json_string, 200);
        let repository = PostRepository::new(mock.0.url());

        assert_eq!(repository.find_posts(1, 1).await?, posts);
        mock.1.assert();
        mock.1.remove();
        Ok(())
    }

    fn create_mock_server(
        path: String,
        json_string: String,
        status_code: usize,
    ) -> (mockito::ServerGuard, mockito::Mock) {
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", path.as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(json_string)
            .create();
        (server, mock)
    }
}
