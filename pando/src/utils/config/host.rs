use serde::{Deserialize, Serialize};
use std::io::Write;

use super::project;
#[derive(Deserialize, Serialize, Default)]
pub struct ConfigData {
    pub host: Option<HostCredentials>,
    pub project: Option<project::ProjectData>,
}
impl ConfigData {
    fn merge(self, other: ConfigData) -> Self {
        Self {
            host: self.host.or(other.host),
            project: self.project.or(other.project),
        }
    }
    fn update(self, new: ConfigData) -> Self {
        Self {
            host: new.host.or(self.host),
            project: new.project.or(self.project),
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct HostCredentials {
    pub token: String,
}

pub fn save_token_to_config(token: &str) {
    let token_config = ConfigData {
        host: Some(HostCredentials {
            token: String::from(token),
        }),
        project: None,
    };
    update_config(token_config)
}

pub fn read_config() -> ConfigData {
    if config_exist() {
        let file = std::fs::read_to_string(config_path()).expect("Could not read credentials file");
        let config: ConfigData = toml::from_str(&file).unwrap();
        config
    } else {
        ConfigData {
            ..ConfigData::default()
        }
    }
}

fn home() -> Result<std::path::PathBuf, i32> {
    home::home_dir().ok_or(0)
}

pub fn aspn_dir() -> String {
    format!("{}/.aspn", home().unwrap().display())
}

fn config_path() -> String {
    format!("{}/config", aspn_dir())
}

pub fn config_exist() -> bool {
    std::path::Path::new(&config_path()).exists()
}

fn write_config(config: &ConfigData) {
    if !config_exist() {
        create_config_dir();
    }
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/config", aspn_dir()))
        .expect("Could not create config file");

    let toml = toml::to_string(config).unwrap();

    f.write_all(toml.as_bytes())
        .expect("Cannot write to config file");
}

fn create_config_dir() {
    std::fs::create_dir_all(aspn_dir()).expect("Could not create config directory");
}

pub fn update_config(new_config: ConfigData) {
    let old_config = read_config();
    let config = old_config.update(new_config);
    write_config(&config)
}
