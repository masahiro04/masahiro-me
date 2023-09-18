use super::{
    fetch_advisory_projects_usecase::FetchAdvisoryProjectsUsecase,
    fetch_past_work_projects_usecase::FetchPastWorkProjectsUsecase,
    fetch_post_usecase::FetchPostUsecase, fetch_posts_usecase::FetchPostsUsecase,
    fetch_related_posts_usecase::FetchRelatedPostsUsecase,
    fetch_work_projects_usecase::FetchWorkProjectsUsecase,
};
use std::io::Result;
use {
    domain::entities::{post::Post, project::Project},
    infrastructure::repositories::{
        post_repository::PostRepository, project_repository::ProjectRepository,
    },
};

pub async fn fetch_posts_usecase(per_page: i32, offset: i32) -> Result<Vec<Post>> {
    let api_url = format!("{}", "https://api.masahiro.me/api");
    let repo = PostRepository::new(api_url);
    let usecase = FetchPostsUsecase::new(repo);
    usecase.execute(per_page, offset).await
}
pub async fn fetch_related_posts_usecase(category_ids: &str) -> Result<Vec<Post>> {
    let api_url = format!("{}", "https://api.masahiro.me/api");
    let repo = PostRepository::new(api_url);
    let usecase = FetchRelatedPostsUsecase::new(repo);
    usecase.execute(category_ids).await
}
pub async fn fetch_post_usecase(slug: String) -> Result<Option<Post>> {
    let api_url = format!("{}", "https://api.masahiro.me/api");
    let repo = PostRepository::new(api_url);
    let usecase = FetchPostUsecase::new(repo);
    usecase.execute(slug).await
}
pub fn fetch_work_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchWorkProjectsUsecase::new(repo);
    usecase.execute()
}
pub fn fetch_past_work_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchPastWorkProjectsUsecase::new(repo);
    usecase.execute()
}
pub fn fetch_advisory_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchAdvisoryProjectsUsecase::new(repo);
    usecase.execute()
}
