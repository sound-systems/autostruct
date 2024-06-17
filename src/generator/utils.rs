use anyhow::{bail, Error};

use crate::database::{
    self,
    postgres::{self},
    Kind, InfoProvider,
};

pub async fn setup(
    connection_string: &str,
    exclude_tables: Vec<String>,
) -> Result<impl InfoProvider, Error> {
    let database: database::Kind = connection_string.try_into()?;
    let provider = match database {
        Kind::Postgres => {
            postgres::Builder::new()
                .exclude(exclude_tables)
                .connect(connection_string)
                .await?
        }
        _ => bail!("database is not yet supported"),
    };
    Ok(provider)
}
