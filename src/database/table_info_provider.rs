use anyhow::Error;
use async_trait::async_trait;

use crate::rust;

use super::table_column::TableColumn;

/**
Contains info describing a table in a database

# Fields
- `name`: The name of the table.
- `columns`: The columns of the table
*/
#[derive(Debug)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
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
pub struct ColumnInfo {
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

impl From<TableColumn> for ColumnInfo {
    fn from(val: TableColumn) -> Self {
        ColumnInfo {
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

pub type RustType = String;

/**
The `TableInfoProvider` trait defines a common interface for retrieving table column information from a database.

# Methods
- `type_name_from`: returns the Rust type name from database column info
- `get_table_columns`: Asynchronously retrieves a list of `TableColumn` structs representing the columns in the database's tables.
*/
#[async_trait]
pub trait TableInfoProvider {
    fn type_name_from(&self, column: &ColumnInfo) -> rust::Type;
    async fn get_table_info(&self) -> Result<Vec<TableInfo>, Error>;
}
