use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| App::new().route("/upload", web::post().to(pando::storage::file::save_file)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
