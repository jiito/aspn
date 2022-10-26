pub async fn upload_file() {
    let res = request_gcp().await;

    println!("res = {:#?}", res)
}

pub fn download_file() {
    println!("Downloading file")
}

async fn request_gcp() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://example.com")
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
