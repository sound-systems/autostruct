/*!
The `postgres` module provides an implementation of the `TableInfoProvider` trait for PostgreSQL databases.
*/

use crate::database::TableInfoProvider;
use anyhow::{Context, Error};
use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres};

use super::{
    table_column::{Converter, TableColumn},
    table_info_provider::TypeGetter,
    TableInfo,
};

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
    implement the `TableInfoProvider` trait
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
    - A `Result` containing a vector of `TableInfo` structs or an error.
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

pub struct PgTypeGetter();

impl PgTypeGetter {
    pub fn new() -> Self {
        return PgTypeGetter {};
    }
}

impl TypeGetter for PgTypeGetter {
    fn get_rust_type(&self, column: super::table_info_provider::ColumnInfo) -> String {
        let rust_type = match column.data_type.as_str() {
            "bool" => "bool",
            "char" => "i8",
            "smallint" | "smallserial" | "int2" => "i16",
            "int" | "serial" | "int4" => "i32",
            "bigint" | "bigserial" | "int8" => "i64",
            "real" | "float4" => "f32",
            "double precision" | "float8" => "f64",
            "varchar" | "char(n)" | "text" | "name" | "citext" => "String",
            "bytea" => "Vec<u8>",
            "void" => "()",
            // assuming [`uuid`](https://crates.io/crates/uuid)
            "uuid" => "uuid::Uuid",
            // assuming [`chrono`](https://crates.io/crates/chrono) for time based types
            "date" => "chrono::NaiveDate",
            "time" => "chrono::NaiveTime",
            "timestamp" => "chrono::NaiveDateTime",
            "timestamptz" => "chrono::DateTime<Utc>",
            // assuming [`rust_decimal`](https://crates.io/crates/rust_decimal) to support numeric types
            "numeric" => "rust_decimal::Decimal",
            // assuming [`ipnetwork`](https://crates.io/crates/ipnetwork)
            "inet" | "cidr" => "ipnetwork::IpNetwork",
            // assuming [`bit-vec`](https://crates.io/crates/bit-vec)
            "bit" | "varbit" => "bit_vec::BitVec",
            // below types are biased towards using the sqlx::postgres::types module
            // this should be considered for configuration when autostruct explicitly supports
            // different rust postgres clients
            "interval" => "PgInterval",
            "int4range" => "PgRange<i32>",
            "int8range" => "PgRange<i64>",
            "tsrange" => "PgRange<chrono::NaiveDateTime>",
            "tstzrange" => "PgRange<chrono::DateTime<Utc>>",
            "daterange" => "PgRange<chrono::NaiveDate>",
            "numrange" => "PgRange<rust_decimal::Decimal>",
            "money" => "PgMoney",
            "ltree" => "PgLTree",
            "lquery" => "PgLQuery",
            _ => "unkown",
        };

        if column.is_nullable {
            return format!("Option<{}>", rust_type);
        }
        rust_type.to_string()
    }
}
