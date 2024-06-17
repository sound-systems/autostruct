use anyhow::Error;
use async_trait::async_trait;

use crate::rust;

#[derive(Debug)]
pub struct DatabaseSchema {
    pub enumerations: Vec<Enum>,
    pub composite_types: Vec<CompositeType>,
    pub tables: Vec<Table>,
}

/**
Contains info describing a user defined enumeration in a database

# Fields
- `name`: The name of the enum.
- `values`: The values of the enumeration
*/
#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

/**
Contains info describing a user defined enum value in a database

# Fields
- `name`: The name of the enum value.
- `order`: The order in which the respective value should be sorted
*/
#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub order: f32,
}

/**
Contains info describing a user defined composite type in a database

# Fields
- `name`: The name of the composite type.
- `attributes`: The attributes of the composite type
*/
#[derive(Debug)]
pub struct CompositeType {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub data_type: String,
}

/**
Contains info describing a table in a database

# Fields
- `name`: The name of the table.
- `columns`: The columns of the table
*/
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

/**
Contains info describing a column in a database table.

# Fields
- `name`: The name of the column.
- `udt_name`: The underlying data type name of the column.
- `data_type`: The data type of the column.
- `is_nullable`: Whether the column can contain NULL values.
- `is_unique`: Whether the column has a unique constraint.
- `is_primary_key`: Whether the column is a primary key.
- `foreign_key_table`: The table that this column references if it is a foreign key.
- `foreign_key_id`: The column that this column references if it is a foreign key.
- `table_schema`: The schema of the table.
*/
#[derive(Debug)]
pub struct Column {
    pub name: String,
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
The `schema::InfoProvider` trait defines a common interface for retrieving database schema information from a database.

# Methods
- `type_name_from`: returns the Rust type name from database column info
- `get_table_info`: Asynchronously retrieves a list of `TableColumn` structs representing the columns in the database's tables.
*/
#[async_trait]
pub trait InfoProvider {
    fn type_name_from(&self, db_type: &str) -> rust::Type;
    async fn get_schema(&self) -> Result<DatabaseSchema, Error>;
}
