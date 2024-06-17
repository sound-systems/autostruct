/*!
The `raw_schema` module exposes types intended to be used directly in sql queries that retrieve info from
a databases respective information schema implementation.
These are intended to be raw types that model the returned rows that are then mapped to types purposed for informing
code generation
*/

#[derive(sqlx::FromRow, Debug)]
pub struct EnumType {
    pub name: String,
    pub value: String,
    pub sort_order: i64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct CompositeType {
    pub name: String,
    pub attribute_name: String,
    pub data_type: String,
}

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
