use crate::port::post_repository::ProvidePostRepository;
use domain::entities::post::Post;

#[async_trait::async_trait(?Send)]
pub trait FetchPostUseCase: ProvidePostRepository {
    async fn fetch_post(&self, slug: String) -> crate::port::post_repository::Result<Option<Post>> {
        self.post_repository().find_post(slug).await
    }
}

pub trait ProvideFetchPostUseCase {
    type FetchPostUseCase: FetchPostUseCase + Send + Sync;
    fn fetch_post_usecase(&self) -> &Self::FetchPostUseCase;
}
