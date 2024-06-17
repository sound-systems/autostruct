use std::{collections::HashMap, mem};

use crate::database::schema::Column;

use super::Table;

/**
The `Converter` trait defines a common interface for converting types into a vector of TableInfo
*/
pub trait Converter {
    fn to_table_info(self) -> Vec<Table>;
}

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

impl Converter for Vec<TableColumn> {
    fn to_table_info(self) -> Vec<Table> {
        let tables: HashMap<String, Vec<Column>> = HashMap::new();
        self.into_iter()
            .fold(tables, |mut acc, mut column| {
                let table_name = mem::take(&mut column.table_name);
                acc.entry(table_name).or_default().push(column.into());
                acc
            })
            .into_iter()
            .map(|t| Table {
                name: t.0,
                columns: t.1,
            })
            .collect()
    }
}
