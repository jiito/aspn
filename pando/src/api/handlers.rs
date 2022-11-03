use std::io::Error;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::storage;

#[derive(Deserialize)]
pub struct SignedUrlQueryParams {
    method: String,
    object_name: String,
}

pub async fn signed_url_handler(
    query_params: web::Query<SignedUrlQueryParams>,
) -> Result<HttpResponse, Error> {
    // build signed url

    let uri = storage::gcs::generate_signed_url(
        "/Users/bjar/service-account.json".into(),
        "aspn_functions".into(),
        query_params.object_name.clone(),
        None,
        query_params.method.clone(),
        None,
    )
    .await?;
    Ok(HttpResponse::Ok().json(format!("{{ uri: {} }}", uri)))
}
