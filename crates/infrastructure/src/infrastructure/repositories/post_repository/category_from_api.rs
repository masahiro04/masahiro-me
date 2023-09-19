use domain::entities::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryFromApi {
    pub id: i32,
    pub name: String,
}

impl CategoryFromApi {
    pub fn into_category(&self) -> anyhow::Result<Category> {
        Ok(Category::reconstruct(self.id, self.name.clone()))
    }
}
