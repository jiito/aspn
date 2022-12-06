use anyhow::Result;
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
async fn main() -> Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Host { command }) => match command {
            Some(commands::Host::Start {}) => commands::host::start().await?,
            Some(commands::Host::Auth {}) => commands::auth(commands::UserType::Host).await?,
            None => {
                println!("No files provided!")
            }
        },
        Some(Commands::Developer { command }) => match command {
            Some(commands::Developer::Upload {}) => {
                commands::developer::upload().await?;
                println!("Phew... everything worked as promised! 🚀 ")
            }
            Some(commands::Developer::Auth {}) => commands::auth(commands::UserType::Dev).await?,
            None => {
                println!("No command provided!")
            }
        },
        Some(Commands::Config {}) => {
            config::host::read_config();
        }
        Some(Commands::Init {}) => commands::init().await?,
        None => {}
    }
    Ok(())
}
