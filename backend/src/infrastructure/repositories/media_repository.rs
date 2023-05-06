use crate::domain::repositories::media_repository::IMediaRepository;
use crate::infrastructure::api::ApiClient;
use crate::infrastructure::repositories::types::media_from_api::MediaFromApi;
use crate::{domain::entities::media::Media, infrastructure::kv::KvClient};
use std::io::Result;
use worker::async_trait::async_trait;
use worker::console_log;
use worker::kv::KvStore;

#[derive(Clone)]
pub struct MediaRepository<'a> {
    api_url: String,
    store: &'a KvStore,
}
impl<'a> MediaRepository<'a> {
    pub fn new(api_url: String, store: &'a KvStore) -> Self {
        Self { api_url, store }
    }
}
#[async_trait(?Send)]
impl<'a> IMediaRepository for MediaRepository<'a> {
    async fn find_one(&self, id: &str) -> Result<Media> {
        let cache_key = format!("media/{}", id);
        let kv_client = KvClient::new(self.store);
        let kv_data = kv_client.get(&cache_key).await;
        let media_from_api = match kv_data {
            Some(mut data) => data.json::<MediaFromApi>().await.unwrap(),
            None => {
                let url = format!("{}/media/{}", self.api_url, id);
                let api = ApiClient::new();
                let _media = match api.get::<MediaFromApi>(&url).await {
                    Ok(media) => media,
                    Err(_) => MediaFromApi {
                        source_url: "".to_string(),
                        alt_text: "".to_string(),
                    },
                };
                let data: Vec<u8> = serde_json::to_vec(&_media)?;
                let media_string = String::from_utf8(data.clone()).unwrap();
                match kv_client.put(&cache_key, media_string).await {
                    Ok(_) => console_log!("{} cache saved", cache_key),
                    Err(_) => console_log!("{} cache saved failed", cache_key),
                };
                _media
            }
        };
        let media = Media::reconstruct(media_from_api.source_url, media_from_api.alt_text);
        Ok(media)
    }
}
