use crate::domain::entities::project::Project;
use crate::domain::repositories::project_repository::IProjectRepository;
use crate::infrastructure::repositories::project_repository::ProjectRepository;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchAdvisoryProjectsUsecase<Repo>
where
    Repo: IProjectRepository,
{
    repo: Repo,
}
impl FetchAdvisoryProjectsUsecase<ProjectRepository> {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }
    pub fn execute(&self) -> Result<Vec<Project>> {
        match self.repo.find_advisory_projects() {
            Ok(projects) => Ok(projects),
            Err(_) => Ok(Vec::new()),
        }
    }
}
