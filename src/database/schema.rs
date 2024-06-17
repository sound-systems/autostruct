use anyhow::Error;
use async_trait::async_trait;

use crate::rust;

use super::raw_schema::TableColumn;

#[derive(Debug)]
pub struct DatabaseSchema {
    enumerations: Vec<Enum>,
    composite_types: Vec<CompositeType>,
    tables: Vec<Table>,
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

#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub order: i64,
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

impl From<TableColumn> for Column {
    fn from(val: TableColumn) -> Self {
        Column {
            name: val.column_name,
            udt_name: val.udt_name,
            data_type: val.data_type,
            is_nullable: val.is_nullable,
            is_unique: val.is_unique,
            is_primary_key: val.is_primary_key,
            foreign_key_table: val.foreign_key_table,
            foreign_key_id: val.foreign_key_id,
            table_schema: val.table_schema,
        }
    }
}

/**
The `schema::InfoProvider` trait defines a common interface for retrieving table column information from a database.

# Methods
- `type_name_from`: returns the Rust type name from database column info
- `get_table_info`: Asynchronously retrieves a list of `TableColumn` structs representing the columns in the database's tables.
*/
#[async_trait]
pub trait InfoProvider {
    fn type_name_from(&self, column: &Column) -> rust::Type;
    async fn get_schema(&self) -> Result<DatabaseSchema, Error>;
    async fn get_table_info(&self) -> Result<Vec<Table>, Error>;
}
