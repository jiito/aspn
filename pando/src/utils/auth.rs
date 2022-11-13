use std::{thread, time::Duration};

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
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: i32,
    pub interval: u64,
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

#[derive(Serialize)]
struct AccessTokenReqBody {
    grant_type: String,
    device_code: String,
    client_id: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
pub async fn request_access_token(
    device_code: String,
    interval: u64,
) -> Result<AccessTokenRes, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );

    let body = AccessTokenReqBody {
        grant_type: String::from("urn:ietf:params:oauth:grant-type:device_code"),
        device_code,
        client_id: String::from("XVIVyn7Ha2hGJH423vVhjiuXRjVy6e8j"),
    };

    let response = loop {
        println!("Requesting token!");
        let response = client
            .post("https://dev-fnppi16zn0on4m4a.us.auth0.com/oauth/token")
            .headers(headers.clone())
            .json(&body)
            .send()
            .await
            .expect("Bad request");

        if response.status() == 200 {
            break response;
        } else {
            println!("{:?}", response.text().await);
            thread::sleep(Duration::from_secs(interval));
        }
    };

    let json_res = response.json::<AccessTokenRes>().await?;

    Ok(json_res)
}
