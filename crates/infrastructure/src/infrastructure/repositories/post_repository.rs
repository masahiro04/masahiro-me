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
        println!("{}", url);
        let response = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        println!("Status: {}", response.status());
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
    use crate::repositories::post_repository::post_from_api::PostFromApi;

    use super::PostRepository;
    use domain::entities::post::Post;
    use domain::repositories::post_repository::IPostRepository;
    // use mockito::{Mock, ServerGuard};

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
        let mut server = mockito::Server::new();
        println!("sentinel1");

        // Use one of these addresses to configure your client
        let host = server.host_with_port();
        let url = server.url();

        println!("{}", host);
        println!("{}", url);
        println!("{}", json_string);
        println!("{}", server.host_with_port());
        // Create a mock

        let mock = server
            .mock("GET", "/posts?per_page=1&offset=1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_string)
            .create();

        let repository = PostRepository::new(server.url());
        // let mock = make_response(json_string);

        println!("sentinel3");
        assert_eq!(repository.find_posts(1, 1).await?, posts);
        mock.assert();
        mock.remove();
        Ok(())
    }

    // fn server() -> ServerGuard {
    //     mockito::Server::new()
    // }
    //
    // fn make_response(json_string: String) -> Mock {
    //     // Request a new server from the pool
    //     let mut server = server();
    //     println!("sentinel1");
    //
    //     // Use one of these addresses to configure your client
    //     let host = server.host_with_port();
    //     let url = server.url();
    //
    //     println!("{}", host);
    //     println!("{}", url);
    //     println!("{}", json_string);
    //     println!("{}", server.host_with_port());
    //     // Create a mock
    //
    //     server
    //         .mock("GET", "/posts?per_page=1&offset=1")
    //         .with_status(201)
    //         .with_header("content-type", "application/json")
    //         .with_body(json_string)
    //         .create()
    //
    //     // You can use `Mock::assert` to verify that your mock was called
    //     // mock.assert();
    // }
}
