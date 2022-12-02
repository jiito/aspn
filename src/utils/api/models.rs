use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

impl Project {
    pub fn calculate_hash(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish().to_string()
    }
    pub fn host_dir(&self) -> std::path::PathBuf {
        let path = std::path::PathBuf::from(format!(
            "{}/{}/",
            config::host::aspn_dir(),
            self.calculate_hash()
        ));
        // make sure the dir exists
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    pub fn connect(&self) -> Result<()> {
        config::project::save_project_connnection(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Developer {
    pub id: i32,
    pub name: String,
    pub project_id: Option<i32>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDeveloper {
    pub name: String,
    pub project_id: i32,
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub id: i32,
    pub gcs_uri: String,
    pub route: String,
    pub project_id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewFunction {
    pub gcs_uri: String,
    pub route: String,
    pub project_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub id: i32,
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
    pub is_online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewHost {
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostsFunctions {
    pub function_id: i32,
    pub host_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewHostFunctionIDs {
    pub function_id: i32,
    pub host_id: i32,
}
