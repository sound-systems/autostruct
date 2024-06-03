/*!
The `postgres` module provides an implementation of the `ColumnFetcher` trait for PostgreSQL databases.
*/

use crate::database::TableInfoProvider;
use anyhow::{Context, Error};
use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres};

use super::{table_column::TableColumn, table_info_provider::Converter, TableInfo};

// A builder for configuring and creating a `Database` connection.
pub struct Builder {
    /// The schema to use for the database connection.
    schema: Option<String>,
    /// A list of tables to exclude from the database connection.
    excluded_tables: Vec<String>,
}

impl Builder {
    /// Creates a new `Builder` instance.
    pub fn new() -> Self {
        Self {
            schema: None,
            excluded_tables: Vec::new(),
        }
    }

    /// Excludes the specified tables from the database connection.
    ///
    /// # Arguments
    ///
    /// * `tables` - A vector of table names to exclude.
    ///
    /// # Returns
    ///
    /// A `Builder` instance with the specified tables excluded.
    pub fn exclude(mut self, tables: Vec<String>) -> Self {
        self.excluded_tables = tables;
        self
    }

    /// Sets the schema to use for the database connection.
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema name to use.
    ///
    /// # Returns
    ///
    /// A `Builder` instance with the specified schema.
    pub fn table_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    /// Builds the `Database` and established a connection with the specified configurations.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The connection string for the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Database` instance or an error.
    pub async fn connect(self, connection_string: String) -> Result<impl TableInfoProvider, Error> {
        let pool = PgPool::connect(&connection_string)
            .await
            .context("failed to connect to postgresql database")?;

        let db = Database {
            pool,
            excluded_tables: self.excluded_tables,
            schema: self.schema.map_or(String::from("public"), |v| v),
        };

        Ok(db)
    }
}

/**
    Represents a connection to a PostgreSQL database with various options used to meaningully
    implement the `ColumnFetcher` trait
*/
pub struct Database {
    schema: String,
    excluded_tables: Vec<String>,
    pool: Pool<Postgres>,
}

#[async_trait]
impl TableInfoProvider for Database {
    /**
    Retrieves a list of columns for all tables in the PostgreSQL database.

    # Returns
    - A `Result` containing a vector of `TableColumn` structs or an error.
    */
    async fn get_table_info(&self) -> Result<Vec<TableInfo>, Error> {
        let excluded_tables = self.excluded_tables.join(",");
        let query = "
    SELECT
        c.table_name,
        c.column_name,
        c.udt_name,
        c.data_type,
        c.is_nullable = 'YES' AS is_nullable,
        COALESCE(tc.constraint_type = 'UNIQUE', false) AS is_unique,
        COALESCE(tc.constraint_type = 'PRIMARY KEY', false) AS is_primary_key,
        kcu2.table_name AS foreign_key_table,
        kcu2.column_name AS foreign_key_id,
        c.table_schema
    FROM
        information_schema.columns c
        LEFT JOIN information_schema.key_column_usage kcu
            ON c.table_name = kcu.table_name
            AND c.column_name = kcu.column_name
            AND c.table_schema = kcu.table_schema
        LEFT JOIN information_schema.table_constraints tc
            ON kcu.constraint_name = tc.constraint_name
            AND kcu.table_schema = tc.table_schema
        LEFT JOIN information_schema.referential_constraints rc
            ON kcu.constraint_name = rc.constraint_name
        LEFT JOIN information_schema.key_column_usage kcu2
            ON rc.unique_constraint_name = kcu2.constraint_name
            AND kcu2.ordinal_position = kcu.ordinal_position
            AND kcu2.table_schema = rc.unique_constraint_schema
    WHERE
        c.table_schema = $1
        AND c.table_name NOT IN ($2)

    ORDER BY
        c.table_name,
        c.ordinal_position;";

        let tables = sqlx::query_as::<_, TableColumn>(query)
            .bind(&self.schema)
            .bind(excluded_tables)
            .fetch_all(&self.pool)
            .await?
            .to_table_info();

        Ok(tables)
    }
}
