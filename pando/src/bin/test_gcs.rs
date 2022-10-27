use pando::storage::{
    self,
    gcs::{create_jwt, get_access_token, get_gcp_token},
};

#[tokio::main]
async fn main() {
    let at = get_gcp_token().await;
    println!("{:?}", at);
    storage::gcs::upload_file(at).await;

    // storage::gcs::download_file();
}
