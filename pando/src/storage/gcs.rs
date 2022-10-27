use reqwest::{
    header::{self, HeaderMap},
    Body,
};
use tokio::fs::File;

pub async fn upload_file(
    file_path: String,
    oauth_token: String,
    object_name: String,
    bucket_name: String,
) {
    let file_location = String::from("/Users/bjar/foo.txt");
    let oauth_token = String::from("ya29.a0Aa4xrXP6IhG3rWKi-5uAbykp_tbTBgWTifE-MQREPsrN6pS45C_QRy6uRWwadK7DvkdHvJDl89hmTFq5yThdFob-wLvMDyc2m2oxr7ypjxYZgoo_mQJg7ngXcMFTo6_4JpgXrpCpK4p8m27gc5LkikYQaPbXaCgYKATASARISFQEjDvL9sFegKq_LX08Zg4NpvZqrjQ0163");
    let object_content_type = "text/plain";
    let object_name = "testObject";
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
