use crate::domain::entities::project::Project;

pub trait IProjectRepository {
    fn find_all(&self) -> Vec<Project>;
    fn find_work_projects(&self) -> Vec<Project>;
    fn find_past_work_projects(&self) -> Vec<Project>;
    fn find_advisory_projects(&self) -> Vec<Project>;
}
