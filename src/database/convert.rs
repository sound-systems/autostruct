/*!
The `convert` module provides various implementations to map raw schema types into types that provide non redundant and necessary context
a consuming module intended to generate Rust code requires
*/

use std::{backtrace, collections::HashMap, mem};

use super::{
    raw_schema::{self, EnumType, TableColumn},
    schema::{self, Attribute, Column, Enum, EnumValue},
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
            .map(|mut e| {
                // ensure enums are sorted
                e.1.sort_by(|a, b| a.order.total_cmp(&b.order));
                Enum {
                    name: e.0,
                    values: e.1,
                }
            })
            .collect()
    }
}

pub trait CompositeTypeConverter {
    fn to_composite_types(self) -> Vec<schema::CompositeType>;
}

impl CompositeTypeConverter for Vec<raw_schema::CompositeType> {
    fn to_composite_types(self) -> Vec<schema::CompositeType> {
        let composites: HashMap<String, Vec<Attribute>> = HashMap::new();
        self.into_iter()
            .fold(composites, |mut acc, mut composite| {
                let c_name = mem::take(&mut composite.name);
                acc.entry(c_name).or_default().push(Attribute {
                    name: composite.attribute_name,
                    data_type: composite.data_type,
                });
                acc
            })
            .into_iter()
            .map(|c| schema::CompositeType {
                name: c.0,
                attributes: c.1,
            })
            .collect()
    }
}
