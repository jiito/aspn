use crate::utils;

pub fn init() {
    let project_name = inquire::Text::new("What is the title of your project?").prompt();

    match project_name {
        Ok(name) => {
            let config = utils::config::create_config(&name);
            utils::config::write_default_config(&config);
            println!("Successfully wrote config [aspn.yaml]")
        }
        Err(_) => println!("Couldn't get the project name"),
    }
}
