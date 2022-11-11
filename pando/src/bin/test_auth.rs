use pando::utils;

#[tokio::main]
async fn main() {
    let device_code = utils::auth::request_device_code().await;
}
