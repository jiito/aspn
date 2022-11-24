use anyhow::Context;

use crate::{commands, storage, utils};

pub async fn start() {
    println!("Connecting to the ASPN cloud...");

    let conn = &mut storage::db::establish_connection();

    // TODO: Check authentication?
    if !utils::config::host::config_exist() {
        commands::auth().await;
    }
    let db_project = storage::db::find_project(conn, &1);

    utils::config::project::save_project_connnection(&db_project);

    utils::api::download(&db_project.id).await.unwrap();

    let project = utils::config::project::read_project_connection()
        .with_context(|| "No Project Config")
        .unwrap();

    println!("The Project Path is {}", project.path.display());

    // run the project from the path

    // Spin up microservice
    utils::wasm::start(format!("{}main.wasm", project.path.to_str().unwrap()).as_str())
        .expect("Could not run the program");
}
