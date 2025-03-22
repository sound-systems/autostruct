use sqlx::migrate::Migrator;


pub static POSTGRES_MIGRATOR: Migrator = sqlx::migrate!("tests/postgres/migrations");