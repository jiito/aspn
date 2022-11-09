use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Host {
        #[arg(short, long)]
        host: bool,
    },
    Developer {
        #[command(subcommand)]
        command: Option<DeveloperCommands>,
    },
}

#[derive(Subcommand, Debug)]
enum DeveloperCommands {
    Upload {
        // #[arg(short, long)]
        files: std::path::PathBuf,
    },
}
fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Host { host }) => {
            if *host {
                println!("You are hosting something")
            }
        }
        Some(Commands::Developer { command }) => match command {
            Some(DeveloperCommands::Upload { files }) => {
                println!("{}", files.to_str().expect("Not a valid Path"))
            }
            None => {
                println!("No files provided!")
            }
        },
        None => {}
    }
}
