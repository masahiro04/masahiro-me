use crate::domain::entities::category::Category;
use async_trait::async_trait;
use std::io::Result;

// NOTE: https://github.com/rustwasm/wasm-bindgen/issues/2409#issuecomment-754574965
#[async_trait(?Send)]
pub trait ICategoryRepository {
    async fn find_all(&self) -> Result<Vec<Category>>;
}
