use aspn::utils::api::storage::function::find;

#[tokio::main]
async fn main() {
    let res = find(&3).await.unwrap();
    println!("{:?}", res);
}
