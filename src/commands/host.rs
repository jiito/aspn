use anyhow::{Context, Result};
use tokio::process::Command;

use crate::{
    commands, config, http,
    utils::{self, api},
    with_loader,
};

pub async fn start() -> Result<()> {
    println!("Connecting to the ASPN cloud...");

    if !config::host::config_exist() {
        commands::auth(commands::UserType::Host).await?;
    }
    let db_projects = api::storage::project::find_all().await?;

    let selected_project = inquire::Select::new(
        "Which ASPN project would you like to connect to?",
        db_projects,
    )
    .prompt()?;

    config::project::save_project_connnection(&selected_project)?;

    with_loader!(
        utils::api::download(&selected_project.id).await?,
        "Downloading the secret sauce..."
    );
    println!("Succesfully downloaded file 🏄‍♂️", );

    api::storage::host::online().await?;

    println!("Listening for requests on http://localhost:4011");
    // TODO: Start this on its own thread
    http::server::start(&selected_project).await;

    Ok(())
}
