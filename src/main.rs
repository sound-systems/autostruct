use clap::Parser;
use cli::{Cli, Commands};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => println!("setting up project with {:#?}", args),
    }
}
