use crate::app::app_state::AppState;
use domain::entities::{post::Post, project::Project};
use use_case::port::{post_repository, project_repository};
use use_case::{
    fetch_advisory_projects_usecase::FetchAdvisoryProjectsUseCase,
    fetch_past_work_projects_usecase::FetchPastWorkProjectsUseCase,
    fetch_post_usecase::FetchPostUseCase,
    fetch_posts_usecase::FetchPostsUseCase,
    fetch_related_posts_usecase::FetchRelatedPostsUseCase,
    fetch_work_projects_usecase::FetchWorkProjectsUseCase,
};

pub async fn fetch_posts_usecase(
    per_page: i32,
    offset: i32,
) -> post_repository::Result<Vec<Post>> {
    let app_state = AppState::from_config();
    app_state.fetch_posts(per_page, offset).await
}

pub async fn fetch_related_posts_usecase(
    category_ids: &str,
) -> post_repository::Result<Vec<Post>> {
    let app_state = AppState::from_config();
    app_state.fetch_related_posts(category_ids).await
}

pub async fn fetch_post_usecase(slug: String) -> post_repository::Result<Option<Post>> {
    let app_state = AppState::from_config();
    app_state.fetch_post(slug).await
}

pub fn fetch_work_projects_usecase() -> project_repository::Result<Vec<Project>> {
    let app_state = AppState::from_config();
    app_state.fetch_work_projects()
}

pub fn fetch_past_work_projects_usecase() -> project_repository::Result<Vec<Project>> {
    let app_state = AppState::from_config();
    app_state.fetch_past_work_projects()
}

pub fn fetch_advisory_projects_usecase() -> project_repository::Result<Vec<Project>> {
    let app_state = AppState::from_config();
    app_state.fetch_advisory_projects()
}
