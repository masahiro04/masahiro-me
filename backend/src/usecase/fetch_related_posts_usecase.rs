use crate::{
    domain::{entities::post::Post, repositories::post_repository::IPostRepository},
    infrastructure::repositories::post_repository::PostRepository,
};
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchRelatedPostsUsecase<Repo>
where
    Repo: IPostRepository,
{
    repo: Repo,
}

impl<'a> FetchRelatedPostsUsecase<PostRepository<'a>> {
    pub fn new(repo: PostRepository<'a>) -> Self {
        Self { repo }
    }
    pub async fn execute(&self, category_ids: &str) -> Result<Vec<Post>> {
        match self.repo.find_related(category_ids).await {
            Ok(posts) => Ok(posts),
            Err(_) => Ok(Vec::new()),
        }
    }
}
