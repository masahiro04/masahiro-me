use std::sync::Arc;

use domain::entities::project::Project;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Project not found")]
    NotFound,
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ProjectRepository {
    fn find_all(&self) -> Result<Vec<Project>>;
    fn find_work_projects(&self) -> Result<Vec<Project>>;
    fn find_past_work_projects(&self) -> Result<Vec<Project>>;
    fn find_advisory_projects(&self) -> Result<Vec<Project>>;
}

pub trait ProvideProjectRepository {
    fn project_repository(&self) -> Arc<dyn ProjectRepository + Send + Sync>;
}
