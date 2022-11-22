use anyhow::Result;
use diesel::prelude::*;
use diesel::upsert::*;
use diesel::PgConnection;

use crate::models;

pub fn save(
    conn: &mut PgConnection,
    new_function: &models::NewFunction,
) -> Result<models::Function> {
    use crate::schema::functions;

    let function = diesel::insert_into(functions::table)
        .values(new_function)
        .get_result::<models::Function>(conn)?;

    Ok(function)
}

pub fn find_by_id(conn: &mut PgConnection, id: &i32) -> Result<models::Function> {
    use crate::schema::functions::dsl;

    let query = dsl::functions.filter(crate::schema::functions::id.eq(id));

    let developer = query.first::<models::Function>(conn)?;

    Ok(developer)
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
