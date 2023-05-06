use std::io::Result;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
    pub fn new(id: i32, name: String) -> Result<Self> {
        Ok(Self { id, name })
    }
    pub fn reconstruct(id: i32, name: String) -> Self {
        Self { id, name }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
