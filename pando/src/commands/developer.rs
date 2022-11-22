use anyhow::Result;
use tokio::fs::File;

use crate::{storage, utils};

pub async fn upload() -> Result<()> {
    // find the file to upload from the config
    let config = utils::config::dev::read()?;

    let entrypoint = config.service.entrypoint;
    // upload the file to GCS using a signed url
    // let access_token = storage::gcs::
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
        .await?;
    // save the file to a funciton record with the GCS link
    Ok(())
}
