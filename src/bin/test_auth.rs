use aspn::utils;

#[tokio::main]
async fn main() {
    let res = utils::auth::request_device_code().await.unwrap();

    let res = utils::auth::request_access_token(res.device_code, res.interval)
        .await
        .unwrap();
}
