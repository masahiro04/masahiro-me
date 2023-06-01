use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CategoryFromApi {
    pub id: i32,
    pub name: String,
}
