use anyhow::{bail, Error};

use crate::database::{
    self,
    postgres::{self, PgTypeGetter},
    Kind, TableInfoProvider, TypeGetter,
};

pub async fn setup(
    database: database::Kind,
    exclude_tables: Vec<String>,
    connection_string: String,
) -> Result<(impl TableInfoProvider, impl TypeGetter), Error> {
    let (provider, typer) = match database {
        Kind::Postgres => {
            let db = postgres::Builder::new()
                .exclude(exclude_tables)
                .connect(connection_string)
                .await?;

            (db, PgTypeGetter::new())
        }
        _ => bail!("database is not yet supported"),
    };
    Ok((provider, typer))
}
