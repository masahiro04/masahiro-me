mod category_from_api;
pub mod post_from_api;
use self::post_from_api::PostFromApi;
use async_trait::async_trait;
use domain::entities::post::Post;
use use_case::port::post_repository::{Error, PostRepository as PostRepositoryPort, Result};

#[derive(Clone)]
pub struct PostRepository {
    api_url: String,
    client: reqwest::Client,
}

impl PostRepository {
    pub fn new(api_url: String, client: reqwest::Client) -> Self {
        Self { api_url, client }
    }
}

#[async_trait(?Send)]
impl PostRepositoryPort for PostRepository {
    async fn find_posts(&self, per_page: i32, offset: i32) -> Result<Vec<Post>> {
        let url = format!(
            "{}/posts?per_page={}&offset={}",
            &self.api_url, per_page, offset
        );
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        let posts_from_api = response
            .json::<Vec<PostFromApi>>()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;

        Ok(posts_from_api
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>())
    }
    async fn find_related_posts(&self, category_ids: &str) -> Result<Vec<Post>> {
        let url = format!("{}/posts?category_ids={}", &self.api_url, category_ids);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        let posts_from_api = response
            .json::<Vec<PostFromApi>>()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
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
    async fn find_post(&self, slug: String) -> Result<Option<Post>> {
        let url = format!("{}/posts/{}", self.api_url, slug);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        let post_from_api = response
            .json::<PostFromApi>()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        let post = post_from_api
            .into_post()
            .map_err(|e| Error::Parse(e.to_string()))?;
        Ok(Some(post))
    }
}

#[cfg(test)]
mod tests {
    use super::PostRepository;
    use crate::repositories::post_repository::post_from_api::PostFromApi;
    use domain::entities::post::Post;
    use use_case::port::post_repository::PostRepository as PostRepositoryPort;

    #[tokio::test]
    async fn test_find_all() -> anyhow::Result<()> {
        let posts_from_api = make_posts_from_api();
        let posts = posts_from_api
            .clone()
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>();
        let json_string = serde_json::to_string(&posts_from_api).unwrap();
        let mock =
            create_mock_server("/posts?per_page=1&offset=1".to_string(), json_string, 200).await;

        let client = reqwest::Client::new();
        let repository = PostRepository::new(mock.0.url(), client);

        assert_eq!(repository.find_posts(1, 1).await?, posts);
        mock.1.assert_async().await;
        mock.1.remove_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_find_related_posts() -> anyhow::Result<()> {
        let category_id = 1;
        let posts_from_api = make_posts_from_api();
        let posts = posts_from_api
            .clone()
            .into_iter()
            .map(|post_from_api| post_from_api.into_post().unwrap())
            .collect::<Vec<Post>>();
        let json_string = serde_json::to_string(&posts_from_api).unwrap();
        let mock = create_mock_server(
            format!("/posts?category_ids={}", category_id),
            json_string,
            200,
        )
        .await;
        let client = reqwest::Client::new();
        let repository = PostRepository::new(mock.0.url(), client);

        assert_eq!(
            repository
                .find_related_posts(category_id.to_string().as_str())
                .await?,
            posts
        );
        mock.1.assert_async().await;
        mock.1.remove_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_find_post() -> anyhow::Result<()> {
        let slug = "slug";
        let posts_from_api = make_posts_from_api();
        if posts_from_api.is_empty() {
            panic!("posts_from_api is empty");
        }
        let post_from_api = &posts_from_api[0];
        let post = post_from_api.into_post()?;
        let json_string = serde_json::to_string(post_from_api).unwrap();
        let mock = create_mock_server(format!("/posts/{}", slug), json_string, 200).await;

        let client = reqwest::Client::new();
        let repository = PostRepository::new(mock.0.url(), client);

        assert_eq!(repository.find_post(slug.to_string()).await?, Some(post));
        mock.1.assert_async().await;
        mock.1.remove_async().await;
        Ok(())
    }

    async fn create_mock_server(
        path: String,
        json_string: String,
        status_code: usize,
    ) -> (mockito::ServerGuard, mockito::Mock) {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", path.as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(json_string)
            .create_async()
            .await;
        (server, mock)
    }

    fn make_posts_from_api() -> Vec<PostFromApi> {
        vec![PostFromApi {
            title: "title".to_string(),
            slug: "slug".to_string(),
            date: "date".to_string(),
            excerpt: "excerpt".to_string(),
            content: "content".to_string(),
            categories: vec![],
            featured_media_url: "featured_media".to_string(),
        }]
    }
}
