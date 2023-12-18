use domain::repositories::post_repository::WithPostRepository;
use domain::{entities::post::Post, repositories::post_repository::PostRepositoryInterface};

#[async_trait::async_trait(?Send)]
pub trait FetchPostsByCategoryIdsUsecase: WithPostRepository {
    async fn execute(&self, category_ids: &str) -> anyhow::Result<Vec<Post>> {
        self.post_repository()
            .find_by_category_ids(category_ids)
            .await
    }
}
#[async_trait::async_trait(?Send)]
pub trait HasFetchPostsByCategoryIdsUsecase {
    type FetchPostsByCategoryIdsUsecase: FetchPostsByCategoryIdsUsecase + Sync;
    async fn fetch_posts_by_category_ids(
        &self,
        category_ids: &str,
    ) -> &Self::FetchPostsByCategoryIdsUsecase;
}
