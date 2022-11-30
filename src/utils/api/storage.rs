// These methods will be used to call the Pando CRUD api endpoints

pub mod function {
    use anyhow::Result;

    use crate::utils::api::models;

    pub fn save(new_function: models::NewFunction) -> Result<()> {
        Ok(())
    }
    pub fn find(id: &i32) -> Result<models::Function> {
        todo!("Add find for functions")
    }
    pub fn find_by_project(project_id: &i32) -> Result<models::Function> {
        todo!("Add find by project for funcitons")
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

    use crate::utils::api::models;

    pub fn save(new_host: models::NewHost) -> Result<()> {
        Ok(())
    }
    pub fn find(id: &i32) -> Result<models::Host> {
        todo!("Add find for <Host>")
    }
    pub fn find_by_token(token: &str) -> Result<models::Host> {
        todo!("Add find by token for <Host>")
    }
    pub fn save_function_connection(host_id: &i32, function_id: &i32) -> Result<()> {
        Ok(())
    }
}
