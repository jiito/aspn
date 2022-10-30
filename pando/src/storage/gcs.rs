use chrono::Utc;
// use rand;
use reqwest::{
    header::{self, HeaderMap},
    Body,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
// use signature::{RandomizedSigner, Signature, Signer, Verifier};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use std::{collections::HashMap, fs, io::Error, path::PathBuf};
use tokio::fs::File;

use ring::{rand, signature};

pub async fn upload_file(filename: String, oauth_token: String) {
    let object_content_type = "text/plain";
    let bucket_name = "aspn_functions";

    // open file
    let file = File::open(filename.clone()).await.unwrap();
    let body = Body::from(file);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str(object_content_type).unwrap(),
    );

    let uri = format!("https://storage.googleapis.com/upload/storage/v1/b/{bucket_name}/o?uploadType=media&name={filename}");

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

#[derive(Debug, Deserialize, Serialize)]
struct ServiceAccount {
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
}

pub async fn generate_signed_url(
    service_account_file: String,
    bucket_name: String,
    object_name: String,
    expriation: Option<u32>,
    http_method: String,
    headers: Option<HashMap<String, String>>,
) -> Result<String, Error> {
    let expiration = expriation.unwrap_or(604800).to_string();

    // construct cannonical request
    let mut escaped_obj = String::new();
    let slice_obj = url_escape::encode_path_to_string(object_name, &mut escaped_obj);
    let cannonical_url = format!("/{}", slice_obj);

    // Get the current time and create a req timestamp

    let now = Utc::now();
    let request_timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
    let datestamp = now.format("%Y%m%d");

    let json_file = fs::read_to_string(service_account_file).expect("Unable to read file");
    let service_account: ServiceAccount =
        serde_json::from_str(&json_file).expect("JSON was not well-formatted");

    // Form the credential
    //
    let credential_scope = format!("{datestamp}/auto/storage/goog4_request");
    let credential = format!("{}/{}", service_account.client_email, credential_scope);

    // Form the headers for the cannonical request
    //
    let mut headers = headers.unwrap_or(HashMap::new());
    let host = format!("{}.storage.googleapis.com", bucket_name);
    headers.insert("host".to_string(), host.clone());

    let mut cannonical_headers = String::new();
    let mut signed_headers = String::new();
    let mut ordered_headers: Vec<(&String, &String)> = headers.iter().collect();
    ordered_headers.sort_by_key(|t| t.1);

    for (key, value) in ordered_headers.iter() {
        let lower_k = key.to_lowercase();
        let strip_v = value.to_lowercase();
        cannonical_headers.push_str(&format!("{}:{}\n", lower_k, strip_v));
        signed_headers.push_str(&format!("{};", lower_k))
    }
    signed_headers.pop();

    // Query parameters

    let mut query_params: HashMap<String, String> = HashMap::new();
    query_params.insert(
        "X-Goog-Algorithm".to_string(),
        "GOOG4-RSA-SHA256".to_string(),
    );
    query_params.insert("X-Goog-Credential".to_string(), credential);
    query_params.insert("X-Goog-Date".to_string(), String::from(&request_timestamp));
    query_params.insert("X-Goog-Expires".to_string(), expiration);
    query_params.insert(
        "X-Goog-SignedHeaders".to_string(),
        String::from(&signed_headers),
    );

    let mut canonical_query_string = String::new();

    let mut ordered_params: Vec<(&String, &String)> = query_params.iter().collect();
    ordered_params.sort_by_key(|t| t.0);

    for (key, value) in ordered_params.iter() {
        let encoded_k = url_escape::encode_component(key).to_string();
        let encoded_v = url_escape::encode_component(value).to_string();
        canonical_query_string.push_str(&format!("{}={}&", encoded_k, encoded_v));
    }
    canonical_query_string.pop();

    let canonical_request = vec![
        http_method,
        cannonical_url.clone(),
        canonical_query_string.clone(),
        cannonical_headers,
        signed_headers,
        String::from("UNSIGNED-PAYLOAD"),
    ]
    .join("\n");

    let mut hasher = Sha256::new();

    hasher.update(canonical_request);

    let cr_hash = hex::encode(hasher.finalize());

    // Construct the string-to-sign.

    let string_to_sign = vec![
        "GOOG4-RSA-SHA256".to_string(),
        String::from(&request_timestamp),
        credential_scope,
        cr_hash,
    ]
    .join("\n");

    //Sign the string-to-sign using an RSA signature with SHA-256.

    let private_key = rsa::RsaPrivateKey::from_pkcs8_pem(&service_account.private_key).unwrap();
    let pk_der = private_key.to_pkcs8_der().unwrap();

    let key_pair = ring::signature::RsaKeyPair::from_pkcs8(pk_der.as_bytes()).unwrap();

    // Encrypt
    let mut signed_buffer = vec![0; key_pair.public_modulus_len()];

    key_pair.sign(
        &signature::RSA_PKCS1_SHA256,
        &rand::SystemRandom::new(),
        &string_to_sign.as_bytes(),
        &mut signed_buffer,
    );

    let signature = hex::encode(signed_buffer);

    let scheme_and_host = format!("https://{host}");

    let signed_url = format!(
        "{}{}?{}&x-goog-signature={}",
        scheme_and_host,
        cannonical_url.clone(),
        canonical_query_string.clone(),
        signature
    );

    Ok(signed_url)
}
