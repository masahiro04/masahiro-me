use std::fmt::Error;
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
    pub fn new(
        name: String,
        technologies: String,
        url: String,
        kind: ProjectKind,
    ) -> Result<Self, Error> {
        Ok(Project {
            name,
            technologies,
            url,
            kind,
        })
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
