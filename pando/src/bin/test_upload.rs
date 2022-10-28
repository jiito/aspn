use pando::storage;

#[tokio::main]
async fn main() {
    storage::pando::upload_pando().await;

    // storage::gcs::download_file();
}
