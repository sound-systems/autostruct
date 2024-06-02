/*!
The `generator` module contains the code that that is used to generate the Rust models that map to the database
schema
*/
mod codegen;
mod utils;

use std::{collections::HashMap, mem};

use crate::database::{
    self,
    postgres::{self, Database},
    Kind, TableColumn,
};
use anyhow::{bail, Error};

pub struct Arguments {
    pub target_dir: String,
    pub database: database::Kind,
    pub exclude_tables: Vec<String>,
    pub connection_string: String,
    pub singular_table_names: bool,
}

/// Runs the generation process based on the provided CLI arguments.
///
/// # Arguments
///
/// * `args` - the arguments needed to run the generator
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation.
pub async fn run(args: Arguments) -> Result<(), Error> {
    let Arguments {
        database,
        exclude_tables,
        connection_string,
        ..
    } = args;

    let db = utils::setup_database(database, exclude_tables, connection_string).await?;
    let columns = db.get_table_columns().await?;
    let tables = utils::to_table_map(columns);
    for table in tables {}

    Ok(())
}
