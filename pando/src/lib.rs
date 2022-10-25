pub mod models;
pub mod schema;
pub mod storage;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewProject, Project};
use std::env;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

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
