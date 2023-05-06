use serde::{Deserialize, Serialize};
use std::io::Result;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Media {
    source_url: String,
    alt_text: String,
}

impl Media {
    pub fn new(source_url: String, alt_text: String) -> Result<Self> {
        Ok(Self {
            source_url,
            alt_text,
        })
    }
    pub fn reconstruct(source_url: String, alt_text: String) -> Self {
        Self {
            source_url,
            alt_text,
        }
    }
    pub fn source_url(&self) -> &str {
        &self.source_url
    }
    pub fn alt_text(&self) -> &str {
        &self.alt_text
    }
}
