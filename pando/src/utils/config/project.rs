use serde::{Deserialize, Serialize};

use crate::models;

use super::host::{aspn_dir, update_config, ConfigData};

#[derive(Deserialize, Serialize)]
pub struct ProjectData {
    id: i32,
    name: String,
    path: std::path::PathBuf,
}
pub fn save_project_connnection(project: models::Project) {
    let new_config = ConfigData {
        host: None,
        project: Some(ProjectData {
            id: project.id,
            name: project.name,
            path: std::path::PathBuf::from(format!("{}/{}/", aspn_dir(), project.name)),
        }),
    };
    update_config(new_config)
}
