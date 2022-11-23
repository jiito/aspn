use std::{io::Error, vec};

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::storage;

#[derive(Deserialize, Debug)]
pub struct SignedUrlQueryParams {
    method: String,
    object_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedUrlResponse {
    pub uri: String,
}
pub async fn signed_url_handler(query_params: web::Query<SignedUrlQueryParams>) -> impl Responder {
    // build signed url
    let uri = storage::gcs::generate_signed_url(
        "/Users/bjar/service-account.json".into(),
        "aspn_functions".into(),
        query_params.object_name.clone(),
        None,
        query_params.method.clone(),
        None,
    )
    .await
    .unwrap();

    web::Json(SignedUrlResponse { uri })
}

#[derive(Deserialize)]
pub struct CreateProjectData {
    name: String,
    developer_id: i32,
}
pub async fn create_project(data: web::Json<CreateProjectData>) -> Result<HttpResponse, Error> {
    let conn = &mut storage::db::establish_connection();
    // create project in databse
    let project = storage::db::create_project(conn, &data.name);
    println!("{:?}", project);

    // add project to developer accounts
    let developer = storage::db::find_developer(conn, &data.developer_id);

    println!("{:?}", developer);

    // return a response to the user

    // Ok(HttpResponse::Ok().json(format!("{{ project: {} }}", project)))
    Ok(HttpResponse::Ok().json(format!("{{ project: {} }}", "{}")))
}
