use diesel::prelude::*;
use pando::models::*;
use pando::*;

fn main() {
    fn get_projects() {
        use self::schema::projects::dsl::*;

        let connection = &mut establish_connection();

        let results = projects
            .load::<Project>(connection)
            .expect("Error loeading post");

        println!("Displaying {} project", results.len());

        for project in results {
            println!("{}", project.name)
        }
    }

    get_projects()
}
