use domain::{
    entities::project::{Project, ProjectKind},
    repositories::project_repository::ProjectRepositoryInterface,
};

#[derive(Clone)]
pub struct ProjectRepository {}

impl ProjectRepository {
    pub fn new() -> Self {
        ProjectRepository {}
    }
}
impl Default for ProjectRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectRepositoryInterface for ProjectRepository {
    fn find_all(&self) -> Vec<Project> {
        vec![
            Project::reconstruct(
                "Doctormate".to_string(),
                "Rust / TypeScript / axum / Next.js / NestJS / React Native / Expo / GCP / Firebase".to_string(),
                "https://doctormate.co.jp/".to_string(),
                ProjectKind::Work,
            ),
            Project::reconstruct(
                "Seibii".to_string(),
                "TypeScript / Dart / Ruby / Terraform / React / Remix / Ruby on Rails / Flutter / AWS".to_string(),
                "https://seibii.co.jp/".to_string(),
                ProjectKind::PastWork,
            ),
            Project::reconstruct(
                "Cogane Studio".to_string(),
                "Management / Ruby / React / Ruby on Rails / Heroku / AWS".to_string(),
                "https://bentenmarket.com/".to_string(),
                ProjectKind::Advisory,
            ),
            Project::reconstruct(
                "Everyplus".to_string(),
                "Management / Rust / Ruby / TypeScript / React /Ruby on Rails / Heroku / AWS".to_string(),
                "https://recreation.everyplus.jp/".to_string(),
                ProjectKind::Advisory,
            ),
            Project::reconstruct(
                "Flucle".to_string(),
                "Golang / TypeScript / Terraform / Gin / React / Next.js / Heroku / AWS"
                    .to_string(),
                "https://hrbase.jp/".to_string(),
                ProjectKind::PastWork,
            )
        ]
    }
    fn find_works(&self) -> Vec<Project> {
        self.find_all()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::Work)
            .collect::<Vec<Project>>()
    }
    fn find_past_works(&self) -> Vec<Project> {
        self.find_all()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::PastWork)
            .collect()
    }
    fn find_advisories(&self) -> Vec<Project> {
        self.find_all()
            .clone()
            .into_iter()
            .filter(|project| *project.kind() == ProjectKind::Advisory)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectRepository;
    use domain::repositories::project_repository::ProjectRepositoryInterface;

    #[test]
    fn test_find_all() -> anyhow::Result<()> {
        let repo = ProjectRepository::new();
        let projects = repo.find_all();
        assert_eq!(projects.len(), 5);
        Ok(())
    }

    #[test]
    fn test_find_works() -> anyhow::Result<()> {
        let repo = ProjectRepository::new();
        let projects = repo.find_works();
        assert_eq!(projects.len(), 1);
        Ok(())
    }

    #[test]
    fn test_find_past_works() -> anyhow::Result<()> {
        let repo = ProjectRepository::new();
        let projects = repo.find_past_works();
        assert_eq!(projects.len(), 2);
        Ok(())
    }

    #[test]
    fn test_find_advisories() -> anyhow::Result<()> {
        let repo = ProjectRepository::new();
        let projects = repo.find_advisories();
        assert_eq!(projects.len(), 2);
        Ok(())
    }
}
