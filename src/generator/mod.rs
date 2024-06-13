/*!
The `generator` module contains the code that that is used to generate the Rust models that map to the database
schema
*/
mod generate;
mod utils;

use crate::database::TableInfoProvider;
use anyhow::{bail, Error};

pub struct Arguments {
    pub target_dir: String,
    pub exclude_tables: Vec<String>,
    pub connection_string: String,
    pub singular_table_names: bool,
}

impl Arguments {
    pub fn from(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            ..Default::default()
        }
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            target_dir: "./autostructs".to_string(),
            exclude_tables: Default::default(),
            connection_string: Default::default(),
            singular_table_names: false,
        }
    }
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
        exclude_tables,
        connection_string,
        ..
    } = args;

    let provider = utils::setup(&connection_string, exclude_tables).await?;
    let tables = provider.get_table_info().await?;
    println!("table info retrieved from database: {:?}", tables);

    bail!("autostruct isn't quite ready yet...");
}
