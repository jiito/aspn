use aspn::{commands, config};
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Init {},
    Config {},
    Auth {},
    Host {
        #[command(subcommand)]
        command: Option<commands::Host>,
    },
    Developer {
        #[command(subcommand)]
        command: Option<commands::Developer>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Host { command }) => match command {
            Some(commands::Host::Start {}) => commands::host::start().await,
            None => {
                println!("No files provided!")
            }
        },
        Some(Commands::Developer { command }) => match command {
            Some(commands::Developer::Upload {}) => {
                commands::developer::upload().await.unwrap();
                println!("Phew... everything worked as promised! 🚀 ")
            }
            None => {
                println!("No command provided!")
            }
        },
        Some(Commands::Config {}) => {
            config::host::read_config();
        }
        Some(Commands::Init {}) => commands::init(),
        Some(Commands::Auth {}) => commands::auth().await,
        None => {}
    }
}
