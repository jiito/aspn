use std::io::Error;

use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignedUrlQueryParams {
    method: String,
    object_name: String,
}

pub async fn signed_url_handler(
    query_params: web::Query<SignedUrlQueryParams>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!(
        "{}, {}",
        query_params.method, query_params.object_name
    )))
}
