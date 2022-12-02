use anyhow::Context;
use tokio::process::Command;

use crate::{
    commands, config,
    utils::{self, api},
};

pub async fn start() {
    println!("Connecting to the ASPN cloud...");

    if !config::host::config_exist() {
        commands::auth(commands::UserType::Host).await;
    }
    let db_project = api::storage::project::find(&4).await.unwrap();

    config::project::save_project_connnection(&db_project).unwrap();

    utils::api::download(&db_project.id).await.unwrap();
    println!("Succesfully downloaded file...");

    let project = config::project::read_project_connection()
        .with_context(|| "No Project Config")
        .unwrap();

    api::storage::host::online().await.unwrap();
    // Spin up microservice
    utils::docker::build(
        format!("{}Dockerfile", project.path.to_str().unwrap()).as_str(),
        &project.name,
    )
    .await;
    utils::docker::start(&project.name).await;
}
