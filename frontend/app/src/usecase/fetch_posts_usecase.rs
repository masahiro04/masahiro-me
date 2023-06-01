use crate::domain::{entities::post::Post, repositories::post_repository::IPostRepository};
use crate::infrastructure::repositories::post_repository::PostRepository;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchPostsUsecase<Repo>
where
    Repo: IPostRepository,
{
    repo: Repo,
}
impl FetchPostsUsecase<PostRepository> {
    pub fn new(repo: PostRepository) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, per_page: i32, offset: i32) -> Result<Vec<Post>> {
        match self.repo.find_posts(per_page, offset).await {
            Ok(posts) => Ok(posts),
            Err(_) => Ok(Vec::new()),
        }
    }
}
