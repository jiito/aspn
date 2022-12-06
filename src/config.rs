pub mod dev;
pub mod host;
pub mod project;

pub mod env {

    pub fn database_url() -> String {
        std::env::var("DATABASE_URL").unwrap_or("postgresql://localhost:5432/pando".to_string())
    }
    pub fn api_url() -> String {
        std::env::var("API_URL").unwrap_or("localhost:8080".to_string())
    }
    pub fn degub() -> bool {
        api_url().eq("localhost:8080")
    }
}
