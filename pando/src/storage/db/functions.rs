use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::upsert::*;
use diesel::PgConnection;

use crate::models;
use crate::storage;
use crate::utils;

pub fn save(
    conn: &mut PgConnection,
    new_function: &models::NewFunction,
) -> Result<models::Function> {
    use crate::schema::functions;

    println!("{:?}", new_function.project_id);

    let function = diesel::insert_into(functions::table)
        .values(new_function)
        .get_result::<models::Function>(conn)
        .with_context(|| "failed to save function")?;
    println!("Saved the func dawg");

    Ok(function)
}

pub fn save_w_host(
    conn: &mut PgConnection,
    new_function: &models::NewFunction,
    host: &models::Host,
) -> Result<models::Function> {
    let function = save(conn, new_function)?;
    storage::db::hosts::connect_host_to_function(conn, host, &function)?;

    Ok(function)
}
pub fn find_by_id(conn: &mut PgConnection, id: &i32) -> Result<models::Function> {
    use crate::schema::functions::dsl;

    let query = dsl::functions.filter(crate::schema::functions::id.eq(id));

    let function = query.first::<models::Function>(conn)?;

    Ok(function)
}

pub fn find_by_project(conn: &mut PgConnection, project_id: &i32) -> Result<models::Function> {
    use crate::schema::functions::dsl;

    let query = dsl::functions.filter(crate::schema::functions::project_id.eq(project_id));

    // TODO: Eliminate first usage and return all functions
    let functions = query.first::<models::Function>(conn)?;

    Ok(functions)
}

pub fn upsert(conn: &mut PgConnection, function: &models::NewFunction) -> Result<usize> {
    use crate::schema::functions;

    let res = diesel::insert_into(functions::table)
        .values(function)
        .on_conflict(functions::id)
        .do_update()
        .set(functions::project_id.eq(0))
        .execute(conn)?;

    Ok(res)
}
