/*!
The `postgres` module provides an implementation of the `TableInfoProvider` trait for PostgreSQL databases.
*/

use crate::{
    database::InfoProvider,
    rust::{self, Type},
};
use anyhow::{Context, Error};
use async_trait::async_trait;
use sqlx::{PgPool, Pool, Postgres};

use super::{
    convert::{CompositeTypeConverter, EnumConverter, TableConverter},
    raw_schema::{self, TableColumn},
    schema::{self, DatabaseSchema, Enum},
    Table,
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

    /// Builds the `Database` and establishes a connection with the specified configurations.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The connection string for the database.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Database` instance or an error.
    pub async fn connect(self, connection_string: &str) -> Result<impl InfoProvider, Error> {
        let pool = PgPool::connect(connection_string)
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
    pool: Pool<Postgres>,
    schema: String,
    excluded_tables: Vec<String>,
}

impl Database {
    async fn get_enums(&self) -> Result<Vec<Enum>, Error> {
        let query = "
        SELECT
            n.nspname AS schema_name,
            t.typname AS name,
            e.enumlabel AS value,
            e.enumsortorder AS sort_order
        FROM
            pg_type t
        JOIN
            pg_namespace n ON t.typnamespace = n.oid
        JOIN
            pg_enum e ON t.oid = e.enumtypid
        WHERE
            n.nspname = $1
        ORDER BY
            schema_name, t.typname, e.enumsortorder;";

        let enums = sqlx::query_as::<_, raw_schema::EnumType>(query)
            .bind(&self.schema)
            .fetch_all(&self.pool)
            .await?
            .to_enums();

        Ok(enums)
    }

    async fn get_composite_types(&self) -> Result<Vec<schema::CompositeType>, Error> {
        let query = "
        SELECT
            n.nspname AS schema_name,
            t.typname AS name,
            a.attname AS attribute_name,
            bt.typname AS data_type,
            a.attnum AS attribute_position
        FROM
            pg_type t
        JOIN
            pg_namespace n ON t.typnamespace = n.oid
        JOIN
            pg_class c ON t.typrelid = c.oid
        JOIN
            pg_attribute a ON c.oid = a.attrelid
        JOIN
            pg_type bt ON a.atttypid = bt.oid
        WHERE
            t.typtype = 'c'
            AND c.relkind = 'c'
            AND a.attnum > 0
            AND n.nspname = $1
        ORDER BY
            schema_name, t.typname, a.attnum;";

        let composite_types = sqlx::query_as::<_, raw_schema::CompositeType>(query)
            .bind(&self.schema)
            .fetch_all(&self.pool)
            .await?
            .to_composite_types();

        Ok(composite_types)
    }

    /**
    Retrieves a list of columns for all tables in the PostgreSQL database.

    # Returns
    - A `Result` containing a vector of `TableInfo` structs or an error.
    */
    async fn get_table_info(&self) -> Result<Vec<Table>, Error> {
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
            .to_tables();

        Ok(tables)
    }
}

#[async_trait]
impl InfoProvider for Database {

    fn type_name_from(&self, db_type: &str) -> rust::Type {
        // Handle arrays first
        if let Some(inner_type) = db_type.strip_prefix('_') {
            return Type::Vector(Box::new(self.type_name_from(inner_type)));
        }

        match db_type {
            t if NUMERIC_TYPES.contains(&t) => map_numeric_type(t),
            t if TEMPORAL_TYPES.contains(&t) => map_temporal_type(t),
            t if STRING_TYPES.contains(&t) => Type::String("String"),
            t if BINARY_TYPES.contains(&t) => Type::ByteArray("Vec<u8>"),
            t => map_specialized_type(t),
        }
    }

    async fn get_schema(&self) -> Result<DatabaseSchema, Error> {
        let enumerations = self.get_enums().await?;
        let composite_types = self.get_composite_types().await?;
        let tables = self.get_table_info().await?;
        let schema = DatabaseSchema {
            enumerations,
            composite_types,
            tables,
        };
        Ok(schema)
    }
}


// Constants for type categorization
const NUMERIC_TYPES: &[&str] = &[
    "bool", "boolean", "smallint", "smallserial", "int2", "int", "integer",
    "serial", "int4", "bigint", "bigserial", "int8", "numeric", "decimal",
    "real", "float4", "double precision", "float8",
];

const TEMPORAL_TYPES: &[&str] = &[
    "date", "time", "time without time zone", "timestamp",
    "timestamp without time zone", "timestamp with time zone", "timestamptz",
];

const STRING_TYPES: &[&str] = &[
    "varchar", "text", "name", "character varying", "character", "citext",
    "bpchar", "bit", "varbit",
];

const BINARY_TYPES: &[&str] = &["bytea"];

fn map_numeric_type(typ: &str) -> rust::Type {
    match typ {
        "bool" | "boolean" => Type::Bool("bool"),
        "smallint" | "smallserial" | "int2" => Type::I16("i16"),
        "int" | "integer" | "serial" | "int4" => Type::I32("i32"),
        "bigint" | "bigserial" | "int8" => Type::I64("i64"),
        "numeric" | "decimal" => Type::Decimal("rust_decimal::Decimal"),
        "real" | "float4" => Type::F32("f32"),
        "double precision" | "float8" => Type::F64("f64"),
        _ => unreachable!("invalid numeric type"),
    }
}

fn map_temporal_type(typ: &str) -> rust::Type {
    match typ {
        "date" => Type::Date("chrono::NaiveDate"),
        "time" | "time without time zone" => Type::Time("chrono::NaiveTime"),
        "timestamp" | "timestamp without time zone" => Type::Timestamp("chrono::NaiveDateTime"),
        "timestamp with time zone" | "timestamptz" => Type::TimestampWithTz("chrono::DateTime<chrono::Utc>"),
        _ => unreachable!("invalid temporal type"),
    }
}

fn map_specialized_type(typ: &str) -> rust::Type {
    match typ {
        "uuid" => Type::Uuid("uuid::Uuid"),
        "json" | "jsonb" => Type::Json("serde_json::Value"),
        "inet" | "cidr" => Type::IpNetwork("ipnetwork::IpNetwork"),
        "xml" => Type::Xml("String"),
        // Add other specialized types here
        other => Type::Custom(other.to_string()),
    }
}