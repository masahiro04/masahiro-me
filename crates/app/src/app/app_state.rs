use std::sync::Arc;

use infrastructure::repositories::{
    post_repository::PostRepository as PostRepositoryImpl,
    project_repository::ProjectRepository as ProjectRepositoryImpl,
};
use use_case::port::{
    post_repository::{PostRepository, ProvidePostRepository},
    project_repository::{ProjectRepository as ProjectRepositoryPort, ProvideProjectRepository},
};
use use_case::{
    fetch_advisory_projects_usecase::{FetchAdvisoryProjectsUseCase, ProvideFetchAdvisoryProjectsUseCase},
    fetch_past_work_projects_usecase::{FetchPastWorkProjectsUseCase, ProvideFetchPastWorkProjectsUseCase},
    fetch_post_usecase::{FetchPostUseCase, ProvideFetchPostUseCase},
    fetch_posts_usecase::{FetchPostsUseCase, ProvideFetchPostsUseCase},
    fetch_related_posts_usecase::{FetchRelatedPostsUseCase, ProvideFetchRelatedPostsUseCase},
    fetch_work_projects_usecase::{FetchWorkProjectsUseCase, ProvideFetchWorkProjectsUseCase},
};

use super::config::Config;

#[derive(Clone)]
pub struct AppState {
    post_repository: Arc<dyn PostRepository + Send + Sync>,
    project_repository: Arc<dyn ProjectRepositoryPort + Send + Sync>,
}

impl AppState {
    pub fn new(
        post_repository: Arc<dyn PostRepository + Send + Sync>,
        project_repository: Arc<dyn ProjectRepositoryPort + Send + Sync>,
    ) -> Self {
        Self {
            post_repository,
            project_repository,
        }
    }

    pub fn from_config() -> Self {
        let api_url = Config::api_url();
        let client = reqwest::Client::new();

        let post_repository = Arc::new(PostRepositoryImpl::new(api_url, client));
        let project_repository = Arc::new(ProjectRepositoryImpl::new());

        Self::new(post_repository, project_repository)
    }
}

// Repository Provider implementations
impl ProvidePostRepository for AppState {
    fn post_repository(&self) -> Arc<dyn PostRepository + Send + Sync> {
        self.post_repository.clone()
    }
}

impl ProvideProjectRepository for AppState {
    fn project_repository(&self) -> Arc<dyn ProjectRepositoryPort + Send + Sync> {
        self.project_repository.clone()
    }
}

// Use Case implementations
#[async_trait::async_trait(?Send)]
impl FetchPostsUseCase for AppState {}

#[async_trait::async_trait(?Send)]
impl FetchPostUseCase for AppState {}

#[async_trait::async_trait(?Send)]
impl FetchRelatedPostsUseCase for AppState {}

impl FetchWorkProjectsUseCase for AppState {}

impl FetchPastWorkProjectsUseCase for AppState {}

impl FetchAdvisoryProjectsUseCase for AppState {}

// Provide Use Case implementations
impl ProvideFetchPostsUseCase for AppState {
    type FetchPostsUseCase = Self;
    fn fetch_posts_usecase(&self) -> &Self::FetchPostsUseCase {
        self
    }
}

impl ProvideFetchPostUseCase for AppState {
    type FetchPostUseCase = Self;
    fn fetch_post_usecase(&self) -> &Self::FetchPostUseCase {
        self
    }
}

impl ProvideFetchRelatedPostsUseCase for AppState {
    type FetchRelatedPostsUseCase = Self;
    fn fetch_related_posts_usecase(&self) -> &Self::FetchRelatedPostsUseCase {
        self
    }
}

impl ProvideFetchWorkProjectsUseCase for AppState {
    type FetchWorkProjectsUseCase = Self;
    fn fetch_work_projects_usecase(&self) -> &Self::FetchWorkProjectsUseCase {
        self
    }
}

impl ProvideFetchPastWorkProjectsUseCase for AppState {
    type FetchPastWorkProjectsUseCase = Self;
    fn fetch_past_work_projects_usecase(&self) -> &Self::FetchPastWorkProjectsUseCase {
        self
    }
}

impl ProvideFetchAdvisoryProjectsUseCase for AppState {
    type FetchAdvisoryProjectsUseCase = Self;
    fn fetch_advisory_projects_usecase(&self) -> &Self::FetchAdvisoryProjectsUseCase {
        self
    }
}
