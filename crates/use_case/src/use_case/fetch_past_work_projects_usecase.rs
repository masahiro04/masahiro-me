use crate::port::project_repository::ProvideProjectRepository;
use domain::entities::project::Project;

pub trait FetchPastWorkProjectsUseCase: ProvideProjectRepository {
    fn fetch_past_work_projects(&self) -> crate::port::project_repository::Result<Vec<Project>> {
        self.project_repository().find_past_work_projects()
    }
}

pub trait ProvideFetchPastWorkProjectsUseCase {
    type FetchPastWorkProjectsUseCase: FetchPastWorkProjectsUseCase + Send + Sync;
    fn fetch_past_work_projects_usecase(&self) -> &Self::FetchPastWorkProjectsUseCase;
}
