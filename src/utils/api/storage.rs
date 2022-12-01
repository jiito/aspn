// These methods will be used to call the Pando CRUD api endpoints

pub mod function {
    use actix_web::{http, web::method};
    use anyhow::Result;

    use crate::utils::api::{self, models};

    pub fn save(new_function: models::NewFunction) -> Result<()> {
        Ok(())
    }
    pub async fn find(id: &i32) -> Result<models::Function> {
        let path = format!("/function/{id}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
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

    // TODO: Move to config!!
    pub fn save_function_connection(host_id: &i32, function_id: &i32) -> Result<()> {
        Ok(())
    }

    pub fn online() -> Result<()> {
        // used for marking a host online
        let host = current_host()?;
        todo!("Implement online/offline wrappers")
    }
    pub fn current_host() -> Result<models::Host> {
        todo!("Implement find currrent host using pando api")
    }

    pub fn offline() -> Result<()> {
        // to be called when a host kills their program
        let host = current_host()?;
        todo!("Implement online/offline wrappers")
    }
}
