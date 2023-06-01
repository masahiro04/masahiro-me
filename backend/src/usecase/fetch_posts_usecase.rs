use crate::{
    domain::{entities::post::Post, repositories::post_repository::IPostRepository},
    infrastructure::repositories::post_repository::PostRepository,
};
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchPostsUsecase<Repo>
where
    Repo: IPostRepository,
{
    repo: Repo,
}

impl<'a> FetchPostsUsecase<PostRepository<'a>> {
    pub fn new(repo: PostRepository<'a>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, per_page: &str, offset: &str) -> Result<Vec<Post>> {
        match self.repo.find_all(per_page, offset).await {
            Ok(categories) => Ok(categories),
            Err(_) => Ok(Vec::new()),
        }
    }
}
