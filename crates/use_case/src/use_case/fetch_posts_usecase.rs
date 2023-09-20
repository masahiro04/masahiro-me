use domain::repositories::post_repository::WithPostRepository;
use domain::{entities::post::Post, repositories::post_repository::PostRepositoryInterface};

#[async_trait::async_trait(?Send)]
pub trait FetchPostsUsecase: WithPostRepository {
    async fn execute(&self, per_page: i32, offset: i32) -> anyhow::Result<Vec<Post>> {
        self.post_repository().find_posts(per_page, offset).await
    }
}
#[async_trait::async_trait(?Send)]
pub trait HasFetchPostsUsecase {
    type FetchPostsUsecase: FetchPostsUsecase + Sync;
    async fn fetch_posts_usecase(&self, per_page: i32, offset: i32) -> &Self::FetchPostsUsecase;
}
