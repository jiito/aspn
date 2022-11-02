use pando::storage;

#[tokio::main]
async fn main() {
    storage::gcs::generate_signed_url(
        String::from("/Users/bjar/service-account.json"),
        String::from("aspn_functions"),
        String::from("testObject"),
        None,
        "GET".to_string(),
        None,
    )
    .await;
    // storage::gcs::download_file();

    // storage::util::upload_file().await;
}
