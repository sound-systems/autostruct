/*!
The `generator` module contains the code that that is used to generate the Rust models that map to the database
schema
*/
mod generate;
mod utils;

use std::path::Path;

use crate::database::TableInfoProvider;
use anyhow::{bail, Context, Error};
use cruet::Inflector;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

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
        target_dir,
        ..
    } = args;

    let provider = utils::setup(&connection_string, exclude_tables).await?;
    let tables = provider.get_table_info().await?;
    let output_dir = Path::new(&target_dir);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .await
            .context("failed to create directory that generated source code will be written to")?;
    }

    for table in tables {
        let code = generate::code_from(&table, &provider)
            .context("failed to generate Rust struct from table definition")?;
        let file_name = table.name.to_snake_case();
        let source_file = output_dir.join(format!("{file_name}.rs"));
        let mut file = File::create(source_file)
            .await
            .context("failed to create source code file")?;
        file.write_all(code.as_bytes())
            .await
            .context("failed to write generated source code to file")?;
    }

    bail!("autostruct isn't quite ready yet...");
}
