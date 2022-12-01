use anyhow::Context;

use crate::{
    commands, config,
    utils::{self, api},
};

pub async fn start() {
    println!("Connecting to the ASPN cloud...");

    // TODO: Check authentication?
    if !config::host::config_exist() {
        commands::auth().await;
    }
    let db_project = api::storage::project::find(&1).await.unwrap();

    config::project::save_project_connnection(&db_project).unwrap();

    utils::api::download(&db_project.id).await.unwrap();
    println!("Succesfully downloaded file...");

    let project = config::project::read_project_connection()
        .with_context(|| "No Project Config")
        .unwrap();

    api::storage::host::online().await.unwrap();
    // Spin up microservice
    // todo!("Use the dockerfile over wasm ");
    // utils::wasm::start(format!("{}main.wasm", project.path.to_str().unwrap()).as_str())
    //     .expect("Could not run the program");
}
