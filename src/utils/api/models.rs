use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use anyhow::Result;
use serde::Serialize;

use crate::config;

#[derive(Debug, Serialize, Hash)]
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
