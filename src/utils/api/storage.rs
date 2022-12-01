// These methods will be used to call the Pando CRUD api endpoints

pub mod function {
    use actix_web::{http, web::method};
    use anyhow::Result;

    use crate::utils::api::{self, models};

    pub async fn save(new_function: &models::NewFunction) -> Result<models::Function> {
        let path = format!("/function");
        let res = api::request(http::Method::POST, &path[..], &Some(new_function)).await;
        res
    }
    pub async fn find(id: &i32) -> Result<models::Function> {
        let path = format!("/function/{id}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
    }
    pub async fn find_by_project(project_id: &i32) -> Result<models::Function> {
        let path = format!("/function?project_id={project_id}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
    }
}

pub mod project {
    use actix_web::http;
    use anyhow::Result;

    use crate::utils::api::{self, models};

    pub fn save() -> Result<()> {
        Ok(())
    }
    pub async fn find(id: &i32) -> Result<models::Project> {
        let path = format!("/project/{id}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
    }
}

pub mod host {
    use anyhow::{Context, Result};

    use actix_web::http;
    use serde::Deserialize;

    use crate::{
        config,
        utils::api::{self, models},
    };

    pub async fn save(new_host: models::NewHost) -> Result<()> {
        let path = format!("/host");
        let res = api::request(http::Method::POST, &path[..], &Some(new_host)).await;
        res
    }
    pub async fn find(id: &i32) -> Result<models::Host> {
        let path = format!("/host/{id}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
    }
    pub async fn find_by_token(token: &str) -> Result<models::Host> {
        let path = format!("/host?token={token}");
        let res = api::request(http::Method::GET, &path[..], &Some("{}")).await;
        res
    }

    pub async fn save_function_connection(function_id: &i32) -> Result<models::HostsFunctions> {
        let host = current_host().await.context("Getting the host")?;
        let path = format!("/host/{}/{}/connect", host.id, function_id);
        let res = api::request(http::Method::POST, &path[..], &Some("{}")).await;
        res
    }

    pub async fn current_host() -> Result<models::Host> {
        let config = config::host::read_config();
        find_by_token(&config.host.unwrap().token).await
    }

    #[derive(Deserialize)]
    pub struct ConnectionResponse {
        is_online: bool,
    }
    pub async fn online() -> Result<ConnectionResponse> {
        // used for marking a host online
        let host = current_host().await?;
        let path = format!("/host/{}/connect", host.id);
        api::request(http::Method::POST, &path[..], &Some("{}")).await
    }
    pub async fn offline() -> Result<ConnectionResponse> {
        // to be called when a host kills their program
        let host = current_host().await?;
        let path = format!("/host/{}/disconnect", host.id);
        api::request(http::Method::POST, &path[..], &Some("{}")).await
    }
}
