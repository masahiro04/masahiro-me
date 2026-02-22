use crate::port::post_repository::ProvidePostRepository;
use domain::entities::post::Post;

#[async_trait::async_trait(?Send)]
pub trait FetchRelatedPostsUseCase: ProvidePostRepository {
    async fn fetch_related_posts(&self, category_ids: &str) -> crate::port::post_repository::Result<Vec<Post>> {
        self.post_repository().find_related_posts(category_ids).await
    }
}

pub trait ProvideFetchRelatedPostsUseCase {
    type FetchRelatedPostsUseCase: FetchRelatedPostsUseCase + Send + Sync;
    fn fetch_related_posts_usecase(&self) -> &Self::FetchRelatedPostsUseCase;
}
