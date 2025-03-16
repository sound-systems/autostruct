#[cfg(feature = "postgres_test")]
mod postgres;

#[cfg(feature = "postgres_test")]
#[tokio::test]
async fn test_postgres() -> Result<(), Box<dyn std::error::Error>> {
    postgres::test_integration().await?;
    Ok(())
}
