use std::error::Error;

use anyhow::Context;
use autostruct::{
    cli::{Cli, Commands},
    generator,
};
use clap::Parser;
use sqlx::{migrate::Migrator, PgPool};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

static MIGRATOR: Migrator = sqlx::migrate!("tests/postgres/migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            let mut args: generator::Arguments = args.try_into()?;

            // setup postgres with relevant migrations
            // startup the module
            let node = Postgres::default()
                .start()
                .await
                .context("postgres container did not start up ok")?;

            let port = node.get_host_port_ipv4(5432).await.context(
                "port that the postgres docker image is listening is not available or discoverable",
            )?;
        
            // prepare connection string
            let url = &format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

            let pool = PgPool::connect(url).await.context(
                "postgres connection pool failed to establish connection with test container",
            )?;

            MIGRATOR
                .run(&pool)
                .await
                .context("migrations used for testing purposes failed to apply")?;

            args.connection_string = url.to_string();

            autostruct::generator::run(args)
                .await
                .context("autostruct generator failed")?;
        }
    };

    Ok(())
}
