use std::io::Result;

use crate::domain::{
    entities::project::{Project, ProjectKind},
    repositories::project_repository::IProjectRepository,
};

#[derive(Clone)]
pub struct ProjectRepository {}

impl ProjectRepository {
    pub fn new() -> Self {
        ProjectRepository {}
    }
}

impl IProjectRepository for ProjectRepository {
    fn find_all(&self) -> Result<Vec<Project>> {
        let projects = vec![
            Project::new(
                "Doctormate".to_string(),
                "Rust / TypeScript / axum / Next.js / NestJS / React Native / Expo / GCP / Firebase".to_string(),
                "https://doctormate.co.jp/".to_string(),
                ProjectKind::Work,
            )
            .unwrap(),
            Project::new(
                "Seibii".to_string(),
                "TypeScript / Dart / Ruby / Terraform / React / Remix / Ruby on Rails / Flutter / AWS".to_string(),
                "https://seibii.co.jp/".to_string(),
                ProjectKind::PastWork,
            )
            .unwrap(),
            Project::new(
                "Cogane Studio".to_string(),
                "Management / Ruby / React / Ruby on Rails / Heroku / AWS".to_string(),
                "https://bentenmarket.com/".to_string(),
                ProjectKind::Advisory,
            )
            .unwrap(),
            Project::new(
                "Everyplus".to_string(),
                "Management / Ruby / React /Ruby on Rails / Heroku / AWS".to_string(),
                "https://recreation.everyplus.jp/".to_string(),
                ProjectKind::Advisory,
            )
            .unwrap(),
            Project::new(
                "Flucle".to_string(),
                "Golang / TypeScript / Terraform / Gin / React / Next.js / Heroku / AWS"
                    .to_string(),
                "https://hrbase.jp/".to_string(),
                ProjectKind::PastWork,
            )
            .unwrap(),
        ];
        Ok(projects)
    }
    fn find_work_projects(&self) -> Result<Vec<Project>> {
        let projects = self
            .find_all()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::Work)
            .collect::<Vec<Project>>();
        Ok(projects)
    }
    fn find_past_work_projects(&self) -> Result<Vec<Project>> {
        let projects = self
            .find_all()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::PastWork)
            .collect();
        Ok(projects)
    }
    fn find_advisory_projects(&self) -> Result<Vec<Project>> {
        let projects = self
            .find_all()
            .unwrap()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::Advisory)
            .collect();
        Ok(projects)
    }
}
