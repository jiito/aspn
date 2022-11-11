use actix_web::HttpResponse;
use reqwest::{
    header::{self, HeaderMap},
    Body,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct DeviceCodeBody<'a> {
    client_id: &'a str,
    scope: &'a str,
    audience: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct DeviceCodeRes {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: i32,
    interval: i32,
}

pub async fn request_device_code() -> Result<DeviceCodeRes, std::io::Error> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );

    let body = DeviceCodeBody {
        client_id: "XVIVyn7Ha2hGJH423vVhjiuXRjVy6e8j",
        scope: "profile",
        audience: "pando.aspn.network",
    };

    let response: DeviceCodeRes = client
        .post("https://dev-fnppi16zn0on4m4a.us.auth0.com/oauth/device/code")
        .headers(headers)
        .json(&body)
        .send()
        .await
        .expect("Bad request")
        .json()
        .await
        .expect("Couldnt deserialize JSON");

    println!("Device code response {:?}", response);

    Ok(response)
}
