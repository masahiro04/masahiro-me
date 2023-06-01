use crate::domain::entities::project::Project;
use std::io::Result;

pub trait IProjectRepository {
    fn find_all(&self) -> Result<Vec<Project>>;
    fn find_work_projects(&self) -> Result<Vec<Project>>;
    fn find_past_work_projects(&self) -> Result<Vec<Project>>;
    fn find_advisory_projects(&self) -> Result<Vec<Project>>;
}
