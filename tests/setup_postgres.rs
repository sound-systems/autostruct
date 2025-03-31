pub mod migrate;

use std::time::Duration;

use anyhow::Context;
use autostruct::generator;
use migrate::POSTGRES_MIGRATOR;
use sqlx::PgPool;
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

#[tokio::test]
async fn setup_and_check() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("connecting to postgres at {url}");

    let pool = PgPool::connect(url)
        .await
        .context("postgres connection pool failed to establish connection with test container")?;

    POSTGRES_MIGRATOR
        .run(&pool)
        .await
        .context("migrations used for testing purposes failed to apply")?;

    let args = generator::Arguments {
        target_dir: "tests/postgres/autostructs".to_string(),
        connection_string: url.to_string(),
        exclude_tables: vec![],
        singular_table_names: true,
        framework: generator::Framework::Sqlx,
        timeout: Duration::from_secs(3),
    };

    // Generate the code
    autostruct::generator::run(args)
        .await
        .context("autostruct generator failed")?;

    // Validate that the autostructs folder has been generated
    let autostructs_path = std::path::Path::new("tests/postgres/autostructs");
    assert!(
        autostructs_path.exists(),
        "Autostructs folder was not generated"
    );
    assert!(
        autostructs_path.is_dir(),
        "Autostructs path is not a directory"
    );

    // Check that the folder contains files
    let entries =
        std::fs::read_dir(autostructs_path).context("Failed to read autostructs directory")?;

    let files: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();

    // Ensure we have at least one file generated
    assert_eq!(
        files.len(),
        24,
        "An incorrect amount of files were generated"
    );

    Ok(())
}
