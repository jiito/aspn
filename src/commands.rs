pub mod developer;
pub mod host;
use crate::{
    config,
    utils::{
        self,
        api::{self, models},
    },
};
use anyhow::Result;
use clap::Subcommand;
use local_ip_address::local_ip;
use webbrowser;

#[derive(Subcommand, Debug)]
pub enum Host {
    Start {},
}
#[derive(Subcommand, Debug)]
pub enum Developer {
    Upload {},
}

pub async fn init() -> Result<()> {
    let project_name = inquire::Text::new("What is the title of your project?").prompt();

    match project_name {
        Ok(name) => {
            let config = config::dev::create_config(&name);
            config::dev::write(&config);

            let developer = api::storage::developer::current().await?;

            api::storage::project::save(&name, &developer.id).await?;
            println!("Successfully wrote config [aspn.yaml]");
        }
        Err(_) => println!("Couldn't get the project name"),
    }
    Ok(())
}

pub async fn auth() -> Result<()> {
    let device_code = utils::auth::request_device_code()
        .await
        .expect("Could not get device code");

    if webbrowser::open(&device_code.verification_uri_complete).is_ok() {
        let access_token =
            utils::auth::request_access_token(device_code.device_code, device_code.interval)
                .await
                .expect("Could not get access token");
        let user = utils::auth::get_user(&access_token.access_token)
            .await
            .expect("Could not get user");

        // TODO: find way to branch this before the host save
        config::host::save_token_to_config(&access_token.access_token);
        let project_id = config::dev::read()?
            .project
            .id
            .expect("Must set project ID in config.yaml");
        let dev = models::NewDeveloper {
            name: user.name,
            project_id,
            auth_token: Some(access_token.access_token.clone()),
        };
        let host = models::NewHost {
            ip_address: ipnetwork::IpNetwork::new(local_ip()?, 32)?,
            user_token: access_token.access_token,
        };
        api::storage::developer::save(&dev).await?;
        api::storage::host::save(host).await?;
    };
    Ok(())
}
