use crate::schema::projects;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
}
