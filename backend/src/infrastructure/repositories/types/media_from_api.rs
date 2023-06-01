use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MediaFromApi {
    pub source_url: String,
    pub alt_text: String,
}
