pub mod dev;
pub mod host;
pub mod project;

pub mod env {

    pub fn api_url() -> &'static str {
        option_env!("API_URL").unwrap_or("http://localhost:8080")
    }
    pub fn debug() -> bool {
        option_env!("ASPN_DEBUG").is_some()
    }
}
