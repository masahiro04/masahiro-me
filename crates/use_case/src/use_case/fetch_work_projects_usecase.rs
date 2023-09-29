use domain::entities::project::Project;
use domain::repositories::project_repository::IProjectRepository;
use infrastructure::repositories::project_repository::ProjectRepository;

#[derive(Clone, Debug)]
pub struct FetchWorkProjectsUsecase<Repo>
where
    Repo: IProjectRepository,
{
    repo: Repo,
}
impl FetchWorkProjectsUsecase<ProjectRepository> {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }
    pub fn execute(&self) -> Vec<Project> {
        self.repo.find_works()
    }
}
