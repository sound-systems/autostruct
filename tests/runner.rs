mod postgres;

#[cfg(feature = "postgres_test")]
#[tokio::test]
async fn test_postgres() {
    postgres::test_integration()
        .await
        .expect("postgres integration test failed");
}
