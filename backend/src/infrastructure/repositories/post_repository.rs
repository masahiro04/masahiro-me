use crate::domain::{
    repositories::category_repository::ICategoryRepository,
    repositories::media_repository::IMediaRepository,
    repositories::post_repository::IPostRepository,
};
use crate::infrastructure::api::ApiClient;
use crate::infrastructure::kv::KvClient;
use crate::{
    domain::entities::{category::Category, post::Post},
    infrastructure::repositories::{
        category_repository::CategoryRepository, media_repository::MediaRepository,
    },
};
use async_trait::async_trait;
use futures::future::join_all;
use html2text::from_read;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::io::Result;
use std::sync::Arc;
use worker::wasm_bindgen::JsValue;
use worker::Env;

pub fn extract_text_from_html(html: &str) -> String {
    // NOTE: htmlを扱うたえにHTMLを生成
    let document = Html::parse_document(html);
    let body_selector = Selector::parse("body").unwrap();
    if let Some(body_element) = document.select(&body_selector).next() {
        let body_html = body_element.inner_html();
        let mut text = from_read(body_html.as_bytes(), 80);
        text.retain(|c| c != '\n');
        text
    } else {
        String::new()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderedContent {
    pub rendered: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostFromApi {
    pub slug: String,
    pub date: String,
    pub title: RenderedContent,
    pub content: RenderedContent,
    pub excerpt: RenderedContent,
    pub categories: Vec<i32>,
    pub featured_media: i32,
}

#[derive(Clone)]
pub struct PostRepository<'a> {
    api_url: String,
    env: &'a Env,
}

impl<'a> PostRepository<'a> {
    pub fn new(api_url: String, env: &'a Env) -> Self {
        Self { api_url, env }
    }
}

#[async_trait(?Send)]
impl<'b> IPostRepository for PostRepository<'b> {
    async fn find_all<'a>(&'a self, per_page: &'a str, offset: &'a str) -> Result<Vec<Post>> {
        let cache_key = format!("posts/{}/{}", per_page, offset);
        let kv_client = KvClient::new(&self.env);
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
                match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                }
            }
        };
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.env);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };
        // スレッド間共有(wasmでスレッドはないけど、async closureでデータを共有するために利用してる)
        let categories_arc = Arc::new(categories);
        let media_repository = MediaRepository::new(self.api_url.clone(), self.env);
        let post_futures = posts_from_api.into_iter().map(|post| async {
            let date_string = post.date.clone();
            let date = worker::js_sys::Date::new(&JsValue::from_str(&date_string));
            let year = date.get_full_year();
            let month = date.get_month() + 1; // JavaScriptの月は0から始まるため、1を足す
            let day = date.get_date();
            let formatted_date = format!("{}/{:02}/{:02}", year, month, day);
            let categories = Arc::clone(&categories_arc)
                .to_vec()
                .into_iter()
                .filter(|category| *&post.categories.contains(category.id()))
                .map(|category| category.clone())
                .collect::<Vec<Category>>();
            let excerpt = extract_text_from_html(&post.excerpt.rendered);
            let featured_media = match &media_repository
                .find_one(&post.featured_media.to_string())
                .await
            {
                Ok(media) => media.source_url().to_string(),
                Err(_) => "".to_string(),
            };
            Post::new(
                post.title.rendered,
                post.slug,
                formatted_date,
                excerpt,
                post.content.rendered,
                categories,
                vec![],
                featured_media,
            )
            .unwrap()
        });
        let posts = join_all(post_futures).await;
        Ok(posts)
    }

    async fn find_related(&self, category_ids: &str) -> Result<Vec<Post>> {
        let cache_key = format!("posts/related/{}", category_ids);
        let kv_client = KvClient::new(&self.env);
        let kv_data = kv_client.get(&cache_key).await;
        let posts_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<PostFromApi>>().await.unwrap(),
            None => {
                let url = format!("{}/posts?_embed&categories={}", self.api_url, category_ids);
                let api = ApiClient::new();
                match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                }
            }
        };
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.env);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };
        // スレッド間共有(wasmでスレッドはないけど、async closureでデータを共有するために利用してる)
        let categories_arc = Arc::new(categories);
        let media_repository = MediaRepository::new(self.api_url.clone(), self.env);
        let post_futures = posts_from_api.into_iter().map(|post| async {
            let date_string = post.date.clone();
            let date = worker::js_sys::Date::new(&JsValue::from_str(&date_string));
            let year = date.get_full_year();
            let month = date.get_month() + 1; // JavaScriptの月は0から始まるため、1を足す
            let day = date.get_date();
            let formatted_date = format!("{}/{:02}/{:02}", year, month, day);
            let categories = Arc::clone(&categories_arc)
                .to_vec()
                .into_iter()
                .filter(|category| *&post.categories.contains(category.id()))
                .map(|category| category.clone())
                .collect::<Vec<Category>>();
            let excerpt = extract_text_from_html(&post.excerpt.rendered);
            let featured_media = match &media_repository
                .find_one(&post.featured_media.to_string())
                .await
            {
                Ok(media) => media.source_url().to_string(),
                Err(_) => "".to_string(),
            };
            Post::new(
                post.title.rendered,
                post.slug,
                formatted_date,
                excerpt,
                post.content.rendered,
                categories,
                vec![],
                featured_media,
            )
            .unwrap()
        });
        let posts = join_all(post_futures).await;
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
        let kv_client = KvClient::new(self.env);
        let kv_data = kv_client.get(&cache_key).await;
        let posts_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<PostFromApi>>().await.unwrap(),
            None => {
                let url = format!("{}/posts?_embed&slug={}", self.api_url, slug);
                let api = ApiClient::new();
                match api.get::<Vec<PostFromApi>>(&url).await {
                    Ok(posts) => posts,
                    Err(_) => Vec::new(),
                }
            }
        };
        if posts_from_api.is_empty() {
            return Ok(None);
        }
        let post_from_api = posts_from_api.first().unwrap();
        let category_repository = CategoryRepository::new(self.api_url.clone(), self.env);
        let categories = match category_repository.find_all().await {
            Ok(categories) => categories,
            Err(_) => Vec::new(),
        };
        let media_repository = MediaRepository::new(self.api_url.clone(), self.env);
        let date_string = post_from_api.date.clone();
        let date = worker::js_sys::Date::new(&JsValue::from_str(&date_string));
        let year = date.get_full_year();
        let month = date.get_month() + 1; // JavaScriptの月は0から始まるため、1を足す
        let day = date.get_date();
        let formatted_date = format!("{}/{:02}/{:02}", year, month, day);
        let categories = categories
            .into_iter()
            .filter(|category| *&post_from_api.categories.contains(category.id()))
            .map(|category| category.clone())
            .collect::<Vec<Category>>();
        let excerpt = extract_text_from_html(&post_from_api.excerpt.rendered);
        let featured_media = match &media_repository
            .find_one(&post_from_api.featured_media.to_string())
            .await
        {
            Ok(media) => media.source_url().to_string(),
            Err(_) => "".to_string(),
        };
        let post = Post::new(
            post_from_api.title.rendered.clone(),
            post_from_api.slug.clone(),
            formatted_date,
            excerpt,
            post_from_api.content.rendered.clone(),
            categories,
            vec![],
            featured_media,
        )
        .unwrap();
        Ok(Some(post))
    }
}
