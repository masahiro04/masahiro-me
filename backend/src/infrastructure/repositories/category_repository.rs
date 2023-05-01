use crate::{
    domain::{
        entities::category::Category, repositories::category_repository::ICategoryRepository,
    },
    infrastructure::{api::ApiClient, kv::KvClient},
};
use serde::{Deserialize, Serialize};
use std::io::Result;
use worker::{async_trait::async_trait, Env};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CategoryFromApi {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct CategoryRepository<'a> {
    api_url: String,
    env: &'a Env,
}

impl<'a> CategoryRepository<'a> {
    pub fn new(api_url: String, env: &'a Env) -> Self {
        Self { api_url, env }
    }
}

#[async_trait(?Send)]
impl<'a> ICategoryRepository for CategoryRepository<'a> {
    async fn find_all(&self) -> Result<Vec<Category>> {
        let cache_key = "categories".to_string();
        let kv_client = KvClient::new(&self.env);
        let kv_data = kv_client.get(&cache_key).await;
        let categories_from_api = match kv_data {
            Some(mut data) => data.json::<Vec<CategoryFromApi>>().await.unwrap(),
            None => {
                let per_page = "100".to_string();
                let url = format!("{}/categories?per_page={}", self.api_url, per_page);
                let api = ApiClient::new();
                match api.get::<Vec<CategoryFromApi>>(&url).await {
                    Ok(categories) => categories,
                    Err(_) => Vec::new(),
                }
            }
        };
        let categories = categories_from_api
            .iter()
            .map(|category_from_api| {
                Category::new(category_from_api.id, category_from_api.name.clone()).unwrap()
            })
            .collect::<Vec<Category>>();
        Ok(categories)
    }
}
