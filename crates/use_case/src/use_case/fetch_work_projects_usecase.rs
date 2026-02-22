use crate::port::project_repository::ProvideProjectRepository;
use domain::entities::project::Project;

pub trait FetchWorkProjectsUseCase: ProvideProjectRepository {
    fn fetch_work_projects(&self) -> crate::port::project_repository::Result<Vec<Project>> {
        self.project_repository().find_work_projects()
    }
}

pub trait ProvideFetchWorkProjectsUseCase {
    type FetchWorkProjectsUseCase: FetchWorkProjectsUseCase + Send + Sync;
    fn fetch_work_projects_usecase(&self) -> &Self::FetchWorkProjectsUseCase;
}
