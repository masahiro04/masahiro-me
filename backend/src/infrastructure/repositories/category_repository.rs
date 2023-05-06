use super::types::category_from_api::CategoryFromApi;
use crate::{
    domain::{
        entities::category::Category, repositories::category_repository::ICategoryRepository,
    },
    infrastructure::{api::ApiClient, kv::KvClient},
};
use std::io::Result;
use worker::{async_trait::async_trait, console_log, kv::KvStore};

#[derive(Clone)]
pub struct CategoryRepository<'a> {
    api_url: String,
    store: &'a KvStore,
}
impl<'a> CategoryRepository<'a> {
    pub fn new(api_url: String, store: &'a KvStore) -> Self {
        Self { api_url, store }
    }
}
#[async_trait(?Send)]
impl<'a> ICategoryRepository for CategoryRepository<'a> {
    async fn find_all(&self) -> Result<Vec<Category>> {
        let cache_key = "categories".to_string();
        let kv_client = KvClient::new(&self.store);
        let kv_data = kv_client.get(&cache_key).await;
        let categories_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<CategoryFromApi>>().await.unwrap(),
            None => {
                let per_page = "100".to_string();
                let url = format!("{}/categories?per_page={}", self.api_url, per_page);
                let api = ApiClient::new();
                let _categories = match api.get::<Vec<CategoryFromApi>>(&url).await {
                    Ok(categories) => categories,
                    Err(_) => Vec::new(),
                };
                let data: Vec<u8> = serde_json::to_vec(&_categories)?;
                let categories_string = String::from_utf8(data.clone()).unwrap();
                match kv_client.put(&cache_key, categories_string).await {
                    Ok(_) => console_log!("{} cache saved", cache_key),
                    Err(_) => console_log!("{} cache saved failed", cache_key),
                };
                _categories
            }
        };
        let categories = categories_from_api
            .iter()
            .map(|category_from_api| {
                Category::reconstruct(category_from_api.id, category_from_api.name.clone())
            })
            .collect::<Vec<Category>>();
        Ok(categories)
    }
}
