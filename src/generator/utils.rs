use anyhow::{bail, Error};

use crate::database::{self, postgres, Kind, TableInfoProvider};

pub async fn setup_database(
    database: database::Kind,
    exclude_tables: Vec<String>,
    connection_string: String,
) -> Result<impl TableInfoProvider, Error> {
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
