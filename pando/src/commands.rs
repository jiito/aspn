pub mod developer;
pub mod host;
use crate::utils;
use clap::Subcommand;
use webbrowser;

#[derive(Subcommand, Debug)]
pub enum Host {
    Start {},
}
#[derive(Subcommand, Debug)]
pub enum Developer {
    Upload {},
}

pub fn init() {
    let project_name = inquire::Text::new("What is the title of your project?").prompt();

    match project_name {
        Ok(name) => {
            let config = utils::config::dev::create_config(&name);
            utils::config::dev::write(&config);
            println!("Successfully wrote config [aspn.yaml]")
        }
        Err(_) => println!("Couldn't get the project name"),
    }
}

pub async fn auth() {
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
        println!("Successfully logged in!  Hello, {}!!", user.name);

        utils::config::host::save_token_to_config(access_token.access_token);
    }
}
