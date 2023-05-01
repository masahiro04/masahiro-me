use crate::{
    domain::{entities::post::Post, repositories::post_repository::IPostRepository},
    infrastructure::repositories::post_repository::PostRepository,
};
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchPostUsecase<Repo>
where
    Repo: IPostRepository,
{
    repo: Repo,
}

impl<'a> FetchPostUsecase<PostRepository<'a>> {
    pub fn new(repo: PostRepository<'a>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, slug: &str) -> Result<Option<Post>> {
        self.repo.find_one(slug).await
    }
}
