use domain::entities::project::Project;
use domain::repositories::project_repository::{ProjectRepositoryInterface, WithProjectRepository};

pub trait FetchPastWorkProjectsUsecase: WithProjectRepository {
    fn execute(&self) -> Vec<Project> {
        self.project_repository().find_past_works()
    }
}
pub trait HasFetchPastWorkProjectsUsecase {
    type FetchPastWorkProjectsUsecase: FetchPastWorkProjectsUsecase;
    fn find_past_works(&self) -> &Self::FetchPastWorkProjectsUsecase;
}
