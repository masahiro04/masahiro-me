use async_trait::async_trait;
use domain::{
    entities::{category::Category, post::Post},
    repositories::post_repository::IPostRepository,
};
use serde::{Deserialize, Serialize};
use std::io::Result;
extern crate reqwest;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryFromApi {
    pub id: i32,
    pub name: String,
}

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
    async fn find_posts(&self, per_page: i32, offset: i32) -> Result<Vec<Post>> {
        let url = format!(
            "{}/posts?per_page={}&offset={}",
            &self.api_url, per_page, offset
        );
        let client = reqwest::Client::new();
        let response = client.get(url).send().await;
        let posts_from_api: Vec<PostFromApi> = match response.unwrap().json().await {
            Ok(posts) => posts,
            Err(_) => Vec::new(),
        };
        let posts = posts_from_api
            .iter()
            .map(|post_from_api| {
                let categories_from_api = post_from_api.categories.clone();
                let categories = categories_from_api
                    .iter()
                    .map(|category_from_api| {
                        Category::reconstruct(category_from_api.id, category_from_api.name.clone())
                    })
                    .collect::<Vec<Category>>();
                Post::reconstruct(
                    post_from_api.title.clone(),
                    post_from_api.slug.clone(),
                    post_from_api.date.clone(),
                    post_from_api.excerpt.clone(),
                    post_from_api.content.clone(),
                    categories,
                    post_from_api.featured_media.clone(),
                )
            })
            .collect::<Vec<Post>>();

        Ok(posts)
    }

    async fn find_related_posts(&self, category_ids: &str) -> Result<Vec<Post>> {
        let url = format!("{}/posts?category_ids={}", &self.api_url, category_ids);
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        let posts_from_api: Vec<PostFromApi> = match response.json().await {
            Ok(posts) => posts,
            Err(_) => Vec::new(),
        };
        let posts = posts_from_api
            .iter()
            .map(|post_from_api| {
                let categories_from_api = post_from_api.categories.clone();
                let categories = categories_from_api
                    .iter()
                    .map(|category_from_api| {
                        Category::reconstruct(
                            category_from_api.id,
                            category_from_api.name.to_string(),
                        )
                    })
                    .collect::<Vec<Category>>();
                Post::reconstruct(
                    post_from_api.title.clone(),
                    post_from_api.slug.clone(),
                    post_from_api.date.clone(),
                    post_from_api.excerpt.clone(),
                    post_from_api.content.clone(),
                    categories,
                    post_from_api.featured_media.clone(),
                )
            })
            .collect::<Vec<Post>>();
        if posts.len() < 3 {
            return Ok(posts);
        }
        let posts = posts[0..3].to_vec();
        Ok(posts)
    }
    async fn find_post(&self, slug: String) -> Result<Option<Post>> {
        let url = format!("{}/posts/{}", self.api_url, slug);
        let client = reqwest::Client::new();
        let response = client.get(url).send().await.unwrap();
        let post_from_api: PostFromApi = match response.json().await {
            Ok(post) => post,
            Err(_) => {
                return Ok(None);
            }
        };
        let categories_from_api = post_from_api.categories.clone();
        let categories = categories_from_api
            .iter()
            .map(|category_from_api| {
                Category::reconstruct(category_from_api.id, category_from_api.name.to_string())
            })
            .collect::<Vec<Category>>();
        let post = Post::reconstruct(
            post_from_api.title,
            post_from_api.slug,
            post_from_api.date,
            post_from_api.excerpt,
            post_from_api.content,
            categories,
            post_from_api.featured_media,
        );
        Ok(Some(post))
    }
}
