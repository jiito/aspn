use pando::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("What's the name of the project");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    let project = create_project(connection, name);
    println!("\nSaved project {} with id {}", name, project.id);
}
