pub mod dev;
pub mod host;
pub mod project;

pub mod env {

    pub fn api_url() -> String {
        std::env::var("API_URL").unwrap_or("http://localhost:8080".to_string())
    }
    pub fn debug() -> bool {
        api_url().eq("http://localhost:8080")
    }
}
