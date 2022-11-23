use anyhow::{Context, Result};
use tokio::fs::File;

use crate::{
    models::NewFunction,
    storage::{self, db::establish_connection},
    utils,
};

pub async fn upload() -> Result<()> {
    // find the file to upload from the config
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
