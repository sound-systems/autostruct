/*!
The `database` module provides a common interface for interacting with various database systems to retrieve table column information.
It defines the `Database` trait that must be implemented by all supported database systems and provides the `TableColumn` struct to
represent column information.

Supported database systems include:
- PostgreSQL
- MySQL
- MSSQL
- SQLite

Each supported database has its own module implementing the `Database` trait.
*/

pub mod mssql;
pub mod mysql;
pub mod postgres;
pub mod sqlite;

mod convert;
mod raw_schema;

mod schema;
pub use schema::{Column, CompositeType, Enum, InfoProvider, Table};

use anyhow::{bail, Error};

/**
The Kind of databases that autostruct supports
*/
pub enum Kind {
    Postgres,
    MySQL,
    MSSQL,
    Sqlite,
}

impl TryFrom<&str> for Kind {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let db = if value.starts_with("postgres://") {
            Self::Postgres
        } else {
            bail!("failed to infer database kind from provided connection string")
        };

        Ok(db)
    }
}
