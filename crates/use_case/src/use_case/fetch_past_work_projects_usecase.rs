use domain::entities::project::Project;
use domain::repositories::project_repository::IProjectRepository;
use infrastructure::repositories::project_repository::ProjectRepository;
use std::io::Result;

#[derive(Clone, Debug)]
pub struct FetchPastWorkProjectsUsecase<Repo>
where
    Repo: IProjectRepository,
{
    repo: Repo,
}
impl FetchPastWorkProjectsUsecase<ProjectRepository> {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }
    pub fn execute(&self) -> Result<Vec<Project>> {
        match self.repo.find_past_work_projects() {
            Ok(projects) => Ok(projects),
            Err(_) => Ok(Vec::new()),
        }
    }
}
