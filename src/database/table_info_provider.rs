use anyhow::Error;
use async_trait::async_trait;

/**
Contains info describing a table in a database

# Fields
- `name`: The name of the table.
- `columns`: The columns of the table
*/
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

/**
The `Converter` trait defines a common interface for converting types into a vector of TableInfo
*/
pub trait Converter {
    fn to_table_info(self) -> Vec<TableInfo>;
}

/**
The `TableInfoProvider` trait defines a common interface for retrieving table column information from a database.

# Methods
- `get_table_columns`: Asynchronously retrieves a list of `TableColumn` structs representing the columns in the database's tables.
*/
#[async_trait]
pub trait TableInfoProvider {
    async fn get_table_info(&self) -> Result<Vec<TableInfo>, Error>;
}
