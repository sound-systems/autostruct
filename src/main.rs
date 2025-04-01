use std::error::Error;

use autostruct::generator;
use clap::Parser;
use cli::{Cli, Commands};
use dotenvy::dotenv;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenv();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            let args: generator::Arguments = args.try_into()?;
            generator::run(args).await?;
        }
    };

    Ok(())
}
