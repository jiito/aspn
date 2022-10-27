use std::fs;

use reqwest::{header, Body};
use tokio::fs::File;

pub async fn upload_file() {
    let res = request_gcp().await;

    println!("res = {:#?}", res)
}

pub fn download_file() {
    println!("Downloading file")
}

async fn request_gcp() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let file_location = "/Users/bjar/foo.txt";
    let oauth_token = "ya29.a0Aa4xrXP6IhG3rWKi-5uAbykp_tbTBgWTifE-MQREPsrN6pS45C_QRy6uRWwadK7DvkdHvJDl89hmTFq5yThdFob-wLvMDyc2m2oxr7ypjxYZgoo_mQJg7ngXcMFTo6_4JpgXrpCpK4p8m27gc5LkikYQaPbXaCgYKATASARISFQEjDvL9sFegKq_LX08Zg4NpvZqrjQ0163";
    let object_content_type = "text/plain";
    let object_name = "testObject";
    let bucket_name = "aspn_functions";

    // open file
    let file = File::open(file_location).await?;
    let body = Body::from(file);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str(object_content_type).unwrap(),
    );

    let uri = format!("https://storage.googleapis.com/upload/storage/v1/b/{bucket_name}/o?uploadType=media&name={object_name}");

    let response = client
        .post(uri)
        .headers(headers)
        .bearer_auth(oauth_token)
        .body(body)
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Status: {:#?}", response.text().await?);

    Ok(String::new())
}
