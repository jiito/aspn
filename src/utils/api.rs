pub mod models;
pub mod storage;
use anyhow::{Context, Result};
use reqwest::{Body, Method};
use serde::{Deserialize, Serialize};
use std::{fmt::format, fs::File as StdFile, io::Cursor};
use tokio::fs::File;

use crate::config;

pub async fn request<T: Serialize, U>(method: Method, route: &str, data: &Option<T>) -> Result<U>
where
    U: for<'de> Deserialize<'de>,
{
    println!("[API] {} | {}", method.as_str(), route);
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str("application/json")?,
    );

    let body = Body::from(serde_json::to_string(data)?);

    let uri = format!("http://localhost:8080{}", route);

    let res: U = client
        .request(method, uri)
        .body(body)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedUrlResponse {
    pub uri: String,
}

// TODO: Move these handlers to their own module
pub async fn upload() -> Result<()> {
    let config = config::dev::read()?;

    let project_id = config
        .project
        .id
        .expect("Must have a project id in aspn.yaml. Please authenticate.");

    let project = storage::project::find(&project_id).await?;
    let entrypoint = config.service.entrypoint;

    let file_uri = format!("{}/{}", project.calculate_hash(), entrypoint);

    let signed_url_res: SignedUrlResponse =
        gcs::request_signed_url(gcs::SignedURLRequest::Upload {
            path: file_uri.clone(),
        })
        .await?;

    let file = File::open(&entrypoint).await?;

    let file_body = reqwest::Body::from(file);

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str("text/plain")?,
    );

    client
        .put(signed_url_res.uri)
        .body(file_body)
        .headers(headers)
        .send()
        .await
        .context("Could not upload the file")?;

    // save the file to a funciton record with the GCS link

    let func = models::NewFunction {
        project_id,
        gcs_uri: file_uri,
        route: config.service.route,
    };
    storage::function::save(&func).await?;

    Ok(())
}

pub async fn download(project_id: &i32) -> Result<()> {
    let project = storage::project::find(project_id).await?;

    //find the functions related to the project
    let function = storage::function::find_by_project(project_id).await?;

    println!("The Function is : {:?}", function);
    // download the function

    let signed_url_res: SignedUrlResponse =
        gcs::request_signed_url(gcs::SignedURLRequest::Download {
            path: function.gcs_uri,
        })
        .await
        .with_context(|| "Could not get signed url for downloading { (function.gcs_uri) }")?;

    // Create the file in the config dirtectory
    let project_dir = project.host_dir();
    let mut dest = StdFile::create(format!("{}Dockerfile", project_dir.display()))?;

    // TODO: abstract this above with interatcing with the signed urls
    let client = reqwest::Client::new();

    let response = client
        .get(signed_url_res.uri)
        .send()
        .await
        .context("Downloading file")?;

    let mut content = Cursor::new(response.bytes().await.context("Setting cursor")?);

    std::io::copy(&mut content, &mut dest).context("Copying file to dest")?;

    // connect the host to the fucntion
    storage::host::save_function_connection(&function.id)
        .await
        .context("Saving function connection")?;

    Ok(())
}

mod gcs {
    use super::SignedUrlResponse;
    use anyhow::{Context, Result};

    pub enum SignedURLRequest {
        Upload { path: String },
        Download { path: String },
    }

    pub async fn request_signed_url(request: SignedURLRequest) -> Result<SignedUrlResponse> {
        let client = reqwest::Client::new();

        let (method, path) = match request {
            SignedURLRequest::Upload { path } => ("PUT", path),
            SignedURLRequest::Download { path } => ("GET", urlencoding::encode(&path).to_string()),
        };

        println!("{} : {}", method, path);
        let response: SignedUrlResponse = client
            .get("http://localhost:8080/signed_url") // TODO; move this to the API URL env var
            .query(&[("method", method), ("object_name", &path[..])])
            .send()
            .await
            .with_context(|| "Could not send the request")?
            .json::<SignedUrlResponse>()
            .await?;

        println!("{:?}", response);
        Ok(response)
    }
}
