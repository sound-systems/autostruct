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
    schema::{self, Column, DatabaseSchema, Enum},
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

    fn type_name_from(&self, db_type: &str) -> rust::Type {
        match db_type {
            "bool" | "boolean" => Type::Bool("bool"),
            "char" => Type::I8("i8"),
            "smallint" | "smallserial" | "int2" => Type::I16("i16"),
            "int" | "integer" | "serial" | "int4" => Type::I32("i32"),
            "bigint" | "bigserial" | "int8" => Type::I64("i64"),
            "real" | "float4" => Type::F32("f32"),
            "double precision" | "float8" => Type::F64("f64"),
            "varchar" | "char(n)" | "text" | "name" | "character varying" | "character"
            | "citext" | "bpchar" => Type::String("String"),
            "bytea" => Type::ByteArray("Vec<u8>"),
            "void" => Type::Void("()"),
            // assuming [`uuid`](https://crates.io/crates/uuid)
            "uuid" => Type::UUID("uuid::Uuid"),
            // assuming [`chrono`](https://crates.io/crates/chrono) for time based types
            "date" => Type::Date("chrono::NaiveDate"),
            "time without time zone" | "time with time zone" | "time" => {
                Type::Time("chrono::NaiveTime")
            }
            "timestamp without time zone" | "timestamp" => Type::Timestamp("chrono::NaiveDateTime"),
            "timestamp with time zone" | "timestamptz" => {
                Type::TimestampWithTz("chrono::DateTime<Utc>")
            }
            // assuming [`rust_decimal`](https://crates.io/crates/rust_decimal) to support numeric types
            "numeric" => Type::Decimal("rust_decimal::Decimal"),
            // assuming [`ipnetwork`](https://crates.io/crates/ipnetwork)
            "inet" | "cidr" => Type::IpNetwork("ipnetwork::IpNetwork"),
            // assuming [`bit-vec`](https://crates.io/crates/bit-vec)
            "bit" | "varbit" | "bit varying" => Type::Bit("bit_vec::BitVec"),
            // below types are biased towards using the sqlx::postgres::types module
            // this should be considered for configuration when autostruct explicitly supports
            // different rust postgres clients
            "interval" => Type::Interval("PgInterval"),
            "int4range" => Type::Range(Box::new(Type::I32("i32"))),
            "int8range" => Type::Range(Box::new(Type::I64("i64"))),
            "tsrange" => Type::Range(Box::new(Type::Timestamp("chrono::NaiveDateTime"))),
            "tstzrange" => Type::Range(Box::new(Type::TimestampWithTz("chrono::DateTime<Utc>"))),
            "daterange" => Type::Range(Box::new(Type::Date("chrono::NaiveDate"))),
            "numrange" => Type::Range(Box::new(Type::Decimal("rust_decimal::Decimal"))),
            "money" => Type::Money("PgMoney"),
            "ltree" => Type::Tree("PgLTree"),
            "lquery" => Type::Query("PgLQuery"),
            pg_type => Type::Custom(pg_type.to_string()),
        }
    }

    // fn to_rust_type(pg_type: &str) -> Type {
    //     match pg_type {
    //          "bool" => Type::Bool,
    //          "bytea" => Type::String,
    //          "char" => Type::String,
    //          "name" => Type::String,
    //          "int8" => Type::String,
    //          "int2" => Type::String,
    //          "int4" => Type::String,
    //          "text" => Type::String,
    //          "oid" => Type::String,
    //          "json" => Type::String,
    //          "_json" => Type::String,
    //          "point" => Type::String,
    //          "lseg" => Type::String,
    //          "path" => Type::String,
    //          "box" => Type::String,
    //          "polygon" => Type::String,
    //          "line" => Type::String,
    //          "_line" => Type::String,
    //          "cidr" => Type::String,
    //          "_cidr" => Type::String,
    //          "float4" => Type::String,
    //          "float8" => Type::String,
    //          "unknown" => Type::String,
    //          "circle" => Type::String,
    //          "_circle" => Type::String,
    //          "macaddr8" => Type::String,
    //          "_macaddr8" => Type::String,
    //          "macaddr" => Type::String,
    //          "inet" => Type::String,
    //          "_bool" => Type::String,
    //          "_bytea" => Type::String,
    //          "_char" => Type::String,
    //          "_name" => Type::String,
    //          "_int2" => Type::String,
    //          "_int4" => Type::String,
    //          "_text" => Type::String,
    //          "_bpchar" => Type::String,
    //          "_varchar" => Type::String,
    //          "_int8" => Type::String,
    //          "_point" => Type::String,
    //          "_lseg" => Type::String,
    //          "_path" => Type::String,
    //          "_box" => Type::String,
    //          "_float4" => Type::String,
    //          "_float8" => Type::String,
    //          "_polygon" => Type::String,
    //          "_oid" => Type::String,
    //          "_macaddr" => Type::String,
    //          "_inet" => Type::String,
    //          "bpchar" => Type::String,
    //          "varchar" => Type::String,
    //          "date" => Type::String,
    //          "time" => Type::String,
    //          "timestamp" => Type::String,
    //          "_timestamp" => Type::String,
    //          "_date" => Type::String,
    //          "_time" => Type::String,
    //          "timestamptz" => Type::String,
    //          "_timestamptz" => Type::String,
    //          "interval" => Type::String,
    //          "_interval" => Type::String,
    //          "_numeric" => Type::String,
    //          "timetz" => Type::String,
    //          "_timetz" => Type::String,
    //          "bit" => Type::String,
    //          "_bit" => Type::String,
    //          "varbit" => Type::String,
    //          "_varbit" => Type::String,
    //          "numeric" => Type::String,
    //          "record" => Type::String,
    //          "_record" => Type::String,
    //          "uuid" => Type::String,
    //          "_uuid" => Type::String,
    //          "jsonb" => Type::String,
    //          "_jsonb" => Type::String,
    //          "int4range" => Type::String,
    //          "_int4range" => Type::String,
    //          "numrange" => Type::String,
    //          "_numrange" => Type::String,
    //          "tsrange" => Type::String,
    //          "_tsrange" => Type::String,
    //          "tstzrange" => Type::String,
    //          "_tstzrange" => Type::String,
    //          "daterange" => Type::String,
    //          "_daterange" => Type::String,
    //          "int8range" => Type::String,
    //          "_int8range" => Type::String,
    //          "jsonpath" => Type::String,
    //          "_jsonpath" => Type::String,
    //          "money" => Type::String,
    //          "_money" => Type::String,
    //          "void" => Type::String,
    //          _ => Type::Custom(pg_type.to_string())
    //     }
    // }
}
