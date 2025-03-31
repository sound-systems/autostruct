#[cfg(feature = "postgres_test")]
mod postgres;

mod migrate;

#[cfg(feature = "postgres_test")]
#[tokio::test]
async fn test_postgres() -> Result<(), Box<dyn std::error::Error>> {
    postgres::test_integration().await?;
    Ok(())
}
