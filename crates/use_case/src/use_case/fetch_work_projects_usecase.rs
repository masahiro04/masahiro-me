use domain::entities::project::Project;
use domain::repositories::project_repository::{ProjectRepositoryInterface, WithProjectRepository};

pub trait FetchWorkProjectsUsecase: WithProjectRepository {
    fn execute(&self) -> Vec<Project> {
        self.project_repository().find_advisories()
    }
}
pub trait HasFetchWorkProjectsUsecase {
    type FetchWorkProjectsUsecase: FetchWorkProjectsUsecase;
    fn find_works(&self) -> &Self::FetchWorkProjectsUsecase;
}
