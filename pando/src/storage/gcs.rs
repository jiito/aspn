use reqwest::{
    header::{self, HeaderMap},
    Body,
};
use std::path::PathBuf;
use tokio::fs::File;

use gcp_auth::{AuthenticationManager, CustomServiceAccount};

pub async fn upload_file(oauth_token: String) {
    let file_location = String::from("/Users/bjar/foo.txt");
    let object_content_type = "text/plain";
    let object_name = "testObject2";
    let bucket_name = "aspn_functions";

    // open file
    let file = File::open(file_location).await.unwrap();
    let body = Body::from(file);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str(object_content_type).unwrap(),
    );

    let uri = format!("https://storage.googleapis.com/upload/storage/v1/b/{bucket_name}/o?uploadType=media&name={object_name}");

    let res = request_gcp(uri, headers, body, oauth_token).await;

    println!("res = {:#?}", res)
}

pub fn download_file() {
    println!("Downloading file")
}

async fn request_gcp(
    path: String,
    headers: HeaderMap,
    body: Body,
    oauth_token: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post(path)
        .headers(headers)
        .bearer_auth(oauth_token)
        .body(body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
pub async fn get_gcp_token() -> String {
    let credentials_path = PathBuf::from("/Users/bjar/service-account.json");
    let service_account = CustomServiceAccount::from_file(credentials_path).unwrap();
    let authentication_manager = AuthenticationManager::from(service_account);
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await.unwrap();
    String::from(token.as_str())
}
