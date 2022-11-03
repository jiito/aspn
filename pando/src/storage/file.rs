use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use uuid::Uuid;

use crate::storage;

pub async fn save_file(mut payload: Multipart) -> Result<String, Error> {
    // iterate over multipart stream
    let filepath = String::new();
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        println!("{}", filename);
        let filepath = format!("./tmp/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    println!("File recieved!");

    Ok(filepath)
}

// pub async fn upload_handler(payload: Multipart) -> Result<HttpResponse, Error> {
//     let filename = save_file(payload).await?;

//     // upload file to GCP

//     let at = storage::gcs::get_gcp_token().await;
//     println!("{:?}", at);
//     storage::gcs::upload_file(filename, at).await;
//     Ok(HttpResponse::Ok().into())
// }
