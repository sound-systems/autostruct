use crate::database::{self, postgres, Kind};
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
    let db: Box<dyn database::ColumnFetcher> = match args.database {
        Kind::Postgres => {
            postgres::Builder::new()
                .exclude(args.exclude_tables)
                .build(&args.connection_string)
                .await?
        }
        _ => bail!("database is not yet supported"),
    };

    let columns = db.get_table_columns().await?;
    for column in columns {
        println!("{:?}", column)
    }

    Ok(())
}
