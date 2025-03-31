use std::time::Duration;

use anyhow::{bail, Error};

use crate::database::{
    self,
    postgres::{self},
    InfoProvider, Kind,
};

pub async fn setup(
    connection_string: &str,
    exclude_tables: Vec<String>,
    timeout: Duration,
) -> Result<impl InfoProvider, Error> {
    let database: database::Kind = connection_string.try_into()?;
    let provider = match database {
        Kind::Postgres => {
            postgres::Builder::new()
                .exclude(exclude_tables)
                .timeout(timeout)
                .connect(connection_string)
                .await?
        }
        _ => bail!("database is not yet supported"),
    };
    Ok(provider)
}
