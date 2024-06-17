/*!
The `convert` module provides various implementations to map raw schema types into types that provide non redundant and necessary context
a consuming module intended to generate Rust code requires
*/

use std::{collections::HashMap, mem};

use super::{
    raw_schema::{EnumType, TableColumn},
    schema::{Column, Enum, EnumValue},
    Table,
};

/**
The `TableConverter` trait defines a common interface for converting types into a vector of TableInfo
*/
pub trait TableConverter {
    fn to_tables(self) -> Vec<Table>;
}

impl TableConverter for Vec<TableColumn> {
    fn to_tables(self) -> Vec<Table> {
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

/// Converts a `raw_schema::TableColumn` to a `schema::Column`
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

pub trait EnumConverter {
    fn to_enums(self) -> Vec<Enum>;
}

impl EnumConverter for Vec<EnumType> {
    fn to_enums(self) -> Vec<Enum> {
        let enumerations: HashMap<String, Vec<EnumValue>> = HashMap::new();
        self.into_iter()
            .fold(enumerations, |mut acc, mut e| {
                let enum_name = mem::take(&mut e.name);
                acc.entry(enum_name).or_default().push(EnumValue {
                    name: e.value,
                    order: e.sort_order,
                });
                acc
            })
            .into_iter()
            .map(|e| Enum {
                name: e.0,
                values: e.1,
            })
            .collect()
    }
}
