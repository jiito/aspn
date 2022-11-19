use serde::{Deserialize, Serialize};

use crate::models;

use super::{
    dev::ProjectConfig,
    host::{aspn_dir, read_config, update_config, ConfigData},
};

#[derive(Deserialize, Serialize, Hash)]
pub struct ProjectData {
    id: i32,
    pub name: String,
    pub path: std::path::PathBuf,
}

pub fn save_project_connnection(project: models::Project) {
    let project_hash = project.calculate_hash::<String>();
    let new_config = ConfigData {
        host: None,
        project: Some(ProjectData {
            id: project.id,
            name: project.name.clone(),
            path: std::path::PathBuf::from(format!("{}/{}/", aspn_dir(), project_hash)),
        }),
    };
    update_config(new_config)
}

pub fn read_project_connection() -> Option<ProjectData> {
    let config = read_config();
    config.project
}

fn calculate_hash() {}
