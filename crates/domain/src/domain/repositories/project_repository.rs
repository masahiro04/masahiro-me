use crate::domain::entities::project::Project;

pub trait ProjectRepositoryInterface {
    fn find_all(&self) -> Vec<Project>;
    fn find_works(&self) -> Vec<Project>;
    fn find_past_works(&self) -> Vec<Project>;
    fn find_advisories(&self) -> Vec<Project>;
}

pub trait WithProjectRepository {
    type ProjectRepository: ProjectRepositoryInterface;
    fn project_repository(&self) -> &Self::ProjectRepository;
}
