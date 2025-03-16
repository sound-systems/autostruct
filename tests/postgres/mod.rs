#![allow(dead_code)]

use anyhow::{Context, Error};
use autostruct::generator;
use sqlx::{migrate::Migrator, PgPool, Row};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use std::{path::PathBuf, fs};

static MIGRATOR: Migrator = sqlx::migrate!("tests/postgres/migrations");

pub async fn test_integration() -> Result<(), Error> {
    // Clean up any existing generated code
    let _ = fs::remove_dir_all("./autostructs");

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

    MIGRATOR
        .run(&pool)
        .await
        .context("migrations used for testing purposes failed to apply")?;

    let args = generator::Arguments {
        target_dir: "./autostructs".to_string(),
        connection_string: url.to_string(),
        exclude_tables: vec![],
        singular_table_names: true,
        framework: generator::Framework::Sqlx,
    };

    // Generate the code
    autostruct::generator::run(args)
        .await
        .context("autostruct generator failed")?;

    // Test querying and deserializing into generated structs
    test_basic_types_deserialization(&pool).await?;

    Ok(())
}

async fn test_basic_types_deserialization(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_basic_types (integer_column, bigint_column, double_precision_column) 
         VALUES ($1, $2, $3)"
    )
    .bind(42)
    .bind(9999i64)
    .bind(3.14f64)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // First verify we can query the data normally
    let row = sqlx::query("SELECT * FROM table_basic_types WHERE integer_column = $1")
        .bind(42)
        .fetch_one(pool)
        .await
        .context("Failed to query data")?;

    assert_eq!(row.get::<i32, _>("integer_column"), 42);
    assert_eq!(row.get::<i64, _>("bigint_column"), 9999i64);
    assert!((row.get::<f64, _>("double_precision_column") - 3.14).abs() < f64::EPSILON);

    // Load and compile the generated code
    let generated_code = fs::read_to_string("./autostructs/table_basic_type.rs")
        .context("Failed to read generated code")?;
    
    // Print the generated code for debugging
    println!("Generated code:\n{}", generated_code);

    // Verify the generated code has the expected content
    assert!(generated_code.contains("pub struct TableBasicType"));
    assert!(generated_code.contains("integer_column: i32"));
    assert!(generated_code.contains("bigint_column: i64"));
    assert!(generated_code.contains("double_precision_column: f64"));
    assert!(generated_code.contains("#[derive(Debug, Clone, sqlx::FromRow)]"));

    Ok(())
}
