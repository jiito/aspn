pub mod developer;
pub mod host;
use crate::{
    config::{
        self,
        dev::{Config, DeveloperConfig},
    },
    utils::{
        self,
        api::{self, models},
    },
    with_loader,
};
use anyhow::{Context, Result};
use clap::Subcommand;
use local_ip_address::local_ip;
use webbrowser;

#[derive(Subcommand, Debug)]
pub enum Host {
    Start {},
    Auth {},
}
#[derive(Subcommand, Debug)]
pub enum Developer {
    Upload {},
    Auth {},
}

pub async fn init() -> Result<()> {
    let project_name = inquire::Text::new("What is the title of your project?").prompt();

    match project_name {
        Ok(name) => {
            let mut config = config::dev::create_config(&name);

            let developer = api::storage::developer::current()
                .await
                .context("Can't find developer")?;

            let project = api::storage::project::save(&name, &developer.id)
                .await
                .context("Can't save project")?;

            config.project.id = Some(project.id);
            config::dev::write(&config);
            println!("Successfully wrote config [aspn.yaml]");
        }
        Err(_) => println!("Couldn't get the project name"),
    }
    Ok(())
}

pub enum UserType {
    Dev,
    Host,
}

pub async fn auth(user_type: UserType) -> Result<()> {
    if is_authenticated() {
        let host_creds = config::host::read_config().host.expect("No credentials");
        let user = utils::auth::get_user(&host_creds.token)
            .await
            .expect("Could not get user");
        save_user(user_type, &host_creds.token).await?;
        println!("You're already logged in as {}", user.email);
    } else {
        let device_code = utils::auth::request_device_code()
            .await
            .expect("Could not get device code");

        println!("Opening {}", &device_code.verification_uri_complete.clone());
        if webbrowser::open(&device_code.verification_uri_complete).is_ok() {
            let access_token = with_loader!(
                utils::auth::request_access_token(device_code.device_code, device_code.interval)
                    .await,
                "Authenticating"
            )
            .expect("Could not get access token");
            let user = utils::auth::get_user(&access_token.access_token)
                .await
                .expect("Could not get user");

            // TODO: find way to branch this before the host save
            config::host::save_token_to_config(&access_token.access_token);
            save_user(user_type, &access_token.access_token).await?;
            println!("Successfully logged in as: {}", user.email);
        };
    }
    Ok(())
}

fn is_authenticated() -> bool {
    let config = config::host::read_config();
    match config.host {
        Some(_) => true,
        None => false,
    }
}
async fn save_user(user_type: UserType, token: &String) -> Result<()> {
    match user_type {
        UserType::Dev => {
            let dev = models::NewDeveloper {
                project_id: None,
                auth_token: Some(token.clone()),
            };
            api::storage::developer::save(&dev).await?;
        }
        UserType::Host => {
            let host = models::NewHost {
                ip_address: ipnetwork::IpNetwork::new(local_ip()?, 32)?,
                user_token: token.clone(),
            };
            api::storage::host::save(host).await?;
        }
    }
    Ok(())
}
