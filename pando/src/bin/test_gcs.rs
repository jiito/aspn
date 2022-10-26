use pando::storage;

#[tokio::main]
async fn main() {
    storage::gcs::upload_file().await;
    // storage::gcs::download_file();
}
