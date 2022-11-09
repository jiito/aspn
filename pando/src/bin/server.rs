use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .route(
                "/signed_url",
                web::get().to(pando::api::handlers::signed_url_handler),
            )
            .route(
                "/project",
                web::post().to(pando::api::handlers::create_project),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
