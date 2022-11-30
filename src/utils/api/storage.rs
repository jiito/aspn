// These methods will be used to call the Pando CRUD api endpoints

pub mod function {
    use anyhow::Result;

    pub fn save() -> Result<()> {
        Ok(())
    }
    pub fn find() -> Result<()> {
        Ok(())
    }
    pub fn find_by_project(project_id: &i32) -> Result<()> {
        Ok(())
    }
}
pub mod project {
    use anyhow::Result;

    use crate::utils::api::models;

    pub fn save() -> Result<()> {
        Ok(())
    }
    pub fn find() -> Result<models::Project> {
        todo!("Get the Project from the API")
    }
}
pub mod host {
    use anyhow::Result;

    pub fn save() -> Result<()> {
        Ok(())
    }
    pub fn find() -> Result<()> {
        Ok(())
    }
    pub fn find_by_token(token: &str) -> Result<()> {
        Ok(())
    }
    pub fn save_function_connection(host_id: &i32, function_id: &i32) -> Result<()> {
        Ok(())
    }
}
