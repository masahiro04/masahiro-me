#[derive(Clone, Debug, PartialEq)]
pub enum ProjectKind {
    Work,
    Advisory,
    PastWork,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    name: String,
    technologies: String,
    url: String,
    kind: ProjectKind,
}

impl Project {
    pub fn reconstruct(name: String, technologies: String, url: String, kind: ProjectKind) -> Self {
        Self {
            name,
            technologies,
            url,
            kind,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn technologies(&self) -> &str {
        &self.technologies
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn kind(&self) -> &ProjectKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entities::project::{Project, ProjectKind};

    #[test]
    fn test() -> anyhow::Result<()> {
        let name = "name".to_string();
        let technologies = "technologies".to_string();
        let url = "url".to_string();
        let kind = ProjectKind::Work;

        // reconstruct
        let project = Project::reconstruct(
            name.clone(),
            technologies.clone(),
            url.clone(),
            kind.clone(),
        );

        assert_eq!(project.name(), &name);
        assert_eq!(project.technologies(), &technologies);
        assert_eq!(project.url(), &url);
        assert_eq!(project.kind(), &kind);
        Ok(())
    }
}
