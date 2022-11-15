use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    project: ProjectConfig,
    developer: DeveloperConfig,
    service: ServiceConfig,
}

#[derive(Deserialize, Serialize, Debug)]
struct ProjectConfig {
    name: String,
    url: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct DeveloperConfig {
    email: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct ServiceConfig {
    entrypoint: String,
    build: String,
}

pub fn parse_config() {
    let f = std::fs::File::open("./aspn.yaml").expect("Could not open file");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values");

    println!("{:?}", scrape_config);
}
