use crate::domain::entities::{category::Category, post::Post};
use crate::domain::repositories::media_repository::IMediaRepository;
use crate::infrastructure::repositories::converter::html_to_text_converter::html_to_text_converter;
use crate::infrastructure::repositories::media_repository::MediaRepository;
use crate::infrastructure::repositories::types::post_from_api::PostFromApi;
use futures::future::join_all;
use std::sync::Arc;
use worker::wasm_bindgen::JsValue;

pub async fn posts_from_api_to_posts_converver<'a>(
    posts_from_api: Vec<PostFromApi>,
    categories: Vec<Category>,
    media_repository: MediaRepository<'a>,
) -> Vec<Post> {
    let categories_arc = Arc::new(categories);
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
        let excerpt = html_to_text_converter(&post.excerpt.rendered);
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
    posts
}
