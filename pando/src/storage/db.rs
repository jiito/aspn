pub mod functions;
pub mod hosts;
use crate::models::{Developer, NewDeveloper, NewProject, Project};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::upsert::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_project(conn: &mut PgConnection, name: &str) -> Project {
    use crate::schema::projects;

    let new_project = NewProject { name };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
        .expect("Error saving project")
}
pub fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
    use crate::schema::projects::dsl;

    let connection = &mut establish_connection();

    let results = dsl::projects
        .load::<Project>(conn)
        .expect("Error loeading post");

    results
}
pub fn find_project(conn: &mut PgConnection, id: &i32) -> Project {
    use crate::schema::projects::dsl;

    let query = dsl::projects.filter(crate::schema::projects::id.eq(id));

    let project = query.first::<Project>(conn).expect("Can't find project");

    project
}

pub fn upsert_developer(conn: &mut PgConnection, developer: &NewDeveloper) -> usize {
    use crate::schema::developers;

    diesel::insert_into(developers::table)
        .values(developer)
        .on_conflict(developers::id)
        .do_update()
        .set(developers::project_id.eq(0))
        .execute(conn)
        .expect("Could not update the developer")
}

pub fn find_developer(conn: &mut PgConnection, id: &i32) -> Developer {
    use crate::schema::developers::dsl;

    let query = dsl::developers.filter(crate::schema::developers::id.eq(id));

    let developer = query
        .first::<Developer>(conn)
        .expect("Can't find developer");

    developer
}
