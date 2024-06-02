use std::{collections::HashMap, mem};

use anyhow::{bail, Error};

use crate::database::{self, postgres, Kind, TableColumn};

pub async fn setup_database(
    database: database::Kind,
    exclude_tables: Vec<String>,
    connection_string: String,
) -> Result<Box<dyn database::TableInfo>, Error> {
    let db = match database {
        Kind::Postgres => {
            postgres::Builder::new()
                .exclude(exclude_tables)
                .connect(connection_string)
                .await?
        }
        _ => bail!("database is not yet supported"),
    };
    Ok(db)
}

pub fn to_table_map(columns: Vec<TableColumn>) -> HashMap<String, Vec<TableColumn>> {
    let mut tables: HashMap<String, Vec<TableColumn>> = HashMap::new();
    for mut column in columns {
        let table_name = mem::take(&mut column.table_name);
        tables
            .entry(table_name)
            .or_insert_with(Vec::new)
            .push(column)
    }
    tables
}
