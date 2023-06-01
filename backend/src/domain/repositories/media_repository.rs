use crate::domain::entities::media::Media;
use async_trait::async_trait;
use std::io::Result;

// NOTE: https://github.com/rustwasm/wasm-bindgen/issues/2409#issuecomment-754574965
#[async_trait(?Send)]
pub trait IMediaRepository {
    async fn find_one(&self, id: &str) -> Result<Media>;
}
