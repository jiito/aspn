use pando::storage::{self, gcs::create_jwt};

#[tokio::main]
async fn main() {
    // storage::gcs::upload_file().await;
    create_jwt();
    // storage::gcs::download_file();
}
