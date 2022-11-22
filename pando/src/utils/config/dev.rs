use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    project: ProjectConfig,
    developer: DeveloperConfig,
    service: ServiceConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectConfig {
    name: String,
    url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DeveloperConfig {
    email: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceConfig {
    entrypoint: String,
    build: String,
}

pub fn read() -> Result<Config> {
    let f = std::fs::File::open("./aspn.yaml").with_context(|| "Could not open file")?;
    let scrape_config: Config = serde_yaml::from_reader(f)?;
    Ok(scrape_config)
}

pub fn write(config: &Config) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("aspn.yaml")
        .expect("Could not create config file");
    serde_yaml::to_writer(f, config).unwrap();
}

pub fn create_config(name: &str) -> Config {
    // TODO: Check login state to get user data

    Config {
        project: ProjectConfig {
            name: String::from(name),
            url: Some(String::new()),
        },
        developer: DeveloperConfig {
            email: Some(String::new()),
        },
        service: ServiceConfig {
            entrypoint: String::new(),
            build: String::new(),
        },
    }
}
