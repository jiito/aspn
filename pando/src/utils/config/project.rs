use anyhow::Result;
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

pub fn save_project_connnection(project: &models::Project) -> Result<()> {
    let project_hash = project.calculate_hash();
    let new_config = ConfigData {
        host: None,
        project: Some(ProjectData {
            id: project.id,
            name: project.name.clone(),
            path: project.host_dir(),
        }),
    };
    update_config(new_config);
    Ok(())
}

pub fn read_project_connection() -> Option<ProjectData> {
    let config = read_config();
    config.project
}
