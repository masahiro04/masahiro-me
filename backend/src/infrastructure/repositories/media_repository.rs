use crate::domain::repositories::media_repository::IMediaRepository;
use crate::infrastructure::api::ApiClient;
use crate::{domain::entities::media::Media, infrastructure::kv::KvClient};
use serde::{Deserialize, Serialize};
use std::io::Result;
use worker::{async_trait::async_trait, Env};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MediaFromApi {
    pub source_url: String,
    pub alt_text: String,
}

#[derive(Clone)]
pub struct MediaRepository<'a> {
    api_url: String,
    env: &'a Env,
}

impl<'a> MediaRepository<'a> {
    pub fn new(api_url: String, env: &'a Env) -> Self {
        Self { api_url, env }
    }
}

#[async_trait(?Send)]
impl<'a> IMediaRepository for MediaRepository<'a> {
    async fn find_one(&self, id: &str) -> Result<Media> {
        let cache_key = format!("media/{}", id);
        let kv_client = KvClient::new(&self.env);
        let kv_data = kv_client.get(&cache_key).await;
        let media_from_api = match kv_data {
            Some(mut data) => data.json::<MediaFromApi>().await.unwrap(),
            None => {
                let url = format!("{}/media/{}", self.api_url, id);
                let api = ApiClient::new();
                match api.get::<MediaFromApi>(&url).await {
                    Ok(media) => media,
                    Err(_) => MediaFromApi {
                        source_url: "".to_string(),
                        alt_text: "".to_string(),
                    },
                }
            }
        };
        let media = Media::new(media_from_api.source_url, media_from_api.alt_text).unwrap();
        Ok(media)
    }
}
