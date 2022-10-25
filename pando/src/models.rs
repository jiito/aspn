use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Identifiable)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(Project)]
#[table_name = "developers"]
pub struct Developer {
    pub id: i32,
    pub name: String,
    pub ip_address: String,
    pub project_id: i32,
}
