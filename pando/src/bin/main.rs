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
        #[command(subcommand)]
        command: Option<HostCommands>,
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
#[derive(Subcommand, Debug)]
enum HostCommands {
    Start {},
}
fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Host { command }) => match command {
            Some(HostCommands::Start {}) => {
                println!("Starting the ASPN cloud")
            }
            None => {
                println!("No files provided!")
            }
        },
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