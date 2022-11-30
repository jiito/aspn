use anyhow::Result;

use crate::utils;

pub async fn upload() -> Result<()> {
    utils::api::upload().await
}
