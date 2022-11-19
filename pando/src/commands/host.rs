use crate::{commands, storage, utils};

pub async fn start() {
    println!("Connecting to the ASPN cloud...");

    let conn = &mut storage::db::establish_connection();

    // TODO: Check authentication?
    if !utils::config::host::config_exist() {
        commands::auth().await;
    }
    let db_project = storage::db::find_project(conn, &1);

    utils::config::project::save_project_connnection(db_project);

    let project = utils::config::project::read_project_connection();

    let project_path = match project {
        Some(project) => project.path,
        None => panic!("could not find a project!"),
    };

    println!("The Project Path is {}", project_path.display())

    // run the project from the path

    // let command = std::process::Command::new("wasmtime");
    // Spin up microservice
    // utils::wasm::start(project_path.to_str().unwrap());
}
