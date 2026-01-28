/// Configuration loaded at compile time from environment variables
pub struct Config;

impl Config {
    /// API base URL - must be set via API_URL environment variable at compile time
    /// Returns the full API URL with /api/v1 path
    pub fn api_url() -> String {
        let base_url = env!("API_URL");
        format!("{}/api/v1", base_url)
    }
}
