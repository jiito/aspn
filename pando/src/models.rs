use diesel::prelude::*;

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
}
