use crate::{commands, utils};

pub async fn start() {
    println!("Connecting to the ASPN cloud...");

    // TODO: Check authentication?
    if !utils::config::credentials_exist() {
        commands::auth().await;
    }
    // Spin up microservice

    // create the ~/.aspn directory
    std::fs::create_dir_all("~/.aspn").expect("Could not create config directory");
}
