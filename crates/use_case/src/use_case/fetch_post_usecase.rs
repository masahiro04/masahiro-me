use domain::{entities::post::Post, repositories::post_repository::PostRepositoryInterface};
use infrastructure::repositories::post_repository::PostRepository;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchPostUsecase<Repo>
where
    Repo: PostRepositoryInterface,
{
    repo: Repo,
}
impl FetchPostUsecase<PostRepository> {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, slug: String) -> Result<Option<Post>> {
        match self.repo.find_one(slug).await {
            Ok(post) => Ok(post),
            Err(_) => Ok(None),
        }
    }
}
