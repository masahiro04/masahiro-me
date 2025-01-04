use std::sync::Arc;

use crate::domain::entities::post::Post;

// NOTE: https://github.com/rustwasm/wasm-bindgen/issues/2409#issuecomment-754574965
#[async_trait::async_trait(?Send)]
pub trait PostRepositoryInterface {
    async fn find_posts(&self, per_page: i32, offset: i32) -> anyhow::Result<Vec<Post>>;
    async fn find_post(&self, slug: String) -> anyhow::Result<Option<Post>>;
    async fn find_related_posts(&self, category_ids: &str) -> anyhow::Result<Vec<Post>>;
}

//pub trait HasCloudStorage {
//    fn cloud_storage(&self) -> Arc<dyn CloudStorage + Send + Sync>;
//}

#[async_trait::async_trait(?Send)]
pub trait WithPostRepository {
    //type PostRepository: PostRepositoryInterface + Sync;
    fn post_repository(&self) -> Arc<dyn PostRepositoryInterface + Sync>;
}
