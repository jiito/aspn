pub mod host;
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
        scope: "openid profile email",
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
            thread::sleep(Duration::from_secs(interval));
        }
    };

    let json_res = response.json::<AccessTokenRes>().await?;

    Ok(json_res)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    // pub sub: Option<String>,
    pub name: String,
    // #[serde(rename = "given_name")]
    // pub given_name: String,
    // #[serde(rename = "family_name")]
    // pub family_name: String,
    // #[serde(rename = "middle_name")]
    // pub middle_name: String,
    // pub nickname: String,
    // #[serde(rename = "preferred_username")]
    // pub preferred_username: String,
    // pub profile: String,
    // pub picture: String,
    // pub website: String,
    pub email: String,
    // #[serde(rename = "email_verified")]
    // pub email_verified: bool,
    // pub gender: String,
    // pub birthdate: String,
    // pub zoneinfo: String,
    // pub locale: String,
    // #[serde(rename = "phone_number")]
    // pub phone_number: String,
    // #[serde(rename = "phone_number_verified")]
    // pub phone_number_verified: bool,
    // pub address: Address,
    // #[serde(rename = "updated_at")]
    // pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: String,
}
pub async fn get_user(access_token: &str) -> Result<UserResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response: UserResponse = client
        .get("https://dev-fnppi16zn0on4m4a.us.auth0.com/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await?;

    // Ok(())
    Ok(response)
}
