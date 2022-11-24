use anyhow::{Context, Result};
use std::{fs::File as StdFile, io::Cursor};
use tokio::fs::File;

use crate::{
    models::NewFunction,
    storage::{self, db::establish_connection},
    utils,
};

use super::config;

pub async fn upload() -> Result<()> {
    let config = utils::config::dev::read()?;

    let project_id = config
        .project
        .id
        .expect("Must have a project id in aspn.yaml. Please authenticate");

    let entrypoint = config.service.entrypoint;

    let signed_url_res: crate::api::handlers::SignedUrlResponse =
        storage::gcs::request_signed_url(storage::gcs::SignedURLRequest::Upload {
            path: entrypoint.clone(),
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

    let res = client
        .put(signed_url_res.uri)
        .body(file_body)
        .headers(headers)
        .send()
        .await
        .context("Could not upload the file")?;

    // save the file to a funciton record with the GCS link
    let conn = &mut establish_connection();
    let new_func = NewFunction {
        gcs_uri: String::from(entrypoint),
        route: config.service.route,
        project_id,
    };

    let function = storage::db::functions::save(conn, &new_func).expect("Error saving function");

    Ok(())
}

pub async fn download(project_id: &i32) -> Result<()> {
    let conn = &mut establish_connection();
    let project = storage::db::find_project(conn, project_id);
    let config = utils::config::host::read_config();

    //find the functions related to the project
    let function = storage::db::functions::find_by_project(conn, project_id)?;

    // download the function

    let signed_url_res: crate::api::handlers::SignedUrlResponse =
        storage::gcs::request_signed_url(storage::gcs::SignedURLRequest::Download {
            path: function.gcs_uri.clone(),
        })
        .await
        .with_context(|| "Could not get signed url for downloading { (function.gcs_uri) }")?;

    // Create the file in the config dirtectory
    let project_dir = project.host_dir();
    let mut dest = StdFile::create(format!("{}main.wasm", project_dir.display()))?;

    // TODO: abstract this above with interatcing with the signed urls
    let client = reqwest::Client::new();

    let response = client.get(signed_url_res.uri).send().await?;

    let mut content = Cursor::new(response.bytes().await?);

    std::io::copy(&mut content, &mut dest)?;

    // connect the host to the fucntion
    let host = storage::db::hosts::find_host_by_token(conn, &config.host.unwrap().token);
    let host_function = storage::db::hosts::connect_host_to_function(conn, &host, &function);

    Ok(())
}
