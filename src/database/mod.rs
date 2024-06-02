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

use anyhow::{bail, Error};
use async_trait::async_trait;

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
            bail!("the database you are trying to connect to is not yet supported")
        };

        Ok(db)
    }
}

/**
Represents a column in a database table.

# Fields
- `table_name`: The name of the table.
- `column_name`: The name of the column.
- `udt_name`: The underlying data type name of the column.
- `data_type`: The data type of the column.
- `is_nullable`: Whether the column can contain NULL values.
- `is_unique`: Whether the column has a unique constraint.
- `is_primary_key`: Whether the column is a primary key.
- `foreign_key_table`: The table that this column references if it is a foreign key.
- `foreign_key_id`: The column that this column references if it is a foreign key.
- `table_schema`: The schema of the table.
*/
#[derive(sqlx::FromRow, Debug)]
pub struct TableColumn {
    pub table_name: String,
    pub column_name: String,
    pub udt_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_unique: bool,
    pub is_primary_key: bool,
    pub foreign_key_table: Option<String>,
    pub foreign_key_id: Option<String>,
    pub table_schema: String,
}

/**
The `ColumnFetcher` trait defines a common interface for retrieving table column information from a database.

# Methods
- `get_table_columns`: Asynchronously retrieves a list of `TableColumn` structs representing the columns in the database's tables.
*/
#[async_trait]
pub trait ColumnFetcher {
    async fn get_table_columns(&self) -> Result<Vec<TableColumn>, Error>;
}
