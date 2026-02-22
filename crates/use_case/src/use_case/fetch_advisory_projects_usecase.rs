use crate::port::project_repository::ProvideProjectRepository;
use domain::entities::project::Project;

pub trait FetchAdvisoryProjectsUseCase: ProvideProjectRepository {
    fn fetch_advisory_projects(&self) -> crate::port::project_repository::Result<Vec<Project>> {
        self.project_repository().find_advisory_projects()
    }
}

pub trait ProvideFetchAdvisoryProjectsUseCase {
    type FetchAdvisoryProjectsUseCase: FetchAdvisoryProjectsUseCase + Send + Sync;
    fn fetch_advisory_projects_usecase(&self) -> &Self::FetchAdvisoryProjectsUseCase;
}
