use domain::repositories::post_repository::WithPostRepository;
use domain::{entities::post::Post, repositories::post_repository::PostRepositoryInterface};

#[async_trait::async_trait(?Send)]
pub trait FetchPostUsecase: WithPostRepository {
    async fn execute(&self, slug: String) -> anyhow::Result<Option<Post>> {
        self.post_repository().find_one(slug).await
    }
}
#[async_trait::async_trait(?Send)]
pub trait HasFetchPostUsecase {
    type FetchPostUsecase: FetchPostUsecase + Sync;
    async fn fetch_one(&self, slug: String) -> &Self::FetchPostUsecase;
}
