use domain::entities::project::Project;
use domain::repositories::project_repository::{ProjectRepositoryInterface, WithProjectRepository};

pub trait FetchAdvisoryProjectsUsecase: WithProjectRepository {
    fn execute(&self) -> Vec<Project> {
        self.project_repository().find_all()
    }
}
pub trait HasFetchAdvisoryProjectsUsecase {
    type FetchAdvisoryProjectsUsecase: FetchAdvisoryProjectsUsecase;
    fn find_advisories(&self) -> &Self::FetchAdvisoryProjectsUsecase;
}
