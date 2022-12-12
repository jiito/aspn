use anyhow::Result;

use crate::{utils, with_loader};

pub async fn upload() -> Result<()> {
    with_loader!(utils::api::upload().await, "Uploading your precious goods")
}
