use anyhow::Result;

use crate::storage::{self, db::establish_connection};

pub fn online() -> Result<()> {
    // used for marking a host online
    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_current_host()?;
    host.online(conn)
}
pub fn offline() -> Result<()> {
    // to be called when a host kills their program
    let conn = &mut establish_connection();
    let host = storage::db::hosts::find_current_host()?;
    host.offline(conn)
}
