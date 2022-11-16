use std::io::Write;

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

pub fn parse_config() {
    let f = std::fs::File::open("./aspn.yaml").expect("Could not open file");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values");

    println!("{:?}", scrape_config);
}

pub fn write_default_config(config: &Config) {
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

#[derive(Deserialize, Serialize)]
pub struct CredentialsData {
    host: HostCredentials,
}
#[derive(Deserialize, Serialize)]
pub struct HostCredentials {
    token: String,
}

pub fn save_token_to_file(token: String) {
    std::fs::create_dir_all(aspn_dir()).expect("Could not create config directory");

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/credentials", aspn_dir()))
        .expect("Could not create token file");

    let data = CredentialsData {
        host: HostCredentials {
            token: token.clone(),
        },
    };

    let toml = toml::to_string(&data).unwrap();

    f.write_all(toml.as_bytes())
        .expect("Cannot write token to file");
}

pub fn read_credentials_file() -> CredentialsData {
    let file =
        std::fs::read_to_string(credentials_path()).expect("Could not read credentials file");
    let credentials: CredentialsData = toml::from_str(&file).unwrap();
    credentials
}

fn home() -> Result<std::path::PathBuf, i32> {
    home::home_dir().ok_or(0)
}

fn aspn_dir() -> String {
    format!("{}/.aspn", home().unwrap().display())
}

fn credentials_path() -> String {
    format!("{}/credentials", aspn_dir())
}

pub fn credentials_exist() -> bool {
    std::path::Path::new(&credentials_path()).exists()
}
