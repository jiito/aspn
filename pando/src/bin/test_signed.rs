use pando::storage;

#[tokio::main]
async fn main() {
    let url = storage::gcs::generate_signed_url(
        String::from("/Users/bjar/service-account.json"),
        String::from("aspn_functions"),
        String::from("TestImage"),
        None,
        "PUT".to_string(),
        None,
    )
    .await
    .unwrap();
    println!("{}", url)
}
