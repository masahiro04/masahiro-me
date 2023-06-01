use crate::{
    domain::repositories::post_repository::IPostRepository,
    infrastructure::repositories::post_repository::PostRepository,
};
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchSlugsUsecase<Repo>
where
    Repo: IPostRepository,
{
    repo: Repo,
}

impl<'a> FetchSlugsUsecase<PostRepository<'a>> {
    pub fn new(repo: PostRepository<'a>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self) -> Result<Vec<String>> {
        match self.repo.find_slugs().await {
            Ok(categories) => Ok(categories),
            Err(_) => Ok(Vec::new()),
        }
    }
}
