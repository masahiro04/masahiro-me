use crate::port::post_repository::ProvidePostRepository;
use domain::entities::post::Post;

#[async_trait::async_trait(?Send)]
pub trait FetchPostsUseCase: ProvidePostRepository {
    async fn fetch_posts(&self, per_page: i32, offset: i32) -> crate::port::post_repository::Result<Vec<Post>> {
        self.post_repository().find_posts(per_page, offset).await
    }
}

pub trait ProvideFetchPostsUseCase {
    type FetchPostsUseCase: FetchPostsUseCase + Send + Sync;
    fn fetch_posts_usecase(&self) -> &Self::FetchPostsUseCase;
}
