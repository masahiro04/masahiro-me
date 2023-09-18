use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
    pub fn reconstruct(id: i32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
