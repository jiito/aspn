// These methods will be used to call the Pando CRUD api endpoints

pub mod developer {
    use anyhow::{anyhow, Context, Result};
    use reqwest::Method;

    use crate::{
        config,
        utils::api::{self, models},
    };

    pub async fn current() -> Result<models::Developer> {
        let config = config::host::read_config();
        let user = config.host.context("Must authenticate first")?;
        find_by_token(&user.token).await
    }
    pub async fn save(dev: &models::NewDeveloper) -> Result<models::Developer> {
        let path = format!("/developer");
        let res = api::request(Method::POST, &path[..], &Some(dev)).await;
        res
    }
    pub async fn find_by_token(token: &str) -> Result<models::Developer> {
        let path = format!("/developer?token={token}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }
}
pub mod function {
    use anyhow::Result;
    use reqwest::Method;

    use crate::utils::api::{self, models};

    pub async fn save(new_function: &models::NewFunction) -> Result<models::Function> {
        let path = format!("/function");
        let res = api::request(Method::POST, &path[..], &Some(new_function)).await;
        res
    }
    pub async fn find(id: &i32) -> Result<models::Function> {
        let path = format!("/function/{id}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }
    pub async fn find_by_project(project_id: &i32) -> Result<models::Function> {
        let path = format!("/function?project_id={project_id}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }
}

pub mod project {
    use anyhow::Result;
    use reqwest::Method;
    use serde::Serialize;

    use crate::utils::api::{self, models};

    #[derive(Serialize)]
    pub struct CreateProjectData<'a> {
        name: &'a str,
        developer_id: &'a i32,
    }
    pub async fn save(name: &str, developer_id: &i32) -> Result<models::Project> {
        let path = format!("/project");
        let res = api::request(
            Method::POST,
            &path[..],
            &Some(CreateProjectData { name, developer_id }),
        )
        .await;
        res
    }
    pub async fn find(id: &i32) -> Result<models::Project> {
        let path = format!("/project/{id}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }
}

pub mod host {
    use anyhow::{Context, Result};

    use reqwest::Method;
    use serde::Deserialize;

    use crate::{
        config,
        utils::api::{self, models},
    };

    pub async fn save(new_host: models::NewHost) -> Result<models::Host> {
        let path = format!("/host");
        let res = api::request(Method::POST, &path[..], &Some(new_host)).await;
        res
    }
    pub async fn find(id: &i32) -> Result<models::Host> {
        let path = format!("/host/{id}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }
    pub async fn find_by_token(token: &str) -> Result<models::Host> {
        let path = format!("/host?token={token}");
        let res = api::request(Method::GET, &path[..], &Some("{}")).await;
        res
    }

    pub async fn save_function_connection(function_id: &i32) -> Result<models::HostsFunctions> {
        let host = current_host().await.context("Getting the host")?;
        let path = format!("/host/{}/{}/connect", host.id, function_id);
        let res = api::request(Method::POST, &path[..], &Some("{}")).await;
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
        api::request(Method::POST, &path[..], &Some("{}")).await
    }
    pub async fn offline() -> Result<ConnectionResponse> {
        // to be called when a host kills their program
        let host = current_host().await?;
        let path = format!("/host/{}/disconnect", host.id);
        api::request(Method::POST, &path[..], &Some("{}")).await
    }
}
