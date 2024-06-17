use std::error::Error;

use anyhow::Context;
use autostruct::generator;
use clap::Parser;
use cli::{Cli, Commands};
use dotenvy::dotenv;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().context("failed to load environment variables")?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            let args: generator::Arguments = args.try_into()?;
            generator::run(args).await?;
        }
    };

    Ok(())
}
