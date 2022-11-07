use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Debug, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Project)]
#[table_name = "developers"]
pub struct Developer {
    pub id: i32,
    pub name: String,
    pub ip_address: ipnetwork::IpNetwork,
    pub project_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = developers)]
pub struct NewDeveloper {
    pub name: String,
    pub project_id: i32,
}
