use std::{collections::HashMap, mem};

use crate::database::table_info_provider::ColumnInfo;

use super::{table_info_provider::Converter, TableInfo};

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

impl Into<ColumnInfo> for TableColumn {
    fn into(self) -> ColumnInfo {
        ColumnInfo {
            name: self.column_name,
            udt_name: self.udt_name,
            data_type: self.data_type,
            is_nullable: self.is_nullable,
            is_unique: self.is_unique,
            is_primary_key: self.is_primary_key,
            foreign_key_table: self.foreign_key_table,
            foreign_key_id: self.foreign_key_id,
            table_schema: self.table_schema,
        }
    }
}

impl Converter for Vec<TableColumn> {
    fn to_table_info(self) -> Vec<TableInfo> {
        let tables: HashMap<String, Vec<ColumnInfo>> = HashMap::new();
        self.into_iter()
            .fold(tables, |mut acc, mut column| {
                let table_name = std::mem::take(&mut column.table_name);
                acc.entry(table_name).or_default().push(column.into());
                acc
            })
            .into_iter()
            .map(|t| TableInfo {
                name: t.0,
                columns: t.1,
            })
            .collect()
    }
}
