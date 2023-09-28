use domain::{entities::post::Post, repositories::post_repository::PostRepositoryInterface};
use infrastructure::repositories::post_repository::PostRepository;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchRelatedPostsUsecase<Repo>
where
    Repo: PostRepositoryInterface,
{
    repo: Repo,
}
impl FetchRelatedPostsUsecase<PostRepository> {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, category_ids: &str) -> Result<Vec<Post>> {
        match self.repo.find_related_posts(category_ids).await {
            Ok(posts) => Ok(posts),
            Err(_) => Ok(Vec::new()),
        }
    }
}
