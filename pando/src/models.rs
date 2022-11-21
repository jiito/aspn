use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Queryable, Identifiable, Debug, Serialize, Hash)]
pub struct Project {
    pub id: i32,
    pub name: String,
}
impl Project {
    pub fn calculate_hash<Project>(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish().to_string()
    }
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Project)]
#[diesel(table_name = developers)]
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

#[derive(Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Project)]
#[diesel(table_name = functions)]
pub struct Function {
    pub id: i32,
    pub ref_name: String,
    pub route: String,
    pub project_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = functions)]
pub struct NewFunction {
    pub ref_name: String,
    pub route: String,
    pub project_id: i32,
}

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = hosts)]
pub struct Host {
    pub id: i32,
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
}

#[derive(Insertable)]
#[diesel(table_name = hosts)]
pub struct NewHost {
    pub ip_address: ipnetwork::IpNetwork,
    pub user_token: String,
}

#[derive(Associations, Queryable, Identifiable, Debug)]
#[diesel(table_name = hosts_functions)]
#[belongs_to(Host)]
#[belongs_to(Function)]
pub struct HostsFunctions {
    pub id: i32,
    pub function_id: i32,
    pub host_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = hosts_functions)]
pub struct NewHostFunctionIDs {
    pub function_id: i32,
    pub host_id: i32,
}
