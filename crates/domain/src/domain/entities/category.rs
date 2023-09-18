use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
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

#[cfg(test)]
mod tests {
    use crate::domain::entities::category::Category;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = 1;
        let name = "category".to_string();

        // reconstruct
        let category = Category::reconstruct(id, name.clone());
        assert_eq!(category.id(), &id);
        assert_eq!(category.name(), name);
        Ok(())
    }
}
